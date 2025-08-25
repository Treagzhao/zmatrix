use zmatrix::physics::basic::{VectorQuantity, Distance, Velocity, Force, Torque, MagneticInduction, Vector3, Mass, Energy, Power, Volume};

fn main() {
    println!("=== Vector3 类型约束验证示例 ===\n");

    // 1. 验证支持向量的物理量可以用于 Vector3
    println!("1. 支持向量的物理量可以用于 Vector3:");
    
    let distance_vector = Vector3::new(
        Distance::from_m(1.0),
        Distance::from_m(2.0),
        Distance::from_m(3.0)
    );
    println!("   ✓ Distance Vector: ({}, {}, {}) m", 
             distance_vector.x.as_m(), 
             distance_vector.y.as_m(), 
             distance_vector.z.as_m());

    let velocity_vector = Vector3::new(
        Velocity::from_m_per_sec(4.0),
        Velocity::from_m_per_sec(5.0),
        Velocity::from_m_per_sec(6.0)
    );
    println!("   ✓ Velocity Vector: ({}, {}, {}) m/s", 
             velocity_vector.x.as_m_per_sec(), 
             velocity_vector.y.as_m_per_sec(), 
             velocity_vector.z.as_m_per_sec());

    let force_vector = Vector3::new(
        Force::from_newton(10.0),
        Force::from_newton(20.0),
        Force::from_newton(30.0)
    );
    println!("   ✓ Force Vector: ({}, {}, {}) N", 
             force_vector.x.as_newton(), 
             force_vector.y.as_newton(), 
             force_vector.z.as_newton());

    // 2. 验证不支持向量的物理量无法用于 Vector3
    println!("\n2. 不支持向量的物理量无法用于 Vector3:");
    println!("   ✓ 以下代码会在编译时被拒绝:");
    println!("     let mass_vector = Vector3::new(");
    println!("         Mass::from_kg(1.0),");
    println!("         Mass::from_kg(2.0),");
    println!("         Mass::from_kg(3.0)");
    println!("     );");
    println!("\n   ✓ 以下代码会在编译时被拒绝:");
    println!("     let energy_vector = Vector3::new(");
    println!("         Energy::from_joule(1.0),");
    println!("         Energy::from_joule(2.0),");
    println!("         Energy::from_joule(3.0)");
    println!("     );");

    // 3. 列出所有支持向量的物理量
    println!("\n3. 支持向量的物理量列表:");
    println!("   ✓ Distance (距离/位移)");
    println!("   ✓ Velocity (速度)");
    println!("   ✓ Acceleration (加速度)");
    println!("   ✓ Angular (角度)");
    println!("   ✓ AngularVelocity (角速度)");
    println!("   ✓ AngularAcceleration (角加速度)");
    println!("   ✓ Coef (系数)");
    println!("   ✓ AngularMomentum (角动量)");
    println!("   ✓ Momentum (动量)");
    println!("   ✓ MagneticInduction (磁感应强度)");
    println!("   ✓ MagneticMoment (磁矩)");
    println!("   ✓ Torque (力矩)");
    println!("   ✓ Force (力)");
    println!("   ✓ MagneticAngularVelocity (磁角速度)");

    // 4. 列出不支持向量的物理量
    println!("\n4. 不支持向量的物理量列表:");
    println!("   ✗ Mass (质量)");
    println!("   ✗ Energy (能量)");
    println!("   ✗ Power (功率)");
    println!("   ✗ Volume (体积)");
    println!("   ✗ Area (面积)");
    println!("   ✗ Time (时间)");

    println!("\n=== 验证完成 ===");
    println!("Vector3 现在只能用于支持向量的物理量，这确保了类型安全！");
}
