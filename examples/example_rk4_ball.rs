/// RK4 演示：水平抛球，预测 10 秒后的位置
/// 
/// 解析解：x = vx*t, y = -0.5*g*t²
/// 用 RK4 数值解来逼近，验证正确性

const G: f64 = 9.80665; // 重力加速度 m/s²

/// 状态导数函数：给定当前状态 [x, y, vx, vy]，返回 [dx/dt, dy/dt, dvx/dt, dvy/dt]
fn derivative(state: &[f64; 4]) -> [f64; 4] {
    let vx = state[2];
    let vy = state[3];
    // ax = 0 (无空气阻力), ay = -g
    [vx, vy, 0.0, -G]
}

/// 一步 RK4 推进
fn rk4_step(state: [f64; 4], dt: f64) -> [f64; 4] {
    // k1: 起点处的斜率
    let k1 = derivative(&state);

    // k2: 用 k1 走半步后的斜率
    let s2 = [
        state[0] + 0.5 * dt * k1[0],
        state[1] + 0.5 * dt * k1[1],
        state[2] + 0.5 * dt * k1[2],
        state[3] + 0.5 * dt * k1[3],
    ];
    let k2 = derivative(&s2);

    // k3: 用 k2 走半步后的斜率（理论上更准）
    let s3 = [
        state[0] + 0.5 * dt * k2[0],
        state[1] + 0.5 * dt * k2[1],
        state[2] + 0.5 * dt * k2[2],
        state[3] + 0.5 * dt * k2[3],
    ];
    let k3 = derivative(&s3);

    // k4: 用 k3 走一整步后的斜率
    let s4 = [
        state[0] + dt * k3[0],
        state[1] + dt * k3[1],
        state[2] + dt * k3[2],
        state[3] + dt * k3[3],
    ];
    let k4 = derivative(&s4);

    // 加权平均：k1:k2:k3:k4 = 1:2:2:1
    [
        state[0] + dt / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
        state[1] + dt / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
        state[2] + dt / 6.0 * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]),
        state[3] + dt / 6.0 * (k1[3] + 2.0 * k2[3] + 2.0 * k3[3] + k4[3]),
    ]
}

fn main() {
    let vx = 10.0; // 水平初速 10 m/s
    let vy = 0.0;  // 竖直初速 0
    let mut state = [0.0, 0.0, vx, vy]; // [x, y, vx, vy]，从原点出发

    let total_time = 10.0;
    let dt = 0.1; // 步长 0.1s，共 100 步

    let steps = (total_time / dt) as usize;
    for _ in 0..steps {
        state = rk4_step(state, dt);
    }

    // 解析解（对比用）
    let exact_x = vx * total_time;
    let exact_y = -0.5 * G * total_time * total_time;

    println!("=== RK4 水平抛球 ===");
    println!("初速: vx = {} m/s, vy = {} m/s", vx, vy);
    println!("时间: {} s, 步长: {} s, 步数: {}", total_time, dt, steps);
    println!();
    println!("RK4 数值解:");
    println!("  x = {:.6} m, y = {:.6} m", state[0], state[1]);
    println!();
    println!("解析解:");
    println!("  x = {:.6} m, y = {:.6} m", exact_x, exact_y);
    println!();
    println!("误差:");
    println!("  Δx = {:.2e} m", (state[0] - exact_x).abs());
    println!("  Δy = {:.2e} m", (state[1] - exact_y).abs());
}
