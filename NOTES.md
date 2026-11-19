# 设计与迭代备忘

这份文档记录 zmatrix 的设计取舍、演变历史与待办清单。

**面向**：想了解「为什么是这样」的读者、准备参与改进的贡献者、从旧版本升级的人。

**第一次接触本库请先看 [README](README.MD)** —— 那里讲的是「现在能做什么」，这里讲的是「为什么这么做」和「还差什么」。

---

## 一、设计取舍与演变

### 1.1 时间类型：只保留 `chrono::TimeDelta`

早期版本同时支持 `std::time::Duration` 与 `chrono::TimeDelta`（双轨），`0.4.1` 移除了前者。

**理由**：`Duration` 是无符号的，无法表示负时长。而仿真中反向制动、时间倒推都是真实需求，
用 `Duration` 就得在调用点自己判断符号、自己带方向标记——每个调用点都要记一遍，
一定会有人漏掉。`TimeDelta` 原生支持负值，从类型层面消灭这个问题。

这条改动是破坏性的：所有接收 `Duration` 的运算全部消失。当时判断库里没有实际下游使用者，
所以走的是 patch 位（0.4.1）而非 minor。

### 1.2 叉乘为什么用专用 trait 而不是 `Mul`

最初的实现以 `T: Mul<B, Output = F>` 推导叉乘的输出类型。这条路走不通，因为**同一对类型
既可能相乘、又可能叉乘，而两者给出不同的物理量**：

```text
Distance * Force  →  Energy     普通乘法给出功 W = F·d
r × F             →  Torque     叉乘给出力矩 τ = r × F
```

Rust 不允许同一个 `Mul` 有两个 `Output`，用 `Mul` 承载叉乘，最常用的 `r × F` 就写不出来。

当时还有第二个冲突源：`MagneticMoment * MagneticInduction` 也曾实现为点乘（输出 `Energy`），
与磁力矩的叉乘争同一个 `Mul`。这条点乘后来在 `5a3b932` 中整体删除了 —— 理由是符号约定：
`U = -m·B`，而 `m * b` 只能给出 `+m·B`，与真实物理反号且不会报错；改为命名函数
`potential_energy` 把负号写进名字里。**所以这个冲突源现在已经不存在**，
`Distance` 那一条足以说明为什么叉乘不能走 `Mul`。

改为专用 trait 后两者并存：

```text
pub trait CrossProduct<B> {
    type Out: VectorQuantity + Default;
}
```

`Coef × B → B` 这条是泛化的，`a × (c·v) = c·(a × v)`，量纲随右侧。
它同时覆盖了早期单独存在的 `Coef × Coef → Coef`。

### 1.3 牵连速度为什么有两个入口

`AngularVelocity * Distance → Velocity`（标量乘法）与 `AngularVelocity × Distance → Velocity`
（叉乘）**同时存在**，不是重复：

```text
ω = 2 ẑ rad/s, r = (3, 0, 4) m      ← r 有沿轴分量
  叉乘 ω × r   → (0, 6, 0) m/s     轴向分量不贡献速度
  标量 ω * |r| → 10 m/s            把 4 m 轴向距离也算进去了
```

标量版适用于「已知半径求速率」的一维场景，保留它是因为这类调用很常见且意图明确。
向量场里算牵连速度必须用叉乘。

这一条曾经被误读成「0.4.9 缺少 `AngularVelocity×Distance`」——实际缺的是叉乘那一半，
标量那半一直在。

### 1.4 `PhysicalQuantity::raw_value` 与单位表示

`raw_value()` 返回内部存储值，不做单位换算：

```text
Mass::from_g(500.0)   raw_value = 500.0   default_unit_value = 0.5
Mass::from_kg(0.5)    raw_value = 0.5     default_unit_value = 0.5
```

它是为三机冗余按位表决准备的（`tmr` feature），默认构建下不存在。
存在的意义是：表决要判「两台冗余机的内部状态是否逐位相同」，而不是「是否描述同一个量」。

具体设计见 `src/tmr.rs` 的模块文档——**故意没有放进 README**：这是一个可选 feature，
面向全体的文档不适合展开它，否则会让人依赖没选择加入的语义。

### 1.5 判零阈值：`RwLock` vs `const`

| feature | 存储 | 运行时可否修改 | 定制方式 |
|---|---|---|---|
| 关（默认） | `static RwLock<f64>` | 是，`set_flt64_zero` | 运行时调用 setter |
| 开 | `const f64` | 否 | `ZMATRIX_FLT64_ZERO`，构建时生效（见 1.6） |

