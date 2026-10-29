pub mod element;
pub mod error;
mod initial;
pub mod operation;
mod shape;
mod util;

pub use element::NumericElement;

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};

/// 编译期定尺寸的稠密矩阵。
///
/// 元素类型 `T` 由 [`NumericElement`] 限定为基础数字量（整数/浮点），
/// 物理量不能作为矩阵元素 —— 原因见 [`element`] 模块文档。
///
/// # 尺寸必须是正数
///
/// `ROWS == 0` 或 `COLS == 0` 的矩阵在构造时即编译失败：
///
/// ```compile_fail
/// use zmatrix::dense::Matrix;
///
/// // 0 行的矩阵没有意义，编译期即被拒绝
/// let m = Matrix::<0, 3, f64>::new([[]; 0]);
/// ```
///
/// ```compile_fail
/// use zmatrix::dense::Matrix;
///
/// let m = Matrix::<3, 0, f64>::new([[], [], []]);
/// ```
///
/// 这是计算库的取舍而非数学立场：`2×0` 这样的空矩阵在代数上良定义，
/// 但在实际计算中没有用途，而允许它存在会让 `det`、`product`、`sum`
/// 等操作各自承担退化尺寸的分支与隐患。
#[derive(Clone, Copy)]
pub struct Matrix<const ROWS: usize, const COLS: usize, T>
where
    T: NumericElement,
{
    data: [[T;COLS];ROWS],
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: NumericElement + Display,
{
    pub fn new(data:[[T;COLS];ROWS]) -> Self {
        // ROWS / COLS 在编译期已知，此断言在 monomorphization 阶段求值：
        // 零尺寸矩阵通不过编译，运行时零开销。
        const { assert!(ROWS > 0 && COLS > 0, "Matrix 的行数和列数都必须大于 0"); }
        Matrix { data }
    }
}

impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: NumericElement + Display,
{
    fn display(&self) -> String {
        let mut digits: u8 = 0;
        for value in self.data.iter() {
            let list = *value;
            for f in list.iter() {
                let d = f.to_string().len() as u8;
                if d > digits {
                    digits = d
                }
            }
        }
        if ROWS == 1 {
            let line = util::print_single_line(&self.data, digits);
            return line;
        }
        let mut result = String::new();
        for row in 0..ROWS {
            let mut line = String::new();
            let (start_char, end_char) = util::get_boundary_char(row, ROWS);
            line.push_str(&start_char);
            let start = row * COLS;
            let end = (row + 1) * COLS;
            for i in start..end {
                let value = self.data[row][i - start];
                let str = format!("{:<digit$}", value, digit = (digits + 2) as usize);
                line.push_str(&str);
            }
            line.push_str(&end_char);
            line.push('\n');
            result.push_str(&line);
        }
        result
    }
}


impl<const ROWS: usize, const COLS: usize, T> Matrix<ROWS, COLS, T>
where
    T: NumericElement,
{
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= COLS || y >= ROWS {
            return None;
        }
        Some(self.data[y][x])
    }

    pub fn height(&self) -> usize {
        ROWS
    }

    pub fn width(&self) -> usize {
        COLS
    }

    pub fn size(&self) -> (usize, usize) {
        (ROWS, COLS)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), error::OperationError> {
        if x >= COLS || y >= ROWS {
            return Err(error::OperationError {
                message: "index out of bounds".to_string(),
            });
        }
        self.data[y][x] = value;
        Ok(())
    }
}

impl<const N: usize, T> Matrix<N, N, T>
where
    T: NumericElement + Display + Default + Add<Output = T> + From<i8> + Mul<Output = T>,
{
    pub fn det(&self) -> Result<T, error::OperationError> {
        let mut sum = T::default();
        let permutation = util::permutation(N)?;
        for perm in permutation {
            let res = util::determinant_in_one_permutation(&self.data, &perm);
            match res {
                Ok(x) => sum = sum + x,
                Err(e) => return Err(e),
            }
        }
        Ok(sum)
    }

}

