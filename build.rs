//! 构建脚本：把 `ZMATRIX_FLT64_ZERO` 环境变量固化成编译期常量。
//!
//! 启用 `tmr` feature 时，判零阈值 `FLT64_ZERO` 是 `const` 而不是运行时可变量
//! （理由见 `src/constant.rs` 的模块文档）。硬编码的常量无法定制，所以这里在
//! 构建时读环境变量，生成一个字面量写入 `OUT_DIR`，由 `constant.rs` `include!`。
//!
//! 为什么仍要编译期而不回到运行时：
//!
//! 三机冗余下运行时可变的阈值是共模故障源——三台若读到不同值，判零结论不同，
//! 表决永久分歧；而且同一份逻辑能同时改写三台。编译期固化后，同一个二进制
//! 里的阈值必然相同，这个失效模式不存在。定制只是把「固化」的时机从写代码
//! 移到构建时，没有把可变性放回运行时。
//!
//! ```bash
//! ZMATRIX_FLT64_ZERO=1e-12 cargo build --features tmr
//! ```

use std::env;
use std::fs;
use std::path::PathBuf;

/// 环境变量名。值必须能解析为 `f64`，且为正的有限数。
const ENV_KEY: &str = "ZMATRIX_FLT64_ZERO";

/// 未指定时的默认阈值。
const DEFAULT: f64 = 1e-14;

fn main() {
    // 没有这行，改了环境变量 cargo 不会重编译 build script，
    // 改动会静默不生效——这类问题极难定位，必须显式声明依赖。
    println!("cargo:rerun-if-env-changed={ENV_KEY}");

    let tmr_enabled = env::var_os("CARGO_FEATURE_TMR").is_some();
    let raw = env::var(ENV_KEY).ok();

    let value = match &raw {
        Some(s) => {
            if !tmr_enabled {
                // 不报错：一个项目可能同时有多个 target，或先设变量后开 feature。
                // 但要让人知道它此刻没起作用，否则会误以为配置已生效。
                println!(
                    "cargo:warning={ENV_KEY} is set but the `tmr` feature is not enabled; \
                     the value is ignored. Build with --features tmr for it to take effect."
                );
            }
            parse_value(s)
        }
        None => DEFAULT,
    };

    emit(out_dir(), value, raw.is_some() && tmr_enabled);
}

/// 解析并校验环境变量，非法值直接让构建失败。
///
/// 宁可编译不过，也不要静默退回默认值：阈值配错的表现是「判零结论悄悄改变」，
/// 而这类错误在仿真里极难从结果反推。
fn parse_value(raw: &str) -> f64 {
    let trimmed = raw.trim();

    let value: f64 = trimmed.parse().unwrap_or_else(|_| {
        panic!(
            "{ENV_KEY} = {raw:?} is not a valid f64.\n\
             Give a decimal literal or scientific notation, e.g. 1e-14 or 0.0000001."
        )
    });

    if value.is_nan() {
        panic!("{ENV_KEY} = {raw:?} is NaN. The zero threshold must be a number.");
    }
    if value.is_infinite() {
        panic!("{ENV_KEY} = {raw:?} is infinite. The zero threshold must be finite.");
    }
    if value == 0.0 && trimmed.chars().any(|c| ('1'..='9').contains(&c)) {
        // 下溢：输入本身是正数，但小于 f64 的最小次正规数，解析成了 0。
        // 报「必须是正数」会让用户困惑——他给的确实是正数。
        panic!(
            "{ENV_KEY} = {raw:?} underflows to 0 when parsed as f64.\n\
             The smallest positive f64 is about 5e-324; values below that round to zero. \
             Choose a larger threshold."
        );
    }
    if value <= 0.0 {
        // 0 会让 `norm > threshold` 对零向量成立，随后除以零；
        // 负数会让该条件恒真，任何量都被当作非零。
        panic!(
            "{ENV_KEY} = {raw:?} must be positive.\n\
             A threshold of 0 makes `norm > threshold` true for zero vectors and divides by zero; \
             a negative one makes it always true."
        );
    }

    value
}

fn out_dir() -> PathBuf {
    PathBuf::from(
        env::var_os("OUT_DIR").expect("OUT_DIR is not set; this script must run under cargo"),
    )
}

/// 生成常量字面量。
fn emit(out_dir: PathBuf, value: f64, customized: bool) {
    // `{:?}` 对 f64 给出能精确往返的最短表示（Rust 的 Debug impl 保证 round-trip）。
    let literal = format!("{value:?}");

    // 防呆：确认写进源码的字面量解析回来是同一个位型。
    // 万一格式化实现有变，宁可构建失败也不要把一个悄悄改过的阈值发出去。
    let round_trip: f64 = literal
        .parse()
        .unwrap_or_else(|_| panic!("generated literal {literal:?} does not parse back as f64"));
    assert_eq!(
        round_trip.to_bits(),
        value.to_bits(),
        "generated literal {literal:?} does not round-trip to the same bits"
    );

    let provenance = if customized {
        format!("set at build time via `{ENV_KEY}`")
    } else {
        format!("default; override with the `{ENV_KEY}` environment variable")
    };

    let code = format!(
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
    );

    let path = out_dir.join("flt64_zero.rs");
    fs::write(&path, code)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
}
