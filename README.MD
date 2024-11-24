**这是一个矩阵运算的库，能够实现一些简单的矩阵预算的功能**

#### 创建一个矩阵
> create a new matrix
``` rust
let m1 = dense::new(2,3,vec![1,2,3,4,5,6]);
println!("{}",m1);
```

#### 获取值以及设置值
> get value & set value
``` rust
  let m1 = dense::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
  let value = m1.get(1, 1);
  if let Some(x) = value {
     println!("get value:{}", x); // 5
  }
  
  m1.set(1,1,10);
  let value = m1.get(1, 1);
  if let Some(x) = value {
     println!("get value:{}", x); // 10
  }
```
#### 矩阵加减乘的运算
> Matrix addition, subtraction, and multiplication operations
``` rust
    // 加法运算 add
    let m1 = dense::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = dense::new(3, 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let m3 = m1 + m2;
    println!("add result:\n{}", m3);
    //减法运算 sub
     let m1 = dense::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = dense::new(3, 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let m3 = m2 - m1;
    println!("sub result:\n{}", m3);
    //乘法运算 multiple
    let m1 = dense::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = dense::new(3, 3, vec![4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let m3 = m2 * m1;
    println!("multi result:\n{}", m3);
```
#### 矩阵乘法
> Matrix product
``` rust
    let m1 = dense::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = dense::new(3, 4, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let result = m1.product(m2).unwrap();
    println!("matrix product:\n{}", result);
```

#### 矩阵的数乘
> Scalar multiplication of a matrix
``` rust
    let m1 = dense::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = m1.scale(3);
    println!("\nscale result:\n{}", m2);
```

#### 矩阵的转置
> Matrix transpose
``` rust
    let m1 = dense::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = !m1;
    println!("\ntransform:\n{}", m2);
```

#### 矩阵的行列式
> matrix determinant
``` rust
    let m1 = dense::new(2, 3, vec![1, 2, 3, 4, 5, 6]).unwrap();
    let m2 = !m1;
    println!("\ntransform:\n{}", m2);
```