/// 浮点方阵的线性方程组求解。
///
/// # 为什么只为 `f64` 实现
///
/// 求解需要除法。整数除法会静默截断（`5/2 → 2`），在 `Matrix<i32>` 上跑
/// 高斯消元会得到看似正常的错误结果。`NumericElement` 无法区分
/// 「有真实除法的类型」与「整除截断的类型」，所以这里只在 `f64` 上开放。
///
/// 这也是 `Matrix` 纯粹的数值定位：**不做带量纲版本**。
/// 惯量张量的 `J⁻¹·L` 请用
/// [`InertiaTensor::solve_omega`](crate::compound::inertia_tensor::InertiaTensor::solve_omega)，
/// 那里能提供量纲检查与正定校验。
impl<const N: usize> Matrix<N, N, f64> {
    /// 解 `A · x = b`，返回 `x`。
    ///
    /// # 算法：带部分主元的 LU 分解
    ///
    /// 相比 Cramer / 排列枚举（现有 [`det`](Self::det) 用的方法，复杂度 `O(N!)`），
    /// LU 是 `O(N³)`，几十阶的方程组才具备实用性。
    ///
    /// **部分主元（行交换）是必需的**：不选主元时，主元接近零会放大舍入误差，
    /// 即使矩阵非奇异也会给出严重失真的解。选主元把误差控制在可接受范围。
    ///
    /// # 奇异判定
    ///
    /// 主元绝对值小于 `1e-12 × 矩阵规模` 视为奇异，返回
    /// [`OperationError`](crate::dense::error::OperationError)。
    /// 用绝对阈值而非精确零，是因为浮点运算下「数学上非奇异但数值上奇异」
    /// 的矩阵很常见（例如两行几乎线性相关）。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::dense::Matrix;
    ///
    /// // 2x + y  = 5
    /// //  x + 3y = 10
    /// let a = Matrix::<2, 2, f64>::new([[2.0, 1.0], [1.0, 3.0]]);
    /// let x = a.solve([5.0, 10.0]).unwrap();
    /// assert!((x[0] - 1.0).abs() < 1e-12);
    /// assert!((x[1] - 3.0).abs() < 1e-12);
    /// ```
    pub fn solve(&self, b: [f64; N]) -> Result<[f64; N], error::OperationError> {
        let (lu, perm) = self.lu_decompose()?;
        Ok(Self::lu_solve(&lu.data, &perm, b))
    }

    /// 带部分主元的 LU 分解，返回 `(LU 合并矩阵, 行交换记录)`。
    ///
    /// `LU` 的下三角（不含对角）存 `L` 的乘数，上三角含对角存 `U`。
    /// `perm[i]` 表示第 `i` 行来自原始的第 `perm[i]` 行。
    ///
    /// 供 [`solve`](Self::solve) 与 [`det_via_lu`](Self::det_via_lu) 复用——
    /// 需要同时求行列式和解方程时，分解只做一次。
    pub fn lu_decompose(&self) -> Result<(Self, [usize; N]), error::OperationError> {
        let mut a = self.data;
        let mut perm = [0usize; N];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = i;
        }

        // 阈值：随规模放大，避免大矩阵上过严
        let tol = 1e-12 * (N as f64);

        for k in 0..N {
            // --- 选主元：第 k 列中绝对值最大的行 ---
            let mut pivot = k;
            let mut max_val = a[k][k].abs();
            for i in (k + 1)..N {
                if a[i][k].abs() > max_val {
                    max_val = a[i][k].abs();
                    pivot = i;
                }
            }

            if max_val <= tol {
                return Err(error::OperationError::new(
                    "solve: matrix is singular (or numerically indistinguishable from singular)",
                ));
            }

            // 交换行
            if pivot != k {
                a.swap(k, pivot);
                perm.swap(k, pivot);
            }

            // --- 消元 ---
            let pivot_val = a[k][k];
            for i in (k + 1)..N {
                let factor = a[i][k] / pivot_val;
                a[i][k] = factor; // 存 L 的乘数
                for j in (k + 1)..N {
                    a[i][j] -= factor * a[k][j];
                }
            }
        }

