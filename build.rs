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
//!
//! # 这个文件只做胶水
//!
//! 解析、校验、生成源码都是纯函数，放在同一目录的 `build_support.rs`，
//! 由这里和 `src/lib.rs`（仅在 `cfg(test)` 下）各自 `#[path]` 引入。
//! 原因：构建脚本在 `cargo test` 的编译阶段就已执行完毕，测试运行时不再运行它，
//! 覆盖率工具采不到本文件的任何一行。把可测逻辑挪出去，覆盖率才有意义。
//!
//! 留在本文件的只有：读环境变量、往 stdout 打印 cargo 指令、写文件。

#[path = "build_support.rs"]
mod build_support;

use std::env;
use std::fs;
use std::path::PathBuf;

use build_support::{DEFAULT, ENV_KEY};

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
            build_support::parse_value(s).unwrap_or_else(|e| panic!("{e}"))
        }
        None => DEFAULT,
    };

    let customized = raw.is_some() && tmr_enabled;
    let code = build_support::render_const_source(value, customized)
        .unwrap_or_else(|e| panic!("{e}"));

    let path = out_dir().join("flt64_zero.rs");
    fs::write(&path, code)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
}

fn out_dir() -> PathBuf {
    PathBuf::from(
        env::var_os("OUT_DIR").expect("OUT_DIR is not set; this script must run under cargo"),
    )
}
