use zmatrix::physics::basic::{
    AngularVelocity, MagneticAngularVelocity, MagneticInduction,
};

fn main() {
    println!("=== 磁角速度 (MagneticAngularVelocity) 示例 ===\n");

    // 1. 基本创建和转换
    println!("1. 基本创建和单位转换:");
    let mag_ang_vel = MagneticAngularVelocity::from_tesla_rad_per_second(2.5);
    println!("   从特斯拉·弧度/秒创建: {} T·rad/s", mag_ang_vel.as_tesla_rad_per_second());
    println!("   转换为毫特斯拉·弧度/秒: {} mT·rad/s", mag_ang_vel.as_mill_tesla_rad_per_second());
    println!("   转换为高斯·弧度/秒: {} G·rad/s", mag_ang_vel.as_gauss_rad_per_second());
    println!("   转换为千高斯·弧度/秒: {} kG·rad/s", mag_ang_vel.as_kilo_gauss_rad_per_second());

    // 2. 从磁感应强度和角速度计算磁角速度
    println!("\n2. 从磁感应强度和角速度计算磁角速度:");
    let magnetic_induction = MagneticInduction::from_tesla(1.5);
    let angular_velocity = AngularVelocity::from_rad_per_second(3.0);
    
    // 使用新实现的乘法运算
    let calculated_mag_ang_vel = magnetic_induction * angular_velocity;
    
    println!("   磁感应强度: {} T", magnetic_induction.as_tesla());
    println!("   角速度: {} rad/s", angular_velocity.as_rad_per_second());
    println!("   计算得到的磁角速度: {} T·rad/s", calculated_mag_ang_vel.as_tesla_rad_per_second());
    
    // 验证交换律
    let calculated_mag_ang_vel2 = angular_velocity * magnetic_induction;
    println!("   验证交换律 (角速度 × 磁感应强度): {} T·rad/s", calculated_mag_ang_vel2.as_tesla_rad_per_second());
    println!("   交换律验证: {}", if (calculated_mag_ang_vel.as_tesla_rad_per_second() - calculated_mag_ang_vel2.as_tesla_rad_per_second()).abs() < 1e-10 { "✓" } else { "✗" });

    // 3. 算术运算
    println!("\n3. 算术运算:");
    let mag_ang_vel1 = MagneticAngularVelocity::from_tesla_rad_per_second(2.0);
    let mag_ang_vel2 = MagneticAngularVelocity::from_tesla_rad_per_second(1.5);
    
    let sum = mag_ang_vel1 + mag_ang_vel2;
    let diff = mag_ang_vel1 - mag_ang_vel2;
    let scaled = mag_ang_vel1 * 2.5;
    let divided = mag_ang_vel1 / 2.0;
    
    println!("   {} T·rad/s + {} T·rad/s = {} T·rad/s", 
             mag_ang_vel1.as_tesla_rad_per_second(), 
             mag_ang_vel2.as_tesla_rad_per_second(), 
             sum.as_tesla_rad_per_second());
    println!("   {} T·rad/s - {} T·rad/s = {} T·rad/s", 
             mag_ang_vel1.as_tesla_rad_per_second(), 
             mag_ang_vel2.as_tesla_rad_per_second(), 
             diff.as_tesla_rad_per_second());
    println!("   {} T·rad/s × 2.5 = {} T·rad/s", 
             mag_ang_vel1.as_tesla_rad_per_second(), 
             scaled.as_tesla_rad_per_second());
    println!("   {} T·rad/s ÷ 2.0 = {} T·rad/s", 
             mag_ang_vel1.as_tesla_rad_per_second(), 
             divided.as_tesla_rad_per_second());

    // 4. 不同单位的创建和转换
    println!("\n4. 不同单位的创建和转换:");
    let from_mill_tesla = MagneticAngularVelocity::from_mill_tesla_rad_per_second(2500.0);
    let from_gauss = MagneticAngularVelocity::from_gauss_rad_per_second(15000.0);
    let from_kilo_gauss = MagneticAngularVelocity::from_kilo_gauss_rad_per_second(15.0);
    
    println!("   从毫特斯拉·弧度/秒创建: {} mT·rad/s", from_mill_tesla.as_mill_tesla_rad_per_second());
    println!("   转换为特斯拉·弧度/秒: {} T·rad/s", from_mill_tesla.as_tesla_rad_per_second());
    println!("   从高斯·弧度/秒创建: {} G·rad/s", from_gauss.as_gauss_rad_per_second());
    println!("   转换为特斯拉·弧度/秒: {} T·rad/s", from_gauss.as_tesla_rad_per_second());
    println!("   从千高斯·弧度/秒创建: {} kG·rad/s", from_kilo_gauss.as_kilo_gauss_rad_per_second());
    println!("   转换为特斯拉·弧度/秒: {} T·rad/s", from_kilo_gauss.as_tesla_rad_per_second());

    // 5. 物理应用示例：电机中的磁角速度
    println!("\n5. 物理应用示例 - 电机中的磁角速度:");
    let motor_field = MagneticInduction::from_tesla(0.8);  // 电机磁场强度
    let motor_speed = AngularVelocity::from_rad_per_second(314.0);  // 电机转速 (约3000 RPM)
    
    // 使用新实现的乘法运算
    let motor_mag_ang_vel = motor_field * motor_speed;
    
    println!("   电机磁场强度: {} T", motor_field.as_tesla());
    println!("   电机转速: {} rad/s (约 {} RPM)", 
             motor_speed.as_rad_per_second(), 
             motor_speed.as_rad_per_second() * 60.0 / (2.0 * std::f64::consts::PI));
    println!("   电机磁角速度: {} T·rad/s", motor_mag_ang_vel.as_tesla_rad_per_second());
    println!("   电机磁角速度: {} mT·rad/s", motor_mag_ang_vel.as_mill_tesla_rad_per_second());
    
    // 演示不同单位的磁感应强度
    let motor_field_gauss = MagneticInduction::from_gauss(8000.0);  // 0.8 T
    let motor_mag_ang_vel_gauss = motor_field_gauss * motor_speed;
    println!("   使用高斯单位: {} G × {} rad/s = {} T·rad/s", 
             motor_field_gauss.as_gauss(), 
             motor_speed.as_rad_per_second(),
             motor_mag_ang_vel_gauss.as_tesla_rad_per_second());

    // 6. 验证单位转换的一致性
    println!("\n6. 验证单位转换的一致性:");
    let test_value = MagneticAngularVelocity::from_tesla_rad_per_second(1.0);
    
    // 验证特斯拉和高斯之间的转换
    let tesla_value = test_value.as_tesla_rad_per_second();
    let gauss_value = test_value.as_gauss_rad_per_second();
    let back_to_tesla = MagneticAngularVelocity::from_gauss_rad_per_second(gauss_value).as_tesla_rad_per_second();
    
    println!("   原始值: {} T·rad/s", tesla_value);
    println!("   转换为高斯: {} G·rad/s", gauss_value);
    println!("   转换回特斯拉: {} T·rad/s", back_to_tesla);
    println!("   转换一致性: {}", if (tesla_value - back_to_tesla).abs() < 1e-10 { "✓" } else { "✗" });

    println!("\n=== 示例完成 ===");
}
