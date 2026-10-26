//! 复合物理量：由单一物理量组装而成的结构化对象。
//!
//! # 与 `physics::basic` 的分工
//!
//! [`physics::basic`](crate::physics::basic) 放**单一物理量** —— 每个类型都是
//! 一个 `f64` 值加一个单位标记，比如 `Distance`、`Inertia`、`Temperature`。
//!
//! 本模块放**由这些量按结构组织起来的对象**，它们不是单个值：
//!
//! | 对象 | 本质 | 元素类型 | 结构 |
//! |---|---|---|---|
//! | [`InertiaTensor`](inertia_tensor::InertiaTensor) | 3×3 张量 | `Inertia` | 密集、对称 |
//!
//! # 依赖方向
//!
//! ```text
//! compound  ──→  physics::basic       (元素类型)
//! compound  ──→  spatial_geometry     (坐标变换需要 CosMatrix)
//! ```
//!
//! 反向依赖**不允许**：`physics::basic` 不得引用本模块。这保证了
//! 「元素」与「容器」的分层不被打破。
//!
//! # 为什么放在这里而不是 `spatial_geometry`
//!
//! [`spatial_geometry`](crate::spatial_geometry) 里的三个类型
//! （`CosMatrix`、`Quaternion`、欧拉角）元素全是 `f64`，是纯几何、无量纲的量。
//! 而本模块的对象元素**带量纲**（kg·m² 等），两者定位不同，混在一处会模糊边界。

pub mod inertia_tensor;
