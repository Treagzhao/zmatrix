//! `build.rs` 的纯逻辑部分，抽出来是为了能被单元测试覆盖。
//!
//! `build.rs` 是独立编译单元，无法 `use` crate 内的代码；而这个文件被
//! `build.rs` 和 `src/lib.rs`（仅在 `cfg(test)` 下）各自 `#[path]` 引入一次，
//! 于是同一份逻辑既能驱动构建，又能被测试。
//!
//! 分成两边的原因：构建脚本在 `cargo test` 的**编译阶段**就已执行完毕，
//! 测试运行时它不会再跑，所以覆盖率工具采不到 `build.rs` 里的任何行。
//! 把可测的纯函数放在这里、只把环境变量读取和写文件留在 `build.rs`，
//! 覆盖率反映的才是真实被测的代码量，而不是一堆结构性无法执行的胶水。
//!
//! # 为什么返回 `Result` 而不是直接 `panic!`
//!
//! 构建脚本需要在出错时中断构建，`panic!` 是对的；但库代码 panic 不好测。
//! 这里返回 `Result<_, String>`，由 `build.rs` 决定 `panic!`，测试则可以直接
//! 断言错误内容。

/// 环境变量名。值必须能解析为 `f64`，且为正的有限数。
pub const ENV_KEY: &str = "ZMATRIX_FLT64_ZERO";

/// 未指定时的默认阈值。
pub const DEFAULT: f64 = 1e-14;

/// 解析并校验环境变量的值。
///
/// 宁可让构建失败，也不要静默退回默认值：阈值配错的表现是「判零结论悄悄改变」，
/// 而这类错误在仿真里极难从结果反推。
pub fn parse_value(raw: &str) -> Result<f64, String> {
    let trimmed = raw.trim();

    let value: f64 = trimmed.parse().map_err(|_| {
        format!(
            "{ENV_KEY} = {raw:?} is not a valid f64.\n\
             Give a decimal literal or scientific notation, e.g. 1e-14 or 0.0000001."
        )
    })?;

    if value.is_nan() {
        return Err(format!(
            "{ENV_KEY} = {raw:?} is NaN. The zero threshold must be a number."
        ));
    }
    if value.is_infinite() {
        return Err(format!(
            "{ENV_KEY} = {raw:?} is infinite. The zero threshold must be finite."
        ));
    }
    // 下溢：输入本身是正数，但小于 f64 的最小次正规数，解析成了 0。
    // 报「必须是正数」会让用户困惑——他给的确实是正数。
    //
    // 判据不能用「原文里有没有非零数字」：`0e5` 的指数带了 5，但那仍是明确的
    // 零。要看**尾数**是否全零——`0e5` 尾数是 "0"（明确的零），
    // `1e-400` 尾数是 "1"（真下溢）。
    if value == 0.0 && !mantissa_is_zero(raw) {
        return Err(format!(
            "{ENV_KEY} = {raw:?} underflows to 0 when parsed as f64.\n\
             The smallest positive f64 is about 5e-324; values below that round to zero. \
             Choose a larger threshold."
        ));
    }
    if value <= 0.0 {
        // 0 会让 `norm > threshold` 对零向量成立，随后除以零；
        // 负数会让该条件恒真，任何量都被当作非零。
        return Err(format!(
            "{ENV_KEY} = {raw:?} must be positive.\n\
             A threshold of 0 makes `norm > threshold` true for zero vectors and divides by zero; \
             a negative one makes it always true."
        ));
    }

    Ok(value)
}

/// 判断字符串的**尾数**部分是否全为零，与指数无关。
///
/// 用于区分「用户明确写了零」和「数值太小下溢成零」：
///
/// ```text
/// "0"       → true    明确是零
/// "0.0"     → true    明确是零
/// "0e5"     → true    尾数 0，指数 5 不改变它是零
/// "1e-400"  → false   尾数 1，真下溢
/// ```
///
/// 只看尾数而不看整串，是因为 `0e5` 里那个 `5` 是非零数字，但它属于指数，
/// 不代表这个数本身非零。
fn mantissa_is_zero(raw: &str) -> bool {
    let s = raw.trim().trim_start_matches(['+', '-']);
    let mantissa = s.split(['e', 'E']).next().unwrap_or(s);
    mantissa.chars().all(|c| c == '0' || c == '.')
}

