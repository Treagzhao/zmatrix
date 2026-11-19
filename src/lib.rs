
pub mod dense;
pub mod physics;
pub mod constant;
pub mod spatial_geometry;
pub mod compound;
pub mod utils;

/// 三机冗余按位表决（TMR / MNV），由 `tmr` feature 控制。
#[cfg(feature = "tmr")]
pub mod tmr;

#[cfg(feature = "tmr")]
pub use constant::{get_flt64_zero, FLT64_ZERO};

#[cfg(not(feature = "tmr"))]
pub use constant::{get_flt64_zero, set_flt64_zero};

// `build.rs` 的纯逻辑部分，在这里引入一次以便单元测试。
//
// 构建脚本本身是独立编译单元，测试运行时不会执行它，覆盖率工具因此采不到
// 它的任何一行。把可测的纯函数抽到 `build_support.rs`，让 lib 在 `cfg(test)`
// 下也编译一份，那些逻辑才真正被测试到。
//
// 只在测试时引入：正常构建时这些函数已经由 build.rs 用过了，再进二进制是浪费。
#[cfg(test)]
#[path = "../build_support.rs"]
mod build_support;