        Ok((Matrix { data: a }, perm))
    }

    /// 用已分解的 LU 解方程（内部用，不选主元）。
    ///
    /// `lu_decompose` 已保证每个主元非零，所以这里无需再检查——
    /// 用 `debug_assert` 兜底而不是运行时分支，避免留下不可达的错误路径。
    fn lu_solve(lu: &[[f64; N]; N], perm: &[usize; N], b: [f64; N]) -> [f64; N] {
        // 按行交换记录重排右端项
        let mut y = [0.0_f64; N];
        for i in 0..N {
            y[i] = b[perm[i]];
        }

        // 前代：L·y = Pb（L 对角为 1）
        for i in 0..N {
            for k in 0..i {
                y[i] -= lu[i][k] * y[k];
            }
        }

        // 回代：U·x = y
        let mut x = [0.0_f64; N];
        for i in (0..N).rev() {
            let mut sum = y[i];
            for k in (i + 1)..N {
                sum -= lu[i][k] * x[k];
            }
            debug_assert!(
                lu[i][i] != 0.0,
                "lu_decompose guarantees non-zero pivots"
            );
            x[i] = sum / lu[i][i];
        }

        x
    }

    /// 用 LU 分解求行列式，复杂度 `O(N³)`。
    ///
    /// 与 [`det`](Self::det) 数值等价，但 `det` 走排列枚举（`O(N!)`），
    /// 仅在 `N` 很小时可用。`N ≥ 6` 起本方法明显更快。
    ///
    /// 符号修正：每次行交换翻转一次符号，所以 `det = (-1)^交换次数 × Π Uᵢᵢ`。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::dense::Matrix;
    ///
    /// let m = Matrix::<3, 3, f64>::new([
    ///     [1.0, 2.0, 3.0],
    ///     [4.0, 5.0, 6.0],
    ///     [7.0, 8.0, 10.0],
    /// ]);
    /// // 与排列枚举版本一致
    /// assert!((m.det_via_lu().unwrap() - (-3.0)).abs() < 1e-12);
    /// ```
    pub fn det_via_lu(&self) -> Result<f64, error::OperationError> {
        let (lu, perm) = self.lu_decompose()?;

        let mut det = 1.0;
        for i in 0..N {
            det *= lu.data[i][i];
        }

        // 统计置换的奇偶性：perm[i] != i 的成环数决定符号
        let mut visited = [false; N];
        let mut swaps = 0usize;
        for i in 0..N {
            if visited[i] {
                continue;
            }
            let mut j = i;
            let mut cycle_len = 0usize;
            while !visited[j] {
                visited[j] = true;
                j = perm[j];
                cycle_len += 1;
            }
            if cycle_len > 1 {
                swaps += cycle_len - 1;
            }
        }
        if swaps % 2 == 1 {
            det = -det;
        }

        Ok(det)
    }

    /// 显式求逆。
    ///
    /// # 为什么不推荐
    ///
    /// 数值上 `J⁻¹·b` 不如直接 [`solve`](Self::solve) 稳定：先求逆再相乘
    /// 引入了额外的舍入误差，且多一次矩阵乘法。**绝大多数场景解一次方程就够。**
    ///
    /// 保留此方法是因为有些算法确实需要 `A⁻¹` 本身（例如要多次乘不同右端项，
    /// 或需要显式的协方差矩阵）。内部做法是对单位阵逐列 [`solve`](Self::solve)。
    ///
    /// # 示例
    /// ```
    /// use zmatrix::dense::Matrix;
    ///
    /// let m = Matrix::<2, 2, f64>::new([[2.0, 0.0], [0.0, 4.0]]);
    /// let inv = m.inverse().unwrap();
    /// assert!((inv.get(0, 0).unwrap() - 0.5).abs() < 1e-12);
    /// assert!((inv.get(1, 1).unwrap() - 0.25).abs() < 1e-12);
    /// ```
    pub fn inverse(&self) -> Result<Self, error::OperationError> {
        let mut cols = [[0.0_f64; N]; N];
        for c in 0..N {
            let mut e = [0.0_f64; N];
            e[c] = 1.0;
            let col = self.solve(e)?;
            for r in 0..N {
                cols[r][c] = col[r];
            }
        }
        Ok(Matrix { data: cols })
    }
}

