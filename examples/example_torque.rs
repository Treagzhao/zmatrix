use zmatrix::physics::basic::{Torque, Distance, Energy, PhysicalQuantity};

fn main() {
    println!("=== 力矩物理量示例 ===\n");

    // 1. 创建不同单位的力矩
    println!("1. 创建不同单位的力矩:");
    let t1 = Torque::from_nm(1.0); // 1 N·m
    let t2 = Torque::from_mill_nm(1000.0); // 1000 mN·m = 1 N·m
    let t3 = Torque::from_micro_nm(1e6); // 1e6 μN·m = 1 N·m
    let t4 = Torque::from_nano_nm(1e9); // 1e9 nN·m = 1 N·m
    let t5 = Torque::from_knm(1e-3); // 1e-3 kN·m = 1 N·m
    let t6 = Torque::from_mnm(1e-6); // 1e-6 MN·m = 1 N·m

    println!("   t1 = {} N·m", t1.as_nm());
    println!("   t2 = {} mN·m = {} N·m", t2.as_mill_nm(), t2.as_nm());
    println!("   t3 = {} μN·m = {} N·m", t3.as_micro_nm(), t3.as_nm());
    println!("   t4 = {} nN·m = {} N·m", t4.as_nano_nm(), t4.as_nm());
    println!("   t5 = {} kN·m = {} N·m", t5.as_knm(), t5.as_nm());
    println!("   t6 = {} MN·m = {} N·m", t6.as_mnm(), t6.as_nm());

    // 2. 单位转换
    println!("\n2. 单位转换:");
    println!("   1 N·m = {} mN·m", t1.as_mill_nm());
    println!("   1 N·m = {} μN·m", t1.as_micro_nm());
    println!("   1 N·m = {} nN·m", t1.as_nano_nm());
    println!("   1 N·m = {} kN·m", t1.as_knm());
    println!("   1 N·m = {} MN·m", t1.as_mnm());

    // 3. 力矩运算
    println!("\n3. 力矩运算:");
    let t_sum = t1 + t2; // 1 N·m + 1 N·m = 2 N·m
    let t_diff = t_sum - t1; // 2 N·m - 1 N·m = 1 N·m
    let t_scale = t1 * 3.0; // 1 N·m * 3 = 3 N·m
    let t_div = t_scale / 2.0; // 3 N·m / 2 = 1.5 N·m

    println!("   t1 + t2 = {} N·m", t_sum.as_nm());
    println!("   t_sum - t1 = {} N·m", t_diff.as_nm());
    println!("   t1 * 3 = {} N·m", t_scale.as_nm());
    println!("   t_scale / 2 = {} N·m", t_div.as_nm());

    // 4. 力矩与距离的乘积（得到功/能量）
    println!("\n4. 力矩与距离的乘积（得到功/能量）:");
    let d = Distance::from_m(2.0); // 2 m
    let work = t1 * d; // 1 N·m × 2 m = 2 J
    println!("   力矩: {} N·m", t1.as_nm());
    println!("   距离: {} m", d.as_m());
    println!("   功/能量: {} J", work.as_joule());

    // 5. 不同单位之间的运算
    println!("\n5. 不同单位之间的运算:");
    let t_micro = Torque::from_micro_nm(1e6); // 1 N·m
    let t_mill = Torque::from_mill_nm(1000.0); // 1 N·m
    let t_result = t_micro + t_mill; // 1 N·m + 1 N·m = 2 N·m
    println!("   {} μN·m + {} mN·m = {} N·m", 
             t_micro.as_micro_nm(), 
             t_mill.as_mill_nm(), 
             t_result.as_nm());

    // 6. 标量与力矩的运算
    println!("\n6. 标量与力矩的运算:");
    let t_scaled = 5.0 * t1; // 5 × 1 N·m = 5 N·m
    let t_divided = 10.0 / t1; // 10 / 1 N·m = 10 N·m
    println!("   5.0 * {} N·m = {} N·m", t1.as_nm(), t_scaled.as_nm());
    println!("   10.0 / {} N·m = {} N·m", t1.as_nm(), t_divided.as_nm());

    // 7. 零值检查
    println!("\n7. 零值检查:");
    let t_zero = Torque::from_nm(0.0);
    let t_nonzero = Torque::from_nm(1.0);
    println!("   t_zero.is_zero() = {}", t_zero.is_zero());
    println!("   t_nonzero.is_zero() = {}", t_nonzero.is_zero());

    // 8. 实际应用示例
    println!("\n8. 实际应用示例:");
    
    // 汽车发动机扭矩
    let engine_torque = Torque::from_nm(300.0); // 300 N·m
    let wheel_radius = Distance::from_m(0.3); // 0.3 m
    let traction_force = engine_torque.as_nm() / wheel_radius.as_m(); // F = τ/r
    println!("   发动机扭矩: {} N·m", engine_torque.as_nm());
    println!("   车轮半径: {} m", wheel_radius.as_m());
    println!("   牵引力: {} N", traction_force);

    // 杠杆原理
    let applied_force = 100.0; // 100 N
    let lever_arm = Distance::from_m(0.5); // 0.5 m
    let lever_torque = Torque::from_nm(applied_force * lever_arm.as_m()); // τ = F × r
    println!("   施加力: {} N", applied_force);
    println!("   力臂: {} m", lever_arm.as_m());
    println!("   杠杆力矩: {} N·m", lever_torque.as_nm());

    println!("\n=== 示例完成 ===");
}
