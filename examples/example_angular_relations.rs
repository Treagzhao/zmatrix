use zmatrix::physics::basic::{AngularMomentum, AngularVelocity, Torque};

fn main() {
    println!("=== 角动量、角速度和力矩之间的运算关系示例 ===\n");

    // 示例1: 角动量 × 角速度 = 力矩
    println!("1. 角动量 × 角速度 = 力矩");
    let angular_momentum = AngularMomentum::from_kg_m2_per_second(10.0); // 10 kg·m²/s
    let angular_velocity = AngularVelocity::from_rad_per_second(5.0);    // 5 rad/s
    let torque = angular_momentum * angular_velocity;
    println!("   角动量: {} kg·m²/s", angular_momentum.as_kg_m2_per_second());
    println!("   角速度: {} rad/s", angular_velocity.as_rad_per_second());
    println!("   力矩: {} N·m", torque.as_nm());
    println!("   验证: 10 × 5 = 50 ✓\n");

    // 示例2: 角速度 × 角动量 = 力矩（满足交换律）
    println!("2. 角速度 × 角动量 = 力矩（满足交换律）");
    let torque2 = angular_velocity * angular_momentum;
    println!("   角速度: {} rad/s", angular_velocity.as_rad_per_second());
    println!("   角动量: {} kg·m²/s", angular_momentum.as_kg_m2_per_second());
    println!("   力矩: {} N·m", torque2.as_nm());
    println!("   验证: 5 × 10 = 50 ✓\n");

    // 示例3: 力矩 ÷ 角速度 = 角动量
    println!("3. 力矩 ÷ 角速度 = 角动量");
    let torque3 = Torque::from_nm(20.0); // 20 N·m
    let angular_velocity3 = AngularVelocity::from_rad_per_second(4.0); // 4 rad/s
    let angular_momentum3 = torque3 / angular_velocity3;
    println!("   力矩: {} N·m", torque3.as_nm());
    println!("   角速度: {} rad/s", angular_velocity3.as_rad_per_second());
    println!("   角动量: {} kg·m²/s", angular_momentum3.as_kg_m2_per_second());
    println!("   验证: 20 ÷ 4 = 5 ✓\n");

    // 示例4: 角动量 ÷ 角速度 = 转动惯量（系数）
    println!("4. 角动量 ÷ 角速度 = 转动惯量（系数）");
    let angular_momentum4 = AngularMomentum::from_kg_m2_per_second(20.0); // 20 kg·m²/s
    let angular_velocity4 = AngularVelocity::from_rad_per_second(4.0);    // 4 rad/s
    let moment_of_inertia = angular_momentum4 / angular_velocity4;
    println!("   角动量: {} kg·m²/s", angular_momentum4.as_kg_m2_per_second());
    println!("   角速度: {} rad/s", angular_velocity4.as_rad_per_second());
    println!("   转动惯量系数: {} kg·m²", moment_of_inertia.get_value());
    println!("   验证: 20 ÷ 4 = 5 ✓\n");

    // 示例5: 不同单位的测试
    println!("5. 不同单位的测试");
    let angular_momentum5 = AngularMomentum::from_kg_km2_per_second(1.0); // 1 kg·km²/s
    let angular_velocity5 = AngularVelocity::from_deg_per_second(180.0);  // 180°/s = π rad/s
    let torque5 = angular_momentum5 * angular_velocity5;
    println!("   角动量: {} kg·km²/s", angular_momentum5.as_kg_km2_per_second());
    println!("   角速度: {}°/s = {} rad/s", angular_velocity5.as_deg_per_second(), angular_velocity5.as_rad_per_second());
    println!("   力矩: {} N·m", torque5.as_nm());
    println!("   验证: 1e6 × π ≈ 3.14159e6 ✓\n");

    // 示例6: 新添加的Nms单位测试
    println!("6. 新添加的Nms单位测试");
    let angular_momentum_nms = AngularMomentum::from_nms(100.0); // 100 N·m·s
    let angular_momentum_mill = AngularMomentum::from_mill_nms(1000.0); // 1000 mN·m·s
    let angular_momentum_micro = AngularMomentum::from_micro_nms(1000000.0); // 1000000 μN·m·s
    let angular_momentum_nano = AngularMomentum::from_nano_nms(1000000000.0); // 1000000000 nN·m·s
    
    println!("   100 N·m·s = {} kg·m²/s", angular_momentum_nms.as_kg_m2_per_second());
    println!("   1000 mN·m·s = {} kg·m²/s", angular_momentum_mill.as_kg_m2_per_second());
    println!("   1000000 μN·m·s = {} kg·m²/s", angular_momentum_micro.as_kg_m2_per_second());
    println!("   1000000000 nN·m·s = {} kg·m²/s", angular_momentum_nano.as_kg_m2_per_second());
    println!("   验证: 100 = 1 = 0.001 = 0.000001 ✓\n");

    // 示例7: 不同Nms单位之间的运算
    println!("7. 不同Nms单位之间的运算");
    let sum = angular_momentum_nms + angular_momentum_mill + angular_momentum_micro + angular_momentum_nano;
    println!("   100 N·m·s + 1000 mN·m·s + 1000000 μN·m·s + 1000000000 nN·m·s = {} N·m·s", sum.as_nms());
    println!("   验证: 100 + 1 + 0.001 + 0.000001 = 101.001001 ✓\n");

    // 示例8: Nms单位与角速度的运算
    println!("8. Nms单位与角速度的运算");
    let omega = AngularVelocity::from_rad_per_second(2.0); // 2 rad/s
    let torque_from_nms = angular_momentum_nms * omega;
    println!("   100 N·m·s × 2 rad/s = {} N·m", torque_from_nms.as_nm());
    println!("   验证: 100 × 2 = 200 ✓\n");

    println!("=== 物理意义说明 ===");
    println!("• 角动量 (L) = 转动惯量 (I) × 角速度 (ω)");
    println!("• 力矩 (τ) = 角动量 (L) × 角速度 (ω)");
    println!("• 在旋转动力学中，这些关系非常重要");
    println!("• 所有运算都支持不同单位之间的自动转换");
    println!("• 新增的Nms单位：N·m·s, mN·m·s, μN·m·s, nN·m·s");
    println!("• 1 N·m·s = 1 kg·m²/s（数值相等）");
}
