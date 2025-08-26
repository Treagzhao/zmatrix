use zmatrix::physics::basic::{VectorQuantity, Distance, Velocity, Force, Torque, MagneticInduction, Vector3};

fn main() {
    println!("=== VectorQuantity Trait 验证示例 ===\n");

    // 验证支持向量的物理量
    println!("1. 验证支持向量的物理量:");
    
    // 创建一些支持向量的物理量
    let distance = Distance::from_m(10.0);
    let velocity = Velocity::from_m_per_sec(5.0);
    let force = Force::from_newton(20.0);
    let torque = Torque::from_nm(15.0);
    let magnetic_induction = MagneticInduction::from_tesla(0.5);

    println!("   Distance: {} m", distance.as_m());
    println!("   Velocity: {} m/s", velocity.as_m_per_sec());
    println!("   Force: {} N", force.as_newton());
    println!("   Torque: {} N·m", torque.as_nm());
    println!("   MagneticInduction: {} T", magnetic_induction.as_tesla());

    // 验证这些类型都实现了 VectorQuantity trait
    println!("\n2. 验证 VectorQuantity trait 实现:");
    
    // 这些函数调用会验证类型是否实现了 VectorQuantity trait
    check_vector_quantity(distance);
    check_vector_quantity(velocity);
    check_vector_quantity(force);
    check_vector_quantity(torque);
    check_vector_quantity(magnetic_induction);

    println!("   ✓ 所有物理量都正确实现了 VectorQuantity trait");

    // 创建 Vector3 实例来验证向量操作
    println!("\n3. 验证 Vector3 操作:");
    
    let distance_vector = Vector3::new(
        Distance::from_m(1.0),
        Distance::from_m(2.0),
        Distance::from_m(3.0)
    );
    
    let velocity_vector = Vector3::new(
        Velocity::from_m_per_sec(4.0),
        Velocity::from_m_per_sec(5.0),
        Velocity::from_m_per_sec(6.0)
    );

    println!("   Distance Vector: ({}, {}, {}) m", 
             distance_vector.x.as_m(), 
             distance_vector.y.as_m(), 
             distance_vector.z.as_m());
    
    println!("   Velocity Vector: ({}, {}, {}) m/s", 
             velocity_vector.x.as_m_per_sec(), 
             velocity_vector.y.as_m_per_sec(), 
             velocity_vector.z.as_m_per_sec());

    // 验证向量运算 - 只能对相同类型的向量进行运算
    let sum_distance = distance_vector + distance_vector;
    println!("   Sum Distance Vector: ({}, {}, {}) m", 
             sum_distance.x.as_m(), 
             sum_distance.y.as_m(), 
             sum_distance.z.as_m());

    println!("\n=== 验证完成 ===");
}

// 这个函数要求参数实现 VectorQuantity trait
fn check_vector_quantity<T: VectorQuantity>(_value: T) {
    // 这个函数体是空的，但类型约束确保了 T 实现了 VectorQuantity
}
