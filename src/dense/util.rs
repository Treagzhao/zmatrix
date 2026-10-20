use crate::dense::error;
use std::convert::From;
use std::fmt::Display;
use std::ops::{Add, Mul};

pub fn print_single_line<const ROWS: usize, const COLS: usize, T>(data: &[[T;COLS];ROWS], digits: u8) -> String
where
    T: Display,
{
    let mut line = String::from("[");
    for row in 0..ROWS {
        for col in 0..COLS {
            let str = format!("{:<digit$}", data[row][col], digit = (digits + 2) as usize);
            line.push_str(&str);
        }
    }
    line.push_str("]\n");
    line.to_string()
}

pub fn get_boundary_char(row: usize, height: usize) -> (String, String) {
    if height == 1 {
        return ("[".to_string(), "]".to_string());
    } else if row == 0 {
        return ("┌".to_string(), "┐".to_string());
    } else if row == height - 1 {
        return ("└".to_string(), "┘".to_string());
    } else {
        return ("|".to_string(), "|".to_string());
    }
}

pub fn determinant_in_one_permutation<const N: usize, T>(
    data: &[[T;N];N],
    permutation: &Vec<u64>,
) -> Result<T, error::OperationError>
where
    T: Copy + Display + Default + Send + Sync + From<i8> + Add<Output=T> + Mul<Output=T>,
{
    let inverse = inversion_number(permutation);
    let ceof: i8 = if inverse % 2 == 0 { 1 } else { -1 };
    let _key = print_permutation(permutation);
    let mut sum = T::default() + T::from(1i8);
    for (i, item) in permutation.iter().enumerate() {
        let y = i;
        let x = *item as usize;
        sum = sum * data[y][x];
    }
    Result::Ok(sum * ceof.into())
}

pub fn print_permutation(permutation: &Vec<u64>) -> String {
    let mut result = String::from("");
    for i in permutation {
        let str = format!("{}", *i);
        result.push_str(&str);
    }
    result
}

pub fn permutation(n: usize) -> Result<Vec<Vec<u64>>, error::OperationError> {
    let mut available: Vec<u64> = vec![0; n];
    let mut flags: Vec<bool> = vec![false; n];
    let mut list: Vec<u64> = Vec::new();
    let mut result: Vec<Vec<u64>> = Vec::new();
    for i in 0..n {
        available[i as usize] = i as u64;
    }
    let _ = fill_in_permutation(&available, &mut flags, &mut list, &mut result, 0)?;
    Ok(result)
}

fn fill_in_permutation(
    available: &Vec<u64>,
    flags: &mut Vec<bool>,
    list: &mut Vec<u64>,
    result: &mut Vec<Vec<u64>>,
    level: u32,
) -> Result<(), error::OperationError> {
    if available.len() != flags.len() {
        return Result::Err(error::OperationError {
            message: "flags length does not match available`s length".to_string(),
        });
    }
    if level as usize >= available.len() {
        let r = list.clone();
        result.push(r);
        return Result::Ok(());
    }
    for (i, x) in available.iter().enumerate() {
        let flag = flags[i];
        if flag {
            continue;
        }
        list.push(*x);
        flags[i] = true;
        fill_in_permutation(available, flags, list, result, level + 1)?;
        flags[i] = false;
        list.pop();
    }
    Result::Ok(())
}

pub fn inversion_number(vec: &Vec<u64>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..vec.len() - 1 {
        for j in i + 1..vec.len() {
            if vec[i] > vec[j] {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_print_single_line() {
        let a: [[i32;5];1] = [[1, 2, 3, 4, 15]];
        let result = print_single_line(&a, 1);
        println!("{}", result.clone());
        assert_eq!("[1  2  3  4  15 ]\n", result);
    }

    #[test]
    fn test_get_boundary_char() {
        let (start, end) = get_boundary_char(2, 1);
        assert_eq!("[", start);
        assert_eq!("]", end);
        let (start, end) = get_boundary_char(0, 3);
        assert_eq!("┌", start);
        assert_eq!("┐", end);
        let (start, end) = get_boundary_char(1, 3);
        assert_eq!("|", start);
        assert_eq!("|", end);
        let (start, end) = get_boundary_char(2, 3);
        assert_eq!("└", start);
        assert_eq!("┘", end);
    }

    #[test]
    fn test_fill_in_permutation() {
        let a: Vec<u64> = vec![1, 2, 3, 4];
        let mut flags: Vec<bool> = vec![false; a.len()];
        let mut list: Vec<u64> = Vec::new();
        let mut result: Vec<Vec<u64>> = Vec::new();
        let res = fill_in_permutation(&a, &mut flags, &mut list, &mut result, 0);
        assert!(res.is_ok());
        if let Ok(_vec) = res {
            println!("{:?}", result);
            assert_eq!(24, result.len());
        }
    }

    #[test]
    fn test_permutation() {
        let res = permutation(2);
        assert!(res.is_ok());
        if let Ok(vec) = res {
            assert_eq!(2, vec.len());
        }

        let res = permutation(4);
        assert!(res.is_ok());
        if let Ok(vec) = res {
            assert_eq!(24, vec.len());
        }

        let res = permutation(6);
        assert!(res.is_ok());
        if let Ok(vec) = res {
            assert_eq!(720, vec.len());
        }
    }

    #[test]
    fn test_inversion_number() {
        // 测试用例1
        {
            let vec = vec![1, 2, 3, 4];
            let res = inversion_number(&vec);
            assert_eq!(res, 0);
        }
        // 测试用例2
        {
            let vec = vec![4, 3, 2, 1];
            let res = inversion_number(&vec);
            assert_eq!(res, 6);
        }
        // 测试用例3
        {
            let vec = vec![2, 4, 1, 3];
            let res = inversion_number(&vec);
            assert_eq!(res, 3);
        }
        // 测试用例4
        {
            let vec = vec![3, 1, 4, 2];
            let res = inversion_number(&vec);
            assert_eq!(res, 3);
        }
        // 测试用例5
        {
            let vec = vec![5, 3, 1, 4, 2];
            let res = inversion_number(&vec);
            assert_eq!(res, 7);
        }
        // 测试用例6
        {
            let vec = vec![1, 3, 2, 4, 5];
            let res = inversion_number(&vec);
            assert_eq!(res, 1);
        }
        // 测试用例7
        {
            let vec = vec![4, 2, 3, 1];
            let res = inversion_number(&vec);
            assert_eq!(res, 5);
        }
        // 测试用例8
        {
            let vec = vec![3, 4, 1, 2];
            let res = inversion_number(&vec);
            assert_eq!(res, 4);
        }
        {
            let vec = vec![0, 1, 2, 4, 3];
            let res = inversion_number(&vec);
            println!("{}", res);
            assert_eq!(res, 1);
        }
    }

    #[test]
    fn test_determinant_in_one_permutation() {
        let data: [[i32;2];2] = [[1, 2], [3, 4]];
        let permutation: Vec<u64> = vec![0, 1];
        let result = determinant_in_one_permutation(&data, &permutation);
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(4, value);
        }

        let permutation: Vec<u64> = vec![1, 0];
        let result = determinant_in_one_permutation(&data, &permutation);
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(-6, value);
        }
    }

    #[test]
    fn test_print_permutation() {
        let vec: Vec<u64> = vec![1, 2, 3, 4];
        let result = print_permutation(&vec);
        assert_eq!("1234", result);
    }
}