impl<const ROWS: usize, const COLS: usize, T> Debug for Matrix<ROWS, COLS, T>
where
    T: NumericElement + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}impl<const ROWS: usize, const COLS: usize, T> Display for Matrix<ROWS, COLS, T>
where
    T: NumericElement + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_ok() {
        let m = Matrix::<3, 2, i32>::new([[1, 2], [3, 4], [5, 6]]);
        assert_eq!(m.height(), 3);
        assert_eq!(m.width(), 2);
        assert_eq!(m.data, [[1, 2], [3, 4], [5, 6]]);
    }

    // 零尺寸矩阵的拒绝由编译期常量断言保证，其回归测试写在
    // `Matrix` 结构体的文档注释里（compile_fail doctest）：
    // `Matrix::<0, 3, _>` 与 `Matrix::<3, 0, _>` 都无法通过编译。
    //
    // 这里只验证「最小合法尺寸」仍然可用，作为边界对照。
    #[test]
    fn smallest_valid_size() {
        let one = Matrix::<1, 1, i32>::new([[7]]);
        assert_eq!(one.size(), (1, 1));
        assert_eq!(one.get(0, 0), Some(7));

        let row = Matrix::<1, 5, i32>::new([[1, 2, 3, 4, 5]]);
        assert_eq!(row.size(), (1, 5));

        let col = Matrix::<5, 1, i32>::new([[1], [2], [3], [4], [5]]);
        assert_eq!(col.size(), (5, 1));
    }

    #[test]
    fn test_print_single_line_matrix() {
        let m = Matrix::<1, 3, i32>::new([[1, 2, 3]]);
        assert_eq!(format!("{}", m), "[1  2  3  ]\n");
    }

    #[test]
    fn test_print_multi_line_matrix() {
        let vec = [[1, 2, 3], [4, 5, 6], [7, 8, 90]];
        let m = Matrix::<3,3,i32>::new(vec);
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!("┌1   2   3   ┐\n|4   5   6   |\n└7   8   90  ┘\n", result);
        let result = format!("{:?}", m);
        println!("{}", result.clone());
        assert_eq!("┌1   2   3   ┐\n|4   5   6   |\n└7   8   90  ┘\n", result);

        let vec = [
            [1.12,
            2.231,
            3.1],
            [4.123123,
            5.123123,
            6.43422342323],
            [7.121,
            8.1,
            90.0],
        ];
        let m = Matrix::<3,3,f64>::new( vec);
        let result = format!("{}", m);
        println!("{}", result.clone());
        assert_eq!(
            r#"┌1.12           2.231          3.1            ┐
|4.123123       5.123123       6.43422342323  |
└7.121          8.1            90             ┘
"#,
            result
        );
    }

    #[test]
    fn test_get() {
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert_eq!(m.get(0, 1), Some(3));
        assert_eq!(m.get(2, 1), None);
    }

    #[test]
    fn test_clone() {
        let m1 = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        let m2 = m1.clone();
        assert_eq!(m1.data, m2.data);
    }

    #[test]
    fn test_size() {
        let m = Matrix::<3, 4, i32>::new([[0;4];3]);
        assert_eq!(m.size(), (3, 4));
    }

    #[test]
    fn test_set() {
        let mut m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert!(m.set(1, 1, 5).is_ok());
        assert_eq!(m.data[1][1], 5);
    }

    #[test]
    fn test_set_out_of_bounds() {
        let mut m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert!(m.set(2, 1, 5).is_err());
    }

    #[test]
    fn test_det_2x2() {
        let m = Matrix::<2, 2, i32>::new([[1, 2], [3, 4]]);
        assert_eq!(m.det().unwrap(), -2);
    }

    #[test]
    fn test_det_3x3() {
        let m = Matrix::<3, 3, i32>::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(m.det().unwrap(), 0);
    }

    // ===== solve / lu_decompose / det_via_lu / inverse =====

    fn assert_close(a: f64, b: f64, eps: f64) {
        assert!((a - b).abs() < eps, "left = {a}, right = {b}");
    }

    #[test]
    fn test_solve_2x2() {
        // 2x + y = 5, x + 3y = 10  →  x = 1, y = 3
        let a = Matrix::<2, 2, f64>::new([[2.0, 1.0], [1.0, 3.0]]);
        let x = a.solve([5.0, 10.0]).unwrap();
        assert_close(x[0], 1.0, 1e-12);
        assert_close(x[1], 3.0, 1e-12);
    }

    #[test]
    fn test_solve_3x3_diagonal() {
        let a = Matrix::<3, 3, f64>::new([
            [2.0, 0.0, 0.0],
            [0.0, 4.0, 0.0],
            [0.0, 0.0, 8.0],
        ]);
        let x = a.solve([6.0, 8.0, 16.0]).unwrap();
        assert_close(x[0], 3.0, 1e-12);
        assert_close(x[1], 2.0, 1e-12);
        assert_close(x[2], 2.0, 1e-12);
    }

    #[test]
    fn test_solve_residual_is_small() {
        // 解的正确性判据：A·x 必须还原 b
        let a = Matrix::<3, 3, f64>::new([
            [120.0, 8.0, 0.0],
            [8.0, 130.0, 0.0],
            [0.0, 0.0, 45.0],
        ]);
        let b = [60.0, 0.0, 90.0];
        let x = a.solve(b).unwrap();

        // 手算 A·x
        let mut ax = [0.0_f64; 3];
        for i in 0..3 {
            for j in 0..3 {
                ax[i] += a.data[i][j] * x[j];
            }
        }
        for i in 0..3 {
            assert_close(ax[i], b[i], 1e-9);
        }
    }

    #[test]
    fn test_solve_requires_pivoting() {
        // 首元为 0：不选主元会直接除零，选主元后正常
        let a = Matrix::<2, 2, f64>::new([[0.0, 1.0], [1.0, 0.0]]);
        let x = a.solve([3.0, 5.0]).unwrap();
        assert_close(x[0], 5.0, 1e-12); // y = 5
        assert_close(x[1], 3.0, 1e-12); // x = 3
    }

    #[test]
    fn test_solve_pivoting_improves_accuracy() {
        // 经典病态例子：主元很小时不选主元会严重失真
        let a = Matrix::<2, 2, f64>::new([[1e-18, 1.0], [1.0, 1.0]]);
        let b = [1.0, 2.0];
        let x = a.solve(b).unwrap();

        let mut ax = [0.0_f64; 2];
        for i in 0..2 {
            for j in 0..2 {
                ax[i] += a.data[i][j] * x[j];
            }
        }
        assert_close(ax[0], b[0], 1e-9);
        assert_close(ax[1], b[1], 1e-9);
    }

    #[test]
    fn test_solve_singular_is_error() {
        // 第二行是第一行的倍数
        let a = Matrix::<2, 2, f64>::new([[1.0, 2.0], [2.0, 4.0]]);
        assert!(a.solve([1.0, 2.0]).is_err());

        // 全零
        let z = Matrix::<3, 3, f64>::new([[0.0; 3]; 3]);
        assert!(z.solve([1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn test_det_via_lu_matches_det() {
        // 与排列枚举版本对标（N 小时两者都能用）
        let m = Matrix::<2, 2, f64>::new([[1.0, 2.0], [3.0, 4.0]]);
        assert_close(m.det_via_lu().unwrap(), -2.0, 1e-12);

        let m = Matrix::<3, 3, f64>::new([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 10.0],
        ]);
        assert_close(m.det_via_lu().unwrap(), -3.0, 1e-12);

        // 与 det() 逐值比对（同一矩阵）
        let m2 = Matrix::<3, 3, f64>::new([
            [4.0, -2.0, 1.0],
            [3.0, 6.0, -4.0],
            [2.0, 1.0, 8.0],
        ]);
        assert_close(m2.det_via_lu().unwrap(), m2.det().unwrap(), 1e-9);
        // 手算：4*(6*8+4) + 2*(3*8+4*2) + (3*1-6*2) = 208 + 64 - 9 = 263
        assert_close(m2.det_via_lu().unwrap(), 263.0, 1e-9);
    }

    #[test]
    fn test_det_via_lu_sign_after_swap() {
        // 需要行交换的情形，符号必须正确
        let m = Matrix::<2, 2, f64>::new([[0.0, 1.0], [1.0, 0.0]]);
        // det = 0*0 - 1*1 = -1
        assert_close(m.det_via_lu().unwrap(), -1.0, 1e-12);
    }

    #[test]
    fn test_det_via_lu_singular() {
        let m = Matrix::<3, 3, f64>::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert!(m.det_via_lu().is_err());
    }

    #[test]
    fn test_inverse_diagonal() {
        let m = Matrix::<2, 2, f64>::new([[2.0, 0.0], [0.0, 4.0]]);
        let inv = m.inverse().unwrap();
        assert_close(inv.get(0, 0).unwrap(), 0.5, 1e-12);
        assert_close(inv.get(1, 1).unwrap(), 0.25, 1e-12);
    }

    #[test]
    fn test_inverse_round_trip() {
        // A · A⁻¹ = I
        let a = Matrix::<3, 3, f64>::new([
            [4.0, 7.0, 2.0],
            [3.0, 6.0, 1.0],
            [2.0, 5.0, 3.0],
        ]);
        let inv = a.inverse().unwrap();

        let mut prod = [[0.0_f64; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    prod[i][j] += a.data[i][k] * inv.data[k][j];
                }
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert_close(prod[i][j], expected, 1e-9);
            }
        }
    }

    #[test]
    fn test_inverse_singular_is_error() {
        let m = Matrix::<2, 2, f64>::new([[1.0, 2.0], [2.0, 4.0]]);
        assert!(m.inverse().is_err());
    }

    #[test]
    fn test_larger_system() {
        // 5x5：验证 O(N³) 路径在 N>3 时也正确
        let a = Matrix::<5, 5, f64>::new([
            [4.0, 1.0, 0.0, 0.0, 0.0],
            [1.0, 4.0, 1.0, 0.0, 0.0],
            [0.0, 1.0, 4.0, 1.0, 0.0],
            [0.0, 0.0, 1.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0, 4.0],
        ]);
        let b = [1.0, 2.0, 3.0, 4.0, 5.0];
        let x = a.solve(b).unwrap();

        let mut ax = [0.0_f64; 5];
        for i in 0..5 {
            for j in 0..5 {
                ax[i] += a.data[i][j] * x[j];
            }
        }
        for i in 0..5 {
            assert_close(ax[i], b[i], 1e-9);
        }
    }

    #[test]
    fn test_get_out_of_bounds() {
        let m = Matrix::<2, 2, f64>::new([[1.0, 2.0], [3.0, 4.0]]);
        assert!(m.get(2, 0).is_none());
        assert!(m.get(0, 2).is_none());
        assert!(m.get(1, 1).is_some());
    }
}
