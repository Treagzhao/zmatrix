use zmatrix::physics::basic::{Power, Energy, Force, Velocity, PhysicalQuantity};
use std::time::Duration;

fn main() {
    println!("=== 功率物理量示例 ===\n");

    // 1. 创建不同单位的功率
    println!("1. 创建不同单位的功率:");
    let p1 = Power::from_watt(1.0); // 1 W
    let p2 = Power::from_mill_watt(1000.0); // 1000 mW = 1 W
    let p3 = Power::from_micro_watt(1000000.0); // 1000000 μW = 1 W
    let p4 = Power::from_nano_watt(1000000000.0); // 1000000000 nW = 1 W
    let p5 = Power::from_kilo_watt(0.001); // 0.001 kW = 1 W
    let p6 = Power::from_mega_watt(0.000001); // 0.000001 MW = 1 W
    let p7 = Power::from_horse_power(1.0 / 745.7); // 1/745.7 hp ≈ 1 W

    println!("   p1 = {} W", p1.as_watt());
    println!("   p2 = {} mW = {} W", p2.as_mill_watt(), p2.as_watt());
    println!("   p3 = {} μW = {} W", p3.as_micro_watt(), p3.as_watt());
    println!("   p4 = {} nW = {} W", p4.as_nano_watt(), p4.as_watt());
    println!("   p5 = {} kW = {} W", p5.as_kilo_watt(), p5.as_watt());
    println!("   p6 = {} MW = {} W", p6.as_mega_watt(), p6.as_watt());
    println!("   p7 = {} hp = {} W", p7.as_horse_power(), p7.as_watt());

    // 2. 单位转换
    println!("\n2. 单位转换:");
    println!("   1 W = {} mW", p1.as_mill_watt());
    println!("   1 W = {} μW", p1.as_micro_watt());
    println!("   1 W = {} nW", p1.as_nano_watt());
    println!("   1 W = {} kW", p1.as_kilo_watt());
    println!("   1 W = {} MW", p1.as_mega_watt());
    println!("   1 W = {} hp", p1.as_horse_power());

    // 3. 功率的运算
    println!("\n3. 功率的运算:");
    let p_sum = p1 + p2; // 1 W + 1 W = 2 W
    let p_diff = p_sum - p1; // 2 W - 1 W = 1 W
    let p_scale = p1 * 3.0; // 1 W * 3 = 3 W
    let p_div = p_scale / 2.0; // 3 W / 2 = 1.5 W

    println!("   p1 + p2 = {} W", p_sum.as_watt());
    println!("   p_sum - p1 = {} W", p_diff.as_watt());
    println!("   p1 * 3 = {} W", p_scale.as_watt());
    println!("   p_scale / 2 = {} W", p_div.as_watt());

    // 4. 功率与时间的乘积（得到能量）
    println!("\n4. 功率与时间的乘积（得到能量）:");
    let power = Power::from_watt(100.0); // 100 W
    let time = Duration::from_secs(5); // 5 s
    let energy: Energy = power * time; // 500 J

    println!("   功率: {} W", power.as_watt());
    println!("   时间: {} s", time.as_secs_f64());
    println!("   能量: {} J", energy.as_joule());
    println!("   验证: 100 W × 5 s = 500 J ✓");

    // 5. 时间与功率的乘积（满足交换律）
    println!("\n5. 时间与功率的乘积（满足交换律）:");
    let energy2: Energy = time * power; // 同样是 500 J
    println!("   时间 × 功率 = {} J", energy2.as_joule());
    println!("   验证: 5 s × 100 W = 500 J ✓");

    // 6. 力与速度的乘积（得到功率）
    println!("\n6. 力与速度的乘积（得到功率）:");
    let force = Force::from_newton(10.0); // 10 N
    let velocity = Velocity::from_m_per_sec(5.0); // 5 m/s
    let power_from_fv: Power = force * velocity; // 50 W

    println!("   力: {} N", force.as_newton());
    println!("   速度: {} m/s", velocity.as_m_per_sec());
    println!("   功率: {} W", power_from_fv.as_watt());
    println!("   验证: 10 N × 5 m/s = 50 W ✓");

    // 7. 速度与力的乘积（满足交换律）
    println!("\n7. 速度与力的乘积（满足交换律）:");
    let power_from_vf: Power = velocity * force; // 同样是 50 W
    println!("   速度 × 力 = {} W", power_from_vf.as_watt());
    println!("   验证: 5 m/s × 10 N = 50 W ✓");

    // 8. 不同单位之间的运算
    println!("\n8. 不同单位之间的运算:");
    let p_micro = Power::from_micro_watt(1000000.0); // 1 W
    let p_kilo = Power::from_kilo_watt(0.001);       // 1 W
    let p_result = p_micro + p_kilo; // 1 W + 1 W = 2 W
    println!("   {} μW + {} kW = {} W",
             p_micro.as_micro_watt(),
             p_kilo.as_kilo_watt(),
             p_result.as_watt());

    // 9. 马力转换
    println!("\n9. 马力转换:");
    let hp = Power::from_horse_power(1.0); // 1 hp
    println!("   1 hp = {} W", hp.as_watt());
    
    let watt = Power::from_watt(745.7); // 745.7 W
    println!("   745.7 W = {} hp", watt.as_horse_power());

    // 10. 零值检查
    println!("\n10. 零值检查:");
    let p_zero = Power::from_watt(0.0);
    let p_nonzero = Power::from_watt(1.0);
    println!("   p_zero.is_zero() = {}", p_zero.is_zero());
    println!("   p_nonzero.is_zero() = {}", p_nonzero.is_zero());

    // 11. 实际应用示例
    println!("\n11. 实际应用示例:");
    
    // 电灯泡功率
    let lightbulb = Power::from_watt(60.0); // 60 W灯泡
    let usage_time = Duration::from_secs(3600); // 1小时
    let energy_consumed: Energy = lightbulb * usage_time; // 216 kJ
    println!("   60W灯泡使用1小时消耗: {} kJ", energy_consumed.as_kilo_joule());

    // 汽车发动机功率
    let engine = Power::from_horse_power(200.0); // 200 hp发动机
    println!("   200 hp发动机 = {} kW", engine.as_kilo_watt());
    
    // 风力发电机
    let wind_turbine = Power::from_mega_watt(2.0); // 2 MW风力发电机
    println!("   2 MW风力发电机 = {} hp", wind_turbine.as_horse_power());

    // 12. 功率与时间的复杂计算
    println!("\n12. 功率与时间的复杂计算:");
    
    // 不同功率设备的使用
    let tv = Power::from_watt(100.0); // 100 W电视
    let tv_time = Duration::from_secs(7200); // 2小时
    
    let computer = Power::from_watt(300.0); // 300 W电脑
    let computer_time = Duration::from_secs(3600); // 1小时
    
    let total_energy: Energy = tv * tv_time + computer * computer_time;
    println!("   电视2小时 + 电脑1小时 = {} kWh", total_energy.as_kilo_joule() / 3600.0);

    // 13. 机械功率计算
    println!("\n13. 机械功率计算:");
    
    // 电梯功率
    let elevator_force = Force::from_kilo_newton(10.0); // 10 kN
    let elevator_speed = Velocity::from_m_per_sec(2.0); // 2 m/s
    let elevator_power: Power = elevator_force * elevator_speed; // 20 kW
    println!("   电梯功率: {} kN × {} m/s = {} kW", 
             elevator_force.as_kilo_newton(), 
             elevator_speed.as_m_per_sec(), 
             elevator_power.as_kilo_watt());

    // 汽车牵引功率
    let car_force = Force::from_newton(2000.0); // 2000 N牵引力
    let car_speed = Velocity::from_km_per_h(80.0); // 80 km/h
    let car_power: Power = car_force * car_speed; // 约44.4 kW
    println!("   汽车牵引功率: {} N × {} km/h = {} kW", 
             car_force.as_newton(), 
             car_speed.as_km_per_h(), 
             car_power.as_kilo_watt());

    println!("\n=== 示例完成 ===");
}
