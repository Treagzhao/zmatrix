use zmatrix::physics::basic::{Energy, Distance, Mass, Velocity, Acceleration, PhysicalQuantity};

fn main() {
    println!("=== 能量物理量示例 ===\n");

    // 1. 创建不同单位的能量
    println!("1. 创建不同单位的能量:");
    let e1 = Energy::from_joule(1.0); // 1 J
    let e2 = Energy::from_mill_joule(1000.0); // 1000 mJ = 1 J
    let e3 = Energy::from_micro_joule(1e6); // 1e6 μJ = 1 J
    let e4 = Energy::from_nano_joule(1e9); // 1e9 nJ = 1 J
    let e5 = Energy::from_kilo_joule(1e-3); // 1e-3 kJ = 1 J
    let e6 = Energy::from_mega_joule(1e-6); // 1e-6 MJ = 1 J
    let e7 = Energy::from_electron_volt(1.0 / 1.602176634e-19); // 1 J
    let e8 = Energy::from_kilo_electron_volt(1.0 / 1.602176634e-16); // 1 J
    let e9 = Energy::from_mega_electron_volt(1.0 / 1.602176634e-13); // 1 J

    println!("   e1 = {} J", e1.as_joule());
    println!("   e2 = {} mJ = {} J", e2.as_mill_joule(), e2.as_joule());
    println!("   e3 = {} μJ = {} J", e3.as_micro_joule(), e3.as_joule());
    println!("   e4 = {} nJ = {} J", e4.as_nano_joule(), e4.as_joule());
    println!("   e5 = {} kJ = {} J", e5.as_kilo_joule(), e5.as_joule());
    println!("   e6 = {} MJ = {} J", e6.as_mega_joule(), e6.as_joule());
    println!("   e7 = {} eV = {} J", e7.as_electron_volt(), e7.as_joule());
    println!("   e8 = {} keV = {} J", e8.as_kilo_electron_volt(), e8.as_joule());
    println!("   e9 = {} MeV = {} J", e9.as_mega_electron_volt(), e9.as_joule());

    // 2. 单位转换
    println!("\n2. 单位转换:");
    println!("   1 J = {} mJ", e1.as_mill_joule());
    println!("   1 J = {} μJ", e1.as_micro_joule());
    println!("   1 J = {} nJ", e1.as_nano_joule());
    println!("   1 J = {} kJ", e1.as_kilo_joule());
    println!("   1 J = {} MJ", e1.as_mega_joule());
    println!("   1 J = {} eV", e1.as_electron_volt());
    println!("   1 J = {} keV", e1.as_kilo_electron_volt());
    println!("   1 J = {} MeV", e1.as_mega_electron_volt());

    // 3. 能量运算
    println!("\n3. 能量运算:");
    let e_sum = e1 + e2; // 1 J + 1 J = 2 J
    let e_diff = e_sum - e1; // 2 J - 1 J = 1 J
    let e_scale = e1 * 3.0; // 1 J * 3 = 3 J
    let e_div = e_scale / 2.0; // 3 J / 2 = 1.5 J

    println!("   e1 + e2 = {} J", e_sum.as_joule());
    println!("   e_sum - e1 = {} J", e_diff.as_joule());
    println!("   e1 * 3 = {} J", e_scale.as_joule());
    println!("   e_scale / 2 = {} J", e_div.as_joule());

    // 4. 不同单位之间的运算
    println!("\n4. 不同单位之间的运算:");
    let e_micro = Energy::from_micro_joule(1e6); // 1 J
    let e_mill = Energy::from_mill_joule(1000.0); // 1 J
    let e_result = e_micro + e_mill; // 1 J + 1 J = 2 J
    println!("   {} μJ + {} mJ = {} J", 
             e_micro.as_micro_joule(), 
             e_mill.as_mill_joule(), 
             e_result.as_joule());

    // 5. 标量与能量的运算
    println!("\n5. 标量与能量的运算:");
    let e_scaled = 5.0 * e1; // 5 × 1 J = 5 J
    let e_divided = 10.0 / e1; // 10 / 1 J = 10 J
    println!("   5.0 * {} J = {} J", e1.as_joule(), e_scaled.as_joule());
    println!("   10.0 / {} J = {} J", e1.as_joule(), e_divided.as_joule());

    // 6. 零值检查
    println!("\n6. 零值检查:");
    let e_zero = Energy::from_joule(0.0);
    let e_nonzero = Energy::from_joule(1.0);
    println!("   e_zero.is_zero() = {}", e_zero.is_zero());
    println!("   e_nonzero.is_zero() = {}", e_nonzero.is_zero());

    // 7. 实际应用示例
    println!("\n7. 实际应用示例:");
    
    // 动能计算
    let mass = Mass::from_kg(2.0); // 2 kg
    let velocity = Velocity::from_m_per_sec(3.0); // 3 m/s
    let kinetic_energy = Energy::from_joule(0.5 * mass.as_kg() * velocity.as_m_per_sec().powi(2));
    println!("   质量: {} kg", mass.as_kg());
    println!("   速度: {} m/s", velocity.as_m_per_sec());
    println!("   动能: {} J", kinetic_energy.as_joule());

    // 势能计算
    let mass = Mass::from_kg(1.0); // 1 kg
    let height = Distance::from_m(10.0); // 10 m
    let g = 9.81; // 重力加速度 m/s²
    let potential_energy = Energy::from_joule(mass.as_kg() * g * height.as_m());
    println!("   质量: {} kg", mass.as_kg());
    println!("   高度: {} m", height.as_m());
    println!("   势能: {} J", potential_energy.as_joule());

    // 电子伏特应用
    let photon_energy = Energy::from_electron_volt(2.5); // 2.5 eV
    println!("   光子能量: {} eV = {} J", 
             photon_energy.as_electron_volt(), 
             photon_energy.as_joule());

    let nuclear_energy = Energy::from_mega_electron_volt(1.0); // 1 MeV
    println!("   核能: {} MeV = {} J", 
             nuclear_energy.as_mega_electron_volt(), 
             nuclear_energy.as_joule());

    println!("\n=== 示例完成 ===");
}
