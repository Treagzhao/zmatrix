use zmatrix::physics::basic::*;
use std::time::Duration;

fn main() {
    println!("=== 新增物理量运算关系示例 ===\n");

    // 1. 能量 ÷ 时间 = 功率
    println!("1. 能量 ÷ 时间 = 功率");
    let energy = Energy::from_joule(1000.0); // 1000 J
    let time = Duration::from_secs(10); // 10 s
    let power: Power = energy / time; // 100 W
    println!("   1000 J ÷ 10 s = {} W", power.as_watt());
    println!("   验证: {} W × 10 s = {} J\n", power.as_watt(), (power * time).as_joule());

    // 2. 力矩 ÷ 距离 = 力
    println!("2. 力矩 ÷ 距离 = 力");
    let torque = Torque::from_nm(50.0); // 50 N·m
    let distance = Distance::from_m(5.0); // 5 m
    let force: Force = torque / distance; // 10 N
    println!("   50 N·m ÷ 5 m = {} N", force.as_newton());
    println!("   验证: {} N × 5 m = {} J\n", force.as_newton(), (force * distance).as_joule());

    // 3. 速度 ÷ 加速度 = 时间
    println!("3. 速度 ÷ 加速度 = 时间");
    let velocity = Velocity::from_m_per_sec(20.0); // 20 m/s
    let acceleration = Acceleration::from_m_per_s2(4.0); // 4 m/s²
    let time = velocity / acceleration; // 5 s
    println!("   20 m/s ÷ 4 m/s² = {} s", time.as_secs_f64());
    println!("   验证: 4 m/s² × {} s = {} m/s\n", time.as_secs_f64(), (acceleration * time).as_m_per_sec());

    // 4. 距离 ÷ 速度 = 时间
    println!("4. 距离 ÷ 速度 = 时间");
    let distance = Distance::from_m(200.0); // 200 m
    let velocity = Velocity::from_m_per_sec(25.0); // 25 m/s
    let time = distance / velocity; // 8 s
    println!("   200 m ÷ 25 m/s = {} s", time.as_secs_f64());
    println!("   验证: 25 m/s × {} s = {} m\n", time.as_secs_f64(), (velocity * time).as_m());

    // 5. 功率 ÷ 力 = 速度
    println!("5. 功率 ÷ 力 = 速度");
    let power = Power::from_watt(300.0); // 300 W
    let force = Force::from_newton(15.0); // 15 N
    let velocity: Velocity = power / force; // 20 m/s
    println!("   300 W ÷ 15 N = {} m/s", velocity.as_m_per_sec());
    println!("   验证: 15 N × {} m/s = {} W\n", velocity.as_m_per_sec(), (force * velocity).as_watt());

    // 6. 功率 ÷ 速度 = 力
    println!("6. 功率 ÷ 速度 = 力");
    let power = Power::from_watt(400.0); // 400 W
    let velocity = Velocity::from_m_per_sec(16.0); // 16 m/s
    let force: Force = power / velocity; // 25 N
    println!("   400 W ÷ 16 m/s = {} N", force.as_newton());
    println!("   验证: {} N × 16 m/s = {} W\n", force.as_newton(), (force * velocity).as_watt());

    // 7. 角速度 ÷ 角加速度 = 时间
    println!("7. 角速度 ÷ 角加速度 = 时间");
    let omega = AngularVelocity::from_rad_per_second(12.0); // 12 rad/s
    let alpha = AngularAcceleration::from_rad_per_second2(3.0); // 3 rad/s²
    let time = omega / alpha; // 4 s
    println!("   12 rad/s ÷ 3 rad/s² = {} s", time.as_secs_f64());
    println!("   验证: 3 rad/s² × {} s = {} rad/s\n", time.as_secs_f64(), (alpha * time).as_rad_per_second());

    // 实际应用示例
    println!("=== 实际应用示例 ===");
    
    // 汽车制动距离计算
    println!("\n汽车制动距离计算:");
    let initial_velocity = Velocity::from_km_per_h(60.0); // 60 km/h
    let deceleration = Acceleration::from_m_per_s2(5.0); // 5 m/s²
    let braking_time = initial_velocity / deceleration; // 制动时间
    let braking_distance = initial_velocity * braking_time; // 制动距离
    println!("   初始速度: {} km/h", initial_velocity.as_km_per_h());
    println!("   减速度: {} m/s²", deceleration.as_m_per_s2());
    println!("   制动时间: {:.2} s", braking_time.as_secs_f64());
    println!("   制动距离: {:.2} m", braking_distance.as_m());

    // 电机功率计算
    println!("\n电机功率计算:");
    let torque = Torque::from_nm(100.0); // 100 N·m
    let angular_velocity = AngularVelocity::from_rad_per_second(50.0); // 50 rad/s
    let power: Power = torque * angular_velocity; // 5000 W
    println!("   扭矩: {} N·m", torque.as_nm());
    println!("   角速度: {} rad/s", angular_velocity.as_rad_per_second());
    println!("   功率: {:.1} W ({:.2} kW)", power.as_watt(), power.as_kilo_watt());

    // 杠杆原理验证
    println!("\n杠杆原理验证:");
    let force = Force::from_newton(20.0); // 20 N
    let lever_arm = Distance::from_m(3.0); // 3 m
    let energy = force * lever_arm; // 60 J
    let calculated_force: Force = energy / lever_arm; // 20 N
    println!("   力: {} N", force.as_newton());
    println!("   力臂: {} m", lever_arm.as_m());
    println!("   功: {} J", energy.as_joule());
    println!("   验证力: {} N", calculated_force.as_newton());

    println!("\n=== 示例完成 ===");
}
