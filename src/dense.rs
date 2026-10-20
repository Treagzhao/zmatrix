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

impl<const ROWS: usize, const COLS: usize, T> Debug for Matrix<ROWS, COLS, T>
where
    T: NumericElement + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl<const ROWS: usize, const COLS: usize, T> Display for Matrix<ROWS, COLS, T>
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
}