/// 生成要写进 `OUT_DIR` 的常量源码。
///
/// `customized` 决定文档里写「来自环境变量」还是「默认值」，并决定
/// `IS_DEFAULT` 的取值——测试靠它判断该不该断言默认阈值。
pub fn render_const_source(value: f64, customized: bool) -> Result<String, String> {
    let literal = format_literal(value)?;

    let provenance = if customized {
        format!("set at build time via `{ENV_KEY}`")
    } else {
        format!("default; override with the `{ENV_KEY}` environment variable")
    };

    Ok(format!(
        "/// Threshold below which a quantity counts as floating-point zero.\n\
         ///\n\
         /// Absolute values smaller than this are treated as zero when testing for\n\
         /// zero and when normalising. Currently `{literal}` ({provenance}).\n\
         ///\n\
         /// Fixed at compile time: it cannot change at run time, which is what keeps\n\
         /// three redundant units from disagreeing. Set `{ENV_KEY}` when building to\n\
         /// choose a different value.\n\
         pub const FLT64_ZERO: f64 = {literal};\n\
         \n\
         /// Whether [`FLT64_ZERO`] is the built-in default rather than a value supplied\n\
         /// through `{ENV_KEY}`.\n\
         ///\n\
         /// Exists so tests can assert the default without breaking when the crate is\n\
         /// built with a custom threshold. Not part of the public API; hidden from docs.\n\
         #[doc(hidden)]\n\
         pub const IS_DEFAULT: bool = {is_default};\n",
        is_default = !customized
    ))
}

/// 把 `f64` 转成可写进源码的字面量，并确认它能精确往返。
///
/// `{:?}` 对 `f64` 给出能精确往返的最短表示（Rust 的 `Debug` impl 保证
/// round-trip）。这里解析回来比对位型，万一格式化实现有变就报错，
/// 而不是把一个悄悄改了值的阈值发出去。
pub fn format_literal(value: f64) -> Result<String, String> {
    let literal = format!("{value:?}");
    verify_literal(&literal, value)?;
    Ok(literal)
}