启用 `tmr` feature 后变成 `const`，setter 消失。

原因是三机冗余下运行时可变的全局容差是个共模故障源：三台若读到不同阈值，判零结论不同，
表决永久分歧；而这个变量能被任何代码（包括已被 SEU 破坏的代码）改写，**三台会同时被改**，
因为跑的是同一份逻辑——这正是冗余无法覆盖的失效模式。

没有选 `OnceLock` 的原因：它仍允许各机启动时配不同值，且**无法重置**——现有容差测试会
修改全局值再恢复，`OnceLock` 下只能设置一次，跑完会把整个测试套件的判零阈值永久改掉。

### 1.6 需要别的判零阈值：编译期定制，不是运行时

常量化的代价是调用方失去了 `set_flt64_zero`。但**不能因此退回运行时可变量**——那会把
共模故障重新放回来。所以定制发生在构建时：

```bash
ZMATRIX_FLT64_ZERO=1e-12 cargo build --features tmr
```

实现方式是 `build.rs` 读环境变量、校验、把值写成字面量存进 `OUT_DIR`，
由 `constant.rs` `include!` 进来。阈值仍是 `const`，仍可用于常量上下文，
运行时依旧没有任何路径能改它——只是「固化」的时机从写代码挪到了构建。

**为什么不用 `option_env!` 直接读**：`f64` 的字符串解析不是 `const fn`（到 1.85 仍不是），
没法在常量上下文里把字符串转成数。必须由构建脚本先解析成字面量。

**三机一致性靠什么保证**：三台用同一个值构建。如果一台是 `1e-12`、另一台是默认的
`1e-14`，判零结论就不同，表决会持续报分歧——这个问题无法在库内检测，只能靠构建流水线
统一。这也是为什么阈值不做成运行时参数：运行时能改的东西，三台就有机会改得不一致。

**非法值让构建失败，不退回默认**：非数字、`NaN`、无穷、非正数、以及下溢成 0 的值
（如 `1e-400`）都会 panic。阈值配错的表现是「判零结论悄悄改变」，从仿真结果极难反推，
所以宁可编译不过。下溢单独给一条诊断——用户明明输入了正数，报「必须是正数」会让人困惑。

**改环境变量会触发重编译**：`cargo:rerun-if-env-changed=ZMATRIX_FLT64_ZERO`。少了这一行，
改值会静默不生效，而产物看起来完全正常——这类问题极难定位。

**设了变量但没开 `tmr`**：给一条 `cargo:warning` 说明被忽略，不报错（可能先设变量后开
feature）。默认构建仍是 `RwLock` + setter，行为完全不变。

**测试如何在定制环境下存活**：`build.rs` 额外生成一个 `IS_DEFAULT: bool`（`#[doc(hidden)]`），
测试据此判断「该不该断言默认值」。否则一旦有人用定制阈值构建，`assert_eq!(FLT64_ZERO, 1e-14)`
就会挂，而这个失败跟他的改动毫无关系。

已知限制：阈值设成不合理的值（如 `0.5`）会让部分数值测试失败。这不是缺陷——`0.5` 意味着
任何小于 0.5 的量都算零，`to_quaternion` 之类的结果必然改变。合理性由使用方判断，
构建脚本只校验格式（正、有限、不下溢）。

---

## 二、尚未实现的账

按当前优先级排列。**已明确不做**的也列在最后，免得反复讨论。

### 线性代数

`dense::Matrix` 现有：`det` / `det_via_lu` / `solve` / `inverse` / `lu_decompose`，
全部只在 `f64` 上开放（量纲原因见 README「线性方程组与行列式」）。

`InertiaTensor` 现有：`solve_omega` / `solve_alpha`（Cholesky）/ `diagonalize`
（Jacobi）/ `parallel_axis` / `from_engineering_array` / `to_engineering_array`。

剩余缺口：

| 缺什么 | 影响 |
|---|---|
| 特征值求解（`N > 3`） | 挠性模态分析需要，目前只有 3×3 的 `diagonalize` |
| 稀疏矩阵 | 大 N 的稳态热网络会把稠密 LU 撑爆内存 |

**量纲约束依然成立**：`J⁻¹` 需要 `(kg·m²)⁻¹`、`det` 需要 `(kg·m²)³`，这两个量都不存在，
所以 `Matrix` 层只能是 `f64`，不能做成带量纲的通用接口。这也是 `Matrix` 保持
纯数值定位的原因。

