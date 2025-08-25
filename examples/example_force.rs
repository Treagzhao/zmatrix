use zmatrix::physics::basic::{Force, Distance, Energy, Mass, Acceleration, PhysicalQuantity};

fn main() {
    println!("=== 力物理量示例 ===\n");

    // 1. 创建不同单位的力
    println!("1. 创建不同单位的力:");
    let f1 = Force::from_newton(1.0); // 1 N
    let f2 = Force::from_mill_newton(1000.0); // 1000 mN = 1 N
    let f3 = Force::from_micro_newton(1000000.0); // 1000000 μN = 1 N
    let f4 = Force::from_nano_newton(1000000000.0); // 1000000000 nN = 1 N
    let f5 = Force::from_kilo_newton(0.001); // 0.001 kN = 1 N
    let f6 = Force::from_mega_newton(0.000001); // 0.000001 MN = 1 N

    println!("   f1 = {} N", f1.as_newton());
    println!("   f2 = {} mN = {} N", f2.as_mill_newton(), f2.as_newton());
    println!("   f3 = {} μN = {} N", f3.as_micro_newton(), f3.as_newton());
    println!("   f4 = {} nN = {} N", f4.as_nano_newton(), f4.as_newton());
    println!("   f5 = {} kN = {} N", f5.as_kilo_newton(), f5.as_newton());
    println!("   f6 = {} MN = {} N", f6.as_mega_newton(), f6.as_newton());

    // 2. 单位转换
    println!("\n2. 单位转换:");
    println!("   1 N = {} mN", f1.as_mill_newton());
    println!("   1 N = {} μN", f1.as_micro_newton());
    println!("   1 N = {} nN", f1.as_nano_newton());
    println!("   1 N = {} kN", f1.as_kilo_newton());
    println!("   1 N = {} MN", f1.as_mega_newton());

    // 3. 力的运算
    println!("\n3. 力的运算:");
    let f_sum = f1 + f2; // 1 N + 1 N = 2 N
    let f_diff = f_sum - f1; // 2 N - 1 N = 1 N
    let f_scale = f1 * 3.0; // 1 N * 3 = 3 N
    let f_div = f_scale / 2.0; // 3 N / 2 = 1.5 N

    println!("   f1 + f2 = {} N", f_sum.as_newton());
    println!("   f_sum - f1 = {} N", f_diff.as_newton());
    println!("   f1 * 3 = {} N", f_scale.as_newton());
    println!("   f_scale / 2 = {} N", f_div.as_newton());

    // 4. 力与距离的乘积（得到功/能量）
    println!("\n4. 力与距离的乘积（得到功/能量）:");
    let force = Force::from_newton(10.0); // 10 N
    let distance = Distance::from_m(5.0);  // 5 m
    let energy: Energy = force * distance; // 50 J

    println!("   力: {} N", force.as_newton());
    println!("   距离: {} m", distance.as_m());
    println!("   功/能量: {} J", energy.as_joule());
    println!("   验证: 10 N × 5 m = 50 J ✓");

    // 5. 距离与力的乘积（满足交换律）
    println!("\n5. 距离与力的乘积（满足交换律）:");
    let energy2: Energy = distance * force; // 同样是 50 J
    println!("   距离 × 力 = {} J", energy2.as_joule());
    println!("   验证: 5 m × 10 N = 50 J ✓");

    // 6. 质量与加速度的乘积（得到力）
    println!("\n6. 质量与加速度的乘积（得到力）:");
    let mass = Mass::from_kg(2.0);                    // 2 kg
    let acceleration = Acceleration::from_m_per_s2(5.0); // 5 m/s²
    let force_from_ma: Force = mass * acceleration;   // 10 N

    println!("   质量: {} kg", mass.as_kg());
    println!("   加速度: {} m/s²", acceleration.as_m_per_s2());
    println!("   力: {} N", force_from_ma.as_newton());
    println!("   验证: 2 kg × 5 m/s² = 10 N ✓");

    // 7. 加速度与质量的乘积（满足交换律）
    println!("\n7. 加速度与质量的乘积（满足交换律）:");
    let force_from_am: Force = acceleration * mass;   // 同样是 10 N
    println!("   加速度 × 质量 = {} N", force_from_am.as_newton());
    println!("   验证: 5 m/s² × 2 kg = 10 N ✓");

    // 8. 不同单位之间的运算
    println!("\n8. 不同单位之间的运算:");
    let f_micro = Force::from_micro_newton(1000000.0); // 1 N
    let f_kilo = Force::from_kilo_newton(0.001);       // 1 N
    let f_result = f_micro + f_kilo; // 1 N + 1 N = 2 N
    println!("   {} μN + {} kN = {} N",
             f_micro.as_micro_newton(),
             f_kilo.as_kilo_newton(),
             f_result.as_newton());

    // 9. 零值检查
    println!("\n9. 零值检查:");
    let f_zero = Force::from_newton(0.0);
    let f_nonzero = Force::from_newton(1.0);
    println!("   f_zero.is_zero() = {}", f_zero.is_zero());
    println!("   f_nonzero.is_zero() = {}", f_nonzero.is_zero());

    // 10. 实际应用示例
    println!("\n10. 实际应用示例:");
    
    // 重力计算
    let mass_earth = Mass::from_kg(70.0); // 70 kg的人
    let g = Acceleration::from_m_per_s2(9.81); // 重力加速度
    let weight: Force = mass_earth * g; // 686.7 N
    println!("   人体重量: {} kg × {} m/s² = {} N", 
             mass_earth.as_kg(), g.as_m_per_s2(), weight.as_newton());

    // 弹簧做功
    let spring_force = Force::from_newton(20.0); // 20 N的弹簧力
    let compression = Distance::from_m(0.1); // 压缩0.1 m
    let spring_energy: Energy = spring_force * compression; // 2 J
    println!("   弹簧做功: {} N × {} m = {} J",
             spring_force.as_newton(), compression.as_m(), spring_energy.as_joule());

    // 汽车牵引力
    let engine_force = Force::from_kilo_newton(2.0); // 2 kN的发动机力
    let travel_distance = Distance::from_m(100.0); // 行驶100 m
    let work_done: Energy = engine_force * travel_distance; // 200 kJ
    println!("   发动机做功: {} kN × {} m = {} kJ",
             engine_force.as_kilo_newton(), travel_distance.as_m(), work_done.as_kilo_joule());

    println!("\n=== 示例完成 ===");
}
