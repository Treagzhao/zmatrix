use zmatrix::physics::basic::{Coef, Vector3};

fn main() {
    // 测试叉积公式
    let a = Vector3::new(
        Coef::new(1.0),  // a_x = 1
        Coef::new(0.0),  // a_y = 0
        Coef::new(0.0),  // a_z = 0
    );
    
    let b = Vector3::new(
        Coef::new(0.0),  // b_x = 0
        Coef::new(1.0),  // b_y = 1
        Coef::new(0.0),  // b_z = 0
    );
    
    // 计算叉积 a × b
    let result = a.cross(&b);
    
    println!("a = ({}, {}, {})", a.x.get_value(), a.y.get_value(), a.z.get_value());
    println!("b = ({}, {}, {})", b.x.get_value(), b.y.get_value(), b.z.get_value());
    println!("a × b = ({}, {}, {})", result.x.get_value(), result.y.get_value(), result.z.get_value());
    
    // 根据标准公式验证：
    // x = a_y * b_z - a_z * b_y = 0 * 0 - 0 * 1 = 0
    // y = a_z * b_x - a_x * b_z = 0 * 0 - 1 * 0 = 0  
    // z = a_x * b_y - a_y * b_x = 1 * 1 - 0 * 0 = 1
    println!("期望结果: (0, 0, 1)");
    
    // 测试另一个例子
    let c = Vector3::new(
        Coef::new(1.0),  // c_x = 1
        Coef::new(2.0),  // c_y = 2
        Coef::new(3.0),  // c_z = 3
    );
    
    let d = Vector3::new(
        Coef::new(4.0),  // d_x = 4
        Coef::new(5.0),  // d_y = 5
        Coef::new(6.0),  // d_z = 6
    );
    
    let result2 = c.cross(&d);
    
    println!("\nc = ({}, {}, {})", c.x.get_value(), c.y.get_value(), c.z.get_value());
    println!("d = ({}, {}, {})", d.x.get_value(), d.y.get_value(), d.z.get_value());
    println!("c × d = ({}, {}, {})", result2.x.get_value(), result2.y.get_value(), result2.z.get_value());
    
    // 手动计算验证：
    // x = c_y * d_z - c_z * d_y = 2 * 6 - 3 * 5 = 12 - 15 = -3
    // y = c_z * d_x - c_x * d_z = 3 * 4 - 1 * 6 = 12 - 6 = 6
    // z = c_x * d_y - c_y * d_x = 1 * 5 - 2 * 4 = 5 - 8 = -3
    println!("期望结果: (-3, 6, -3)");
}
