use std::time::Duration;
use zmatrix::physics::basic::*;

fn main(){
    let distance = Distance::from_m(1001.0);
    println!("{:?}", distance.as_km());
    // 1.001 km
    println!("{:?}", distance.as_light_year());
    // 3.262606876811594e-11 light years

    let velocity = Velocity::from_m_per_sec(100.0);
    println!("{:?}", velocity.as_km_per_h());
    // 360.0 km/h
}

fn calculate() {
    let distance = Distance::from_m(1000.0);
    let time = Duration::from_secs(10);
    let velocity:Velocity = distance / time;
    let acc:Acceleration = velocity / time;

    let ang:Angular = Angular::from_deg(180.0);
    let time =  Duration::from_secs(10);
    let omg:AngularVelocity = ang / time;
    let angular_accel:AngularAcceleration = omg / time;

    let dis:Distance = Distance::from_m(1000.0);
    let area:Area = dis * dis;

}

fn exmples_vector3(){
    let pos_vector:Vector3<Distance> = Vector3::new(
        Distance::from_m(1.0),
        Distance::from_m(3.0),
        Distance::from_m(5.0)
    );
}