### 热学量

| 待做 | 单位 | 说明 |
|---|---|---|
| `SpecificHeatCapacity` 比热容 | J/(kg·K) | 材料属性，`C = c·m` |
| `HeatTransferCoefficient` 传热系数 | W/(m²·K) | 表面对流，`q = h·ΔT`。真空环境很少用，优先级低 |
| `ThermalResistance` 热阻 | K/W | `R = 1/G`，串联分析更直观 |
| 辐射辅助函数 | — | `σT⁴` 在类型系统里没有落点，只能做成函数：`radiative_flux(ε, T)` |

**已明确不做**：

- `Entropy` 熵（J/K）—— 卫星热分析不用，等有明确需求
- `ThermalDiffusivity` 热扩散率（m²/s）—— `α = k/(ρc)` 可推，重复
- `Heat` 热量、`InternalEnergy` 内能 —— 与 `Energy` 重复
- `SpecificHeatRatio` 比热容比 γ —— 无量纲商，`Coef` 够用；且固体 `cp≈cv`、`γ≈1` 无信息量
- `Emissivity` 发射率 —— 先用 `Coef`，不引入值域约束（库里无先例）

### 流体量

| 待做 | 单位 | 说明 |
|---|---|---|
| `DynamicViscosity` 动力粘度 | Pa·s | 雷诺数 `Re = ρvL/μ` 需要 |
| `KinematicViscosity` 运动粘度 | m²/s | `ν = μ/ρ` |
| `SpecificGasConstant` 气体常数 | J/(kg·K) | 理想气体 `P = ρRT` 需要 |
| `SpeedOfSound` 声速 | m/s | 马赫数需要；`a = √(γRT)` 含 γ，属命名函数场景 |

### `compound` 模块

| 待做 | 说明 |
|---|---|
| `InertiaTensor` 由质量分布积分生成 | 从离散质点表 `Σ mᵢ(r²I − rrᵀ)` 直接构造 |
| `ThermalNetwork` 热网络容器 | 见下 |

**`ThermalNetwork` 计划范围**：

```text
add_node(capacity, temperature)     加节点
add_link(i, j, conductance)         加连接
set_source(i, power)                加热源
step(dt) -> Result<(), _>           显式推进一步
temperature(i) -> Option<Temperature>
```

**只做显式推进**：隐式推进与稳态求解都需要线性方程组求解器。
`Matrix::solve` 已可用，所以稳态求解的阻塞已解除，但隐式推进还需要
时间步进框架（Newton 迭代 + 收敛判据），不在当前范围。
`step()` 会检查显式稳定性判据 `Δt < 2·Cᵢ/ΣGᵢⱼ` 并返回错误 ——
不检查的话发散初期的曲线仍然「看起来合理」，等指数爆炸时已经在看错误结论了。

### 轨道力学

| 待做 | 说明 |
|---|---|
| `Vector3<Distance>` 与 `Vector3<Velocity>` 的点乘 | 算偏心率矢量需要 `r·v`；当前 `dot` 要求两侧同类型 |
| `Velocity × Velocity` | `\|v\|²` 需要；也可以为 `Velocity` 加标量平方入口 |
| 近地点/远地点半径、真近点角、平近点角 | 等有明确需求再补 |

### 运算符覆盖

- `CrossProduct` 现有 7 个组合（见 README 向量章节），新增一个只需两行 impl
- 物理量之间的乘除法仍不完整。补齐的判断标准是「这个逆运算有没有唯一且已建模的接收类型」：
  有就补，没有就不补——为对称性硬造类型比缺着更糟
- 单位系统仅覆盖工程常用子集，不是完整的 SI 词头体系

### 表决（`tmr` feature）

已覆盖：所有 `PhysicalQuantity`、`Vector3<T>`、`Quaternion`、`CosMatrix`、`EulerAngles`。

| 缺什么 | 说明 |
|---|---|
| `Matrix<R, C, f64>` 的表决 | 只有 `CosMatrix`（3×3 定长）支持。通用矩阵的元素数在编译期决定，需按 `R*C` 泛化 `TmrBits` 容量 |

`TmrBits` 容量是 `MAX_TMR_WORDS = 9`（够 `CosMatrix` 用满）。要支持更大的类型，
提高该常量即可；超容量时 `push` 会 panic 而不是静默截断——截断会让两个不同状态产生
相同键，那是唯一一种「表决错误地通过」的失效模式。
