
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




