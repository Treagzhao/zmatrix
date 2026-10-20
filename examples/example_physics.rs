use std::time::Duration;
use zmatrix::physics::basic::*;

fn main(){
    let distance = Distance::from_m(1001.0);
    println!("{:?}", distance.as_km());
    // 1.001 km
    println!("{:?}", distance.as_light_year());
    // 1.05805783485864e-13 light years（1001 m ÷ 9.4607e15 m/ly）

    let velocity = Velocity::from_m_per_sec(100.0);
    println!("{:?}", velocity.as_km_per_h());
    // 360.0 km/h

    calculate();
    examples_vector3();
}

fn calculate() {
    let distance = Distance::from_m(1000.0);
    let time = Duration::from_secs(10);
    let velocity: Velocity = distance / time;
    let acc: Acceleration = velocity / time;
    println!("速度: {} m/s, 加速度: {} m/s²", velocity.as_m_per_sec(), acc.as_m_per_s2());

    let ang: Angular = Angular::from_deg(180.0);
    let time = Duration::from_secs(10);
    let omg: AngularVelocity = ang / time;
    let angular_accel: AngularAcceleration = omg / time;
    println!(
        "角速度: {} rad/s, 角加速度: {} rad/s²",
        omg.as_rad_per_second(),
        angular_accel.as_rad_per_second2()
    );

    let dis: Distance = Distance::from_m(1000.0);
    let area: Area = dis * dis;
    println!("面积: {} m²", area.as_m2());
}

fn examples_vector3(){
    let pos_vector: Vector3<Distance> = Vector3::new(
        Distance::from_m(1.0),
        Distance::from_m(3.0),
        Distance::from_m(5.0)
    );
    println!(
        "位置向量: ({}, {}, {}) m",
        pos_vector.x.as_m(),
        pos_vector.y.as_m(),
        pos_vector.z.as_m()
    );
}