/// 检查一个字面量能否解析回原本的位型。
///
/// 单独成函数是为了可测：`format!("{:?}")` 的产物在当前 Rust 版本下总能
/// 往返成功，所以从 `format_literal` 内部触发的错误分支永远走不到。
/// 拆开后测试可以直接喂一个坏字面量，把两条错误路径真正覆盖掉。
fn verify_literal(literal: &str, value: f64) -> Result<(), String> {
    let round_trip: f64 = literal
        .parse()
        .map_err(|_| format!("generated literal {literal:?} does not parse back as f64"))?;

    if round_trip.to_bits() != value.to_bits() {
        return Err(format!(
            "generated literal {literal:?} does not round-trip to the same bits \
             (expected {:?}, got {:?})",
            value, round_trip
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== 合法输入 =====

    #[test]
    fn test_accepts_typical_thresholds() {
        for (input, expected) in [
            ("1e-14", 1e-14),
            ("1e-12", 1e-12),
            ("0.0000001", 1e-7),
            ("3.7e-11", 3.7e-11),
            ("0.5", 0.5),
            ("1", 1.0),
        ] {
            assert_eq!(
                parse_value(input).unwrap(),
                expected,
                "input {input:?} should be accepted"
            );
        }
    }

    #[test]
    fn test_trims_whitespace() {
        assert_eq!(parse_value("  1e-12  ").unwrap(), 1e-12);
        assert_eq!(parse_value("\t1e-12\n").unwrap(), 1e-12);
    }

    // ===== 非法输入，逐条断言诊断内容 =====

    // 不是数字
    #[test]
    fn test_rejects_non_numeric() {
        let err = parse_value("abc").unwrap_err();
        assert!(err.contains("is not a valid f64"), "got: {err}");
        assert!(err.contains("abc"), "错误信息应回显输入值");
    }

    #[test]
    fn test_rejects_empty() {
        assert!(parse_value("").unwrap_err().contains("is not a valid f64"));
        assert!(parse_value("   ").unwrap_err().contains("is not a valid f64"));
    }

    // NaN
    #[test]
    fn test_rejects_nan() {
        let err = parse_value("NaN").unwrap_err();
        assert!(err.contains("is NaN"), "got: {err}");
    }

    // 无穷
    #[test]
    fn test_rejects_infinity() {
        for input in ["inf", "-inf", "infinity"] {
            let err = parse_value(input).unwrap_err();
            assert!(
                err.contains("is infinite") || err.contains("must be positive"),
                "input {input:?} got: {err}"
            );
        }
    }

    // 零与负数走「必须为正」分支
    #[test]
    fn test_rejects_zero_and_negative() {
        for input in ["0", "0.0", "-0", "0e5", "-1e-12", "-1", "-0.5"] {
            let err = parse_value(input).unwrap_err();
            assert!(
                err.contains("must be positive"),
                "input {input:?} got: {err}"
            );
        }
    }

    // ★ 下溢：输入是正数但小于 f64 最小次正规数，要给专门的诊断
    #[test]
    fn test_rejects_underflow_with_specific_message() {
        for input in ["1e-400", "1e-324", "-1e-400"] {
            let err = parse_value(input).unwrap_err();
            assert!(
                err.contains("underflows to 0"),
                "input {input:?} 应报下溢而不是「必须为正」，got: {err}"
            );
        }
    }

    // 边界：最小正数应被接受，再小就下溢
    #[test]
    fn test_boundary_smallest_positive() {
        // f64::MIN_POSITIVE 是最小正*正规*数，次正规数更小但 parse 支持
        let tiny = "5e-324"; // 最小正次正规数
        assert!(parse_value(tiny).is_ok(), "5e-324 应被接受");

        // 比它更小则下溢成 0
        assert!(parse_value("1e-400").unwrap_err().contains("underflows"));
    }

    // ===== 字面量格式化 =====

    #[test]
    fn test_literal_round_trips_exactly() {
        for v in [
            1e-14,
            1e-12,
            0.5,
            1.0,
            1e-300,
            f64::MIN_POSITIVE,
            f64::MAX,
            3.7e-11,
            1.0 / 3.0,
        ] {
            let lit = format_literal(v).unwrap();
            let back: f64 = lit.parse().unwrap();
            assert_eq!(
                back.to_bits(),
                v.to_bits(),
                "value {v:?} -> literal {lit:?} 位型不一致"
            );
        }
    }

    #[test]
    fn test_literal_uses_shortest_form() {
        assert_eq!(format_literal(1e-14).unwrap(), "1e-14");
        assert_eq!(format_literal(0.5).unwrap(), "0.5");
    }

    // ★ 两条错误路径：正常路径下 `{:?}` 总能往返成功，所以从
    // `format_literal` 内部永远触发不到。直接喂坏字面量才能覆盖。
    #[test]
    fn test_verify_rejects_unparsable() {
        let err = verify_literal("not-a-number", 1.0).unwrap_err();
        assert!(err.contains("does not parse back as f64"), "got: {err}");
    }

    #[test]
    fn test_verify_rejects_wrong_value() {
        // 合法 f64，但不是 value 本身 —— 位型不同
        let err = verify_literal("0.5", 1.0).unwrap_err();
        assert!(err.contains("does not round-trip"), "got: {err}");
        assert!(err.contains("0.5"), "应回显坏字面量");
    }

    #[test]
    fn test_verify_accepts_exact_round_trip() {
        for v in [1e-14, 0.5, 3.7e-11, f64::MAX] {
            let lit = format!("{v:?}");
            assert!(verify_literal(&lit, v).is_ok(), "{v:?} 应往返成功");
        }
    }

    // ===== 生成的源码 =====

    #[test]
    fn test_render_contains_value_and_doc() {
        let src = render_const_source(1e-12, true).unwrap();
        assert!(src.contains("pub const FLT64_ZERO: f64 = 1e-12;"), "got: {src}");
        assert!(src.contains("ZMATRIX_FLT64_ZERO"), "文档应提到环境变量名");
        assert!(src.contains("set at build time"), "应标出来自环境变量");
    }

    // IS_DEFAULT 必须与 customized 相反——测试靠它决定要不要断言默认值，
    // 弄反了会让定制构建下的测试误判。
    #[test]
    fn test_render_is_default_flag() {
        let custom = render_const_source(1e-12, true).unwrap();
        assert!(
            custom.contains("pub const IS_DEFAULT: bool = false;"),
            "customized=true 时 IS_DEFAULT 应为 false，got: {custom}"
        );

        let default_ = render_const_source(DEFAULT, false).unwrap();
        assert!(
            default_.contains("pub const IS_DEFAULT: bool = true;"),
            "customized=false 时 IS_DEFAULT 应为 true"
        );
    }

    #[test]
    fn test_render_default_documents_default_origin() {
        let src = render_const_source(DEFAULT, false).unwrap();
        assert!(src.contains("1e-14"));
        assert!(src.contains("default;"), "应标出来自默认值");
    }

    // 生成的源码必须能被当作 Rust 解析——否则构建会写出一个编译不过的文件。
    // 这里只能做文本层面的粗检（真要编译得靠 lib 自己，而 lib 已经在 include 它）。
    #[test]
    fn test_render_is_syntactically_plausible() {
        let src = render_const_source(1e-12, true).unwrap();
        assert!(src.contains("pub const FLT64_ZERO: f64 ="));
        assert!(src.contains("pub const IS_DEFAULT: bool ="));
        assert!(src.ends_with('\n'), "生成的文件应以换行结尾");
        // 花括号必须配平，否则 include! 进来会报错
        let opens = src.matches('{').count();
        let closes = src.matches('}').count();
        assert_eq!(opens, closes, "花括号必须配平");
    }
}
