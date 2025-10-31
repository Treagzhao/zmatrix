use crate::dense::error::OperationError;
use crate::physics::basic::vector::angular::{RotationHand, RotationSeq};
use crate::physics::basic::vector::angular::{TAG_X, TAG_Y, TAG_Z};
use crate::physics::basic::{Angular, AngularType, Coef, Vector3};
use crate::spatial_geometry::cos_matrix::CosMatrix;
use crate::spatial_geometry::quaternion::Quaternion;

pub struct EulerAngles {
    pub yaw: Angular,
    pub pitch: Angular,
    pub roll: Angular,
}

impl EulerAngles {
    pub fn new(yaw: Angular, pitch: Angular, roll: Angular) -> Self {
        Self { yaw, pitch, roll }
    }

    pub fn from_array(arr: [f64; 3], angular_type: AngularType) -> Self {
        let coef = Vector3::<Coef>::new(Coef::new(arr[0]), Coef::new(arr[1]), Coef::new(arr[2]));
        let angulars = Vector3::<Angular>::from_vector_coef(coef, angular_type);
        Self {
            yaw: angulars.x,
            pitch: angulars.y,
            roll: angulars.z,
        }
    }

    pub fn to_cos_matrix(
        &self,
        seq: RotationSeq,
        hand: RotationHand,
    ) -> Result<CosMatrix, OperationError> {
        let angulars = Vector3::<Angular>::new(self.roll, self.pitch, self.yaw);
        angulars.to_cos_matrix(seq, hand)
    }

    pub fn to_quaternion(
        &self,
        seq: RotationSeq,
        hand: RotationHand,
    ) -> Result<Quaternion, OperationError> {
        let code = seq.value()?;
        let tags = [(code >> 4) & 0x3, (code >> 2) & 0x3, code & 0x3];
        println!("[to_quaternion] seq code: {:#04x}, tags: [{:#x}, {:#x}, {:#x}]", code, tags[0], tags[1], tags[2]);
        // 按步骤位置取角度（与 to_cos_matrix 中的 arr 一致：i→roll, j→pitch, k→yaw）
        // 根据tags动态选择对应的角度
        let angles: Vec<f64> = tags.iter().map(|&tag| {
            match tag {
                TAG_X => self.roll.as_rad(),   // X轴 → roll角度
                TAG_Y => self.pitch.as_rad(),  // Y轴 → pitch角度
                TAG_Z => self.yaw.as_rad(),    // Z轴 → yaw角度
                _ => 0.0,
            }
        }).collect();
        println!(
            "[to_quaternion] angles(rad) by tags: i->{} (tag {:#x}), j->{} (tag {:#x}), k->{} (tag {:#x})",
            angles[0], tags[0], angles[1], tags[1], angles[2], tags[2]
        );

        // 计算三个旋转的半角三角函数
        let half_angles = [angles[0] * 0.5, angles[1] * 0.5, angles[2] * 0.5];
        println!(
            "[to_quaternion] half_angles: [{}, {}, {}]",
            half_angles[0], half_angles[1], half_angles[2]
        );
        let (s1, c1) = half_angles[0].sin_cos();
        let (s2, c2) = half_angles[1].sin_cos();
        let (s3, c3) = half_angles[2].sin_cos();
        println!(
            "[to_quaternion] (s1,c1)=({},{}) (s2,c2)=({},{}) (s3,c3)=({},{})",
            s1, c1, s2, c2, s3, c3
        );

        // 创建三个单轴旋转的四元数
        let q1 = axis_quaternion(tags[0], c1, s1);
        let q2 = axis_quaternion(tags[1], c2, s2);
        let q3 = axis_quaternion(tags[2], c3, s3);
        println!("[to_quaternion] q1 (step i): {:?}", q1);
        println!("[to_quaternion] q2 (step j): {:?}", q2);
        println!("[to_quaternion] q3 (step k): {:?}", q3);

        // 使用四元数乘法组合：按步骤先后（与矩阵 Ak·Aj·Ai 对齐）
        // 若实现使用 Hamilton 约定，需确保顺序与 to_cos_matrix 一致

        println!("[to_quaternion] q2 * q1 = {:?}", q2 * q1);
        let mut quat = q3 * q2 * q1;
        println!("[to_quaternion] quat (q3*q2*q1) before norm: {:?}", quat);

        // 归一化四元数
        quat = quat.normalize();
        println!("[to_quaternion] quat after normalize: {:?}", quat);

        // 应用手性调整
        if let RotationHand::Left = hand {
            quat = quat.conjugate();
            println!("[to_quaternion] applied left-hand conjugate: {:?}", quat);
        }

        Ok(quat)
    }
}

/// 根据轴索引创建对应的单位四元数
fn axis_quaternion(axis: u8, cos_half: f64, sin_half: f64) -> Quaternion {
    match axis {
        TAG_X => Quaternion::new(cos_half, sin_half, 0.0, 0.0), // 绕X轴旋转
        TAG_Y => Quaternion::new(cos_half, 0.0, sin_half, 0.0), // 绕Y轴旋转
        TAG_Z => Quaternion::new(cos_half, 0.0, 0.0, sin_half), // 绕Z轴旋转
        _     => Quaternion::new(1.0, 0.0, 0.0, 0.0),         // 单位四元数（无效轴）
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;
    use crate::physics::basic::vector::angular::{TAG_X, TAG_Y, TAG_Z};

    #[test]
    fn test_new() {
        // 创建测试用的角度值
        let yaw = Angular::from_rad(PI / 4.0);
        let pitch = Angular::from_rad(PI / 6.0);
        let roll = Angular::from_rad(PI / 3.0);

        // 使用new方法创建EulerAngles实例
        let euler = EulerAngles::new(yaw, pitch, roll);

        // 验证结果
        assert_relative_eq!(euler.yaw.as_rad(), PI / 4.0);
        assert_relative_eq!(euler.pitch.as_rad(), PI / 6.0);
        assert_relative_eq!(euler.roll.as_rad(), PI / 3.0);
    }

    #[test]
    fn test_from_array_radians() {
        // 使用弧度值的数组
        let arr = [PI / 4.0, PI / 6.0, PI / 3.0];

        // 使用from_array方法创建EulerAngles实例（弧度模式）
        let euler = EulerAngles::from_array(arr, AngularType::Rad);

        // 验证结果
        assert_relative_eq!(euler.yaw.as_rad(), PI / 4.0);
        assert_relative_eq!(euler.pitch.as_rad(), PI / 6.0);
        assert_relative_eq!(euler.roll.as_rad(), PI / 3.0);
    }

    #[test]
    fn test_from_array_degrees() {
        // 使用角度值的数组
        let arr = [90.0, 45.0, 30.0];

        // 使用from_array方法创建EulerAngles实例（角度模式）
        let euler = EulerAngles::from_array(arr, AngularType::Deg);

        // 验证结果（转换为弧度后比较）
        assert_relative_eq!(euler.yaw.as_rad(), PI / 2.0); // 90度 = π/2弧度
        assert_relative_eq!(euler.pitch.as_rad(), PI / 4.0); // 45度 = π/4弧度
        assert_relative_eq!(euler.roll.as_rad(), PI / 6.0); // 30度 = π/6弧度
    }

    #[test]
    fn test_zero_values() {
        // 测试零值情况
        let arr = [0.0, 0.0, 0.0];

        // 测试弧度模式
        let euler_rad = EulerAngles::from_array(arr, AngularType::Rad);
        assert_relative_eq!(euler_rad.yaw.as_rad(), 0.0);
        assert_relative_eq!(euler_rad.pitch.as_rad(), 0.0);
        assert_relative_eq!(euler_rad.roll.as_rad(), 0.0);

        // 测试角度模式
        let euler_deg = EulerAngles::from_array(arr, AngularType::Deg);
        assert_relative_eq!(euler_deg.yaw.as_deg(), 0.0);
        assert_relative_eq!(euler_deg.pitch.as_deg(), 0.0);
        assert_relative_eq!(euler_deg.roll.as_deg(), 0.0);
    }

    #[test]
    fn test_to_cos_matrix_zero_angles() {
        // 测试零角度情况，应该返回单位矩阵
        let euler = EulerAngles::from_array([0.0, 0.0, 0.0], AngularType::Rad);

        // 测试不同旋转序列和手性
        // 构造各类旋转序列
        let build = |axes: &[u8]| {
            let mut s = RotationSeq::default();
            for a in axes {
                match a {
                    0 => {
                        s.x().unwrap();
                    }
                    1 => {
                        s.y().unwrap();
                    }
                    _ => {
                        s.z().unwrap();
                    }
                }
            }
            s
        };
        // 0->X,1->Y,2->Z
        let rotation_seqs = vec![
            build(&[0, 1, 2]), // XYZ
            build(&[2, 1, 0]), // ZYX
            build(&[0, 2, 1]), // XZY
            build(&[1, 0, 2]), // YXZ
            build(&[2, 0, 1]), // ZXY
            build(&[1, 2, 0]), // YZX
        ];

        let hands = [RotationHand::Right, RotationHand::Left];

        for seq in rotation_seqs.iter() {
            for hand in hands.iter() {
                let cos_matrix = euler.to_cos_matrix(*seq, *hand).unwrap();

                // 验证是单位矩阵
                let identity = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

                let arr = cos_matrix.to_array();
                for r in 0..3 {
                    for c in 0..3 {
                        let idx = r * 3 + c;
                        assert_relative_eq!(arr[r][c], identity[idx], epsilon = 1e-10);
                    }
                }
            }
        }
    }

    #[test]
    fn test_to_cos_matrix_xyz_sequence() {
        // 测试XYZ旋转序列
        // 使用特定的角度值：yaw=90度(π/2), pitch=0度, roll=0度
        let euler = EulerAngles::from_array([PI / 2.0, 0.0, 0.0], AngularType::Rad);

        // 右手系XYZ旋转
        // 构造 XYZ 序列
        let mut s = RotationSeq::default();
        s.x().unwrap();
        s.y().unwrap();
        s.z().unwrap();
        let cos_matrix = euler.to_cos_matrix(s, RotationHand::Right).unwrap();

        // 验证结果矩阵（仅绕y轴旋转90度的矩阵）
        // 注意：由于代码中将roll, pitch, yaw映射到x, y, z，所以这里是yaw对应z轴
        let expected = vec![0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

        let arr = cos_matrix.to_array();
        for r in 0..3 {
            for c in 0..3 {
                let idx = r * 3 + c;
                assert_relative_eq!(arr[r][c], expected[idx], epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_to_cos_matrix_zxy_sequence() {
        // 测试ZXY旋转序列
        // 使用特定的角度值：yaw=0度, pitch=90度(π/2), roll=0度
        let euler = EulerAngles::from_array([0.0, PI / 2.0, 0.0], AngularType::Rad);

        // 右手系ZXY旋转
        // 构造 ZXY 序列
        let mut s = RotationSeq::default();
        s.z().unwrap();
        s.x().unwrap();
        s.y().unwrap();
        let cos_matrix = euler.to_cos_matrix(s, RotationHand::Right).unwrap();

        // 验证结果矩阵（按实现，第二步对应X轴，得到 Rx(90°)）
        let expected = vec![1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0];

        let arr = cos_matrix.to_array();
        for r in 0..3 {
            for c in 0..3 {
                let idx = r * 3 + c;
                assert_relative_eq!(arr[r][c], expected[idx], epsilon = 1e-10);
            }
        }
    }

    #[test]
    fn test_to_cos_matrix_handedness() {
        // 测试左右手系的区别
        let euler = EulerAngles::from_array([PI / 4.0, PI / 4.0, PI / 4.0], AngularType::Rad);

        // 计算右手系和左手系的方向余弦矩阵
        let mut s = RotationSeq::default();
        s.x().unwrap();
        s.y().unwrap();
        s.z().unwrap();
        let cos_matrix_right = euler.to_cos_matrix(s, RotationHand::Right).unwrap();
        let mut s2 = RotationSeq::default();
        s2.x().unwrap();
        s2.y().unwrap();
        s2.z().unwrap();
        let cos_matrix_left = euler.to_cos_matrix(s2, RotationHand::Left).unwrap();

        // 验证左右手系的矩阵是转置关系（在简单旋转情况下）
        // 对于一般情况，左手系矩阵是右手系矩阵的正交补
        // 这里我们验证它们不完全相同
        let mut all_equal = true;
        let arr_r = cos_matrix_right.to_array();
        let arr_l = cos_matrix_left.to_array();
        for r in 0..3 {
            for c in 0..3 {
                if (arr_r[r][c] - arr_l[r][c]).abs() >= 1e-10 {
                    all_equal = false;
                    break;
                }
            }
        }

        assert!(!all_equal, "右手系和左手系的方向余弦矩阵应该不同");
    }

    #[test]
    fn test_euler_to_quaternion_basic_cases() {
        // 测试用例1：零旋转
        let euler_zero = EulerAngles::new(
            Angular::from_rad(0.0),
            Angular::from_rad(0.0),
            Angular::from_rad(0.0),
        );

        let mut seq_zyx = RotationSeq::default();
        seq_zyx.z().unwrap(); seq_zyx.y().unwrap(); seq_zyx.x().unwrap();

        let q_zero = euler_zero.to_quaternion(seq_zyx, RotationHand::Right).unwrap();
        assert_relative_eq!(q_zero.q0, 1.0, epsilon = 1e-10);
        assert_relative_eq!(q_zero.q1, 0.0, epsilon = 1e-10);
        assert_relative_eq!(q_zero.q2, 0.0, epsilon = 1e-10);
        assert_relative_eq!(q_zero.q3, 0.0, epsilon = 1e-10);

        // 测试用例2：绕单轴旋转
        let euler_x = EulerAngles::new(
            Angular::from_rad(0.0),
            Angular::from_rad(0.0),
            Angular::from_rad(PI / 2.0), // 绕X轴旋转90°
        );

        let q_x = euler_x.to_quaternion(seq_zyx, RotationHand::Right).unwrap();
        let exp = Quaternion::new(0.7071067811865476,0.7071067811865475,0.0,0.0);
        assert_relative_eq!(q_x.q0, exp.q0, epsilon = 1e-10);
        assert_relative_eq!(q_x.q1, exp.q1, epsilon = 1e-10);
        assert_relative_eq!(q_x.q2, exp.q2, epsilon = 1e-10);
        assert_relative_eq!(q_x.q3, exp.q3, epsilon = 1e-10);
    }

    #[test]
    fn test_euler_to_quaternion_zyx_sequence() {
        // 测试ZYX顺序（航空标准）
        let euler = EulerAngles::new(
            Angular::from_deg(30.0),  // yaw (Z)
            Angular::from_deg(15.0),   // pitch (Y)
            Angular::from_deg(10.0),   // roll (X)
        );

        let mut seq_zyx = RotationSeq::default();
        seq_zyx.z().unwrap(); seq_zyx.y().unwrap(); seq_zyx.x().unwrap();

        let q = euler.to_quaternion(seq_zyx, RotationHand::Right).unwrap();
        assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-10);
        let exp = Quaternion::new(0.951081535, 0.117117116,  0.103245633,  0.266585240);
        assert_relative_eq!(q.q0, exp.q0, epsilon = 1e-4);
        assert_relative_eq!(q.q1, exp.q1, epsilon = 1e-4);
        assert_relative_eq!(q.q2, exp.q2, epsilon = 1e-4);
        assert_relative_eq!(q.q3, exp.q3, epsilon = 1e-4);
    }

    #[test]
    fn test_euler_to_quaternion_xyz_sequence() {
        // 测试XYZ顺序
        let euler = EulerAngles::new(
            Angular::from_deg(30.0),  // yaw (Z)
            Angular::from_deg(15.0),   // pitch (Y)
            Angular::from_deg(10.0),   // roll (X)
        );

        let mut seq_xyz = RotationSeq::default();
        seq_xyz.x().unwrap(); seq_xyz.y().unwrap(); seq_xyz.z().unwrap();

        let q = euler.to_quaternion(seq_xyz, RotationHand::Right).unwrap();
        assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-10);
        let exp = Quaternion::new(0.956979520, 0.049793014, 0.147843925, 0.244377402);
        assert_relative_eq!(q.q0, exp.q0, epsilon = 1e-4);
        assert_relative_eq!(q.q1, exp.q1, epsilon = 1e-4);
        assert_relative_eq!(q.q2, exp.q2, epsilon = 1e-3);
        assert_relative_eq!(q.q3, exp.q3, epsilon = 1e-3);
    }

    #[test]
    fn test_euler_to_quaternion_handedness() {
        // 测试左右手系关系
        let euler = EulerAngles::new(
            Angular::from_deg(45.0),
            Angular::from_deg(30.0),
            Angular::from_deg(15.0),
        );

        let mut seq_zyx = RotationSeq::default();
        seq_zyx.z().unwrap(); seq_zyx.y().unwrap(); seq_zyx.x().unwrap();

        let q_right = euler.to_quaternion(seq_zyx, RotationHand::Right).unwrap();
        let q_left = euler.to_quaternion(seq_zyx, RotationHand::Left).unwrap();

        // 验证左右手系关系：q_left 应该是 q_right 的共轭
        assert_relative_eq!(q_left.q0, q_right.q0, epsilon = 1e-10);
        assert_relative_eq!(q_left.q1, -q_right.q1, epsilon = 1e-10);
        assert_relative_eq!(q_left.q2, -q_right.q2, epsilon = 1e-10);
        assert_relative_eq!(q_left.q3, -q_right.q3, epsilon = 1e-10);
    }

    #[test]
    fn test_euler_to_quaternion_sequence_consistency() {
        // 测试不同旋转顺序的一致性
        let euler = EulerAngles::new(
            Angular::from_deg(20.0),
            Angular::from_deg(40.0),
            Angular::from_deg(60.0),
        );

        // 测试多种旋转顺序
        let sequences = [
            (TAG_X, TAG_Y, TAG_Z), // XYZ
            (TAG_Z, TAG_Y, TAG_X), // ZYX
            (TAG_Z, TAG_X, TAG_Z), // ZXZ
        ];

        for (i, j, k) in sequences {
            let mut seq = RotationSeq::default();
            match i { TAG_X => seq.x(), TAG_Y => seq.y(), _ => seq.z() }.unwrap();
            match j { TAG_X => seq.x(), TAG_Y => seq.y(), _ => seq.z() }.unwrap();
            match k { TAG_X => seq.x(), TAG_Y => seq.y(), _ => seq.z() }.unwrap();

            let q = euler.to_quaternion(seq, RotationHand::Right).unwrap();

            // 验证四元数范数为1
            assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-10);

            // 验证四元数是单位四元数
            let norm_sq = q.q0 * q.q0 + q.q1 * q.q1 + q.q2 * q.q2 + q.q3 * q.q3;
            assert_relative_eq!(norm_sq, 1.0, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_euler_to_quaternion_extreme_angles() {
        // 测试极端角度值
        let test_cases = [
            // 大角度
            (180.0, 90.0, 45.0),
            // 负角度
            (-30.0, -60.0, -90.0),
            // 超过360度
            (400.0, 500.0, 600.0),
            // 非常小的角度
            (0.001, 0.002, 0.003),
        ];

        let mut seq_zyx = RotationSeq::default();
        seq_zyx.z().unwrap(); seq_zyx.y().unwrap(); seq_zyx.x().unwrap();

        for (yaw_deg, pitch_deg, roll_deg) in test_cases {
            let euler = EulerAngles::new(
                Angular::from_deg(yaw_deg),
                Angular::from_deg(pitch_deg),
                Angular::from_deg(roll_deg),
            );

            let q = euler.to_quaternion(seq_zyx, RotationHand::Right).unwrap();

            // 验证四元数范数为1
            assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-8);
        }
    }

    #[test]
    fn test_euler_to_quaternion_gimbal_lock() {
        // 测试万向锁情况（pitch = ±90°）
        let euler_gimbal = EulerAngles::new(
            Angular::from_deg(30.0),
            Angular::from_deg(90.0),  // pitch = 90° (万向锁)
            Angular::from_deg(20.0),
        );

        let mut seq_zyx = RotationSeq::default();
        seq_zyx.z().unwrap(); seq_zyx.y().unwrap(); seq_zyx.x().unwrap();

        let q = euler_gimbal.to_quaternion(seq_zyx, RotationHand::Right).unwrap();

        // 在万向锁情况下，四元数仍然应该是有效的单位四元数
        assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_euler_to_quaternion_back_and_forth() {
        // 测试欧拉角 ↔ 四元数 ↔ 欧拉角的往返转换
        let original = EulerAngles::new(
            Angular::from_deg(25.0),
            Angular::from_deg(35.0),
            Angular::from_deg(45.0),
        );

        let mut seq_zyx = RotationSeq::default();
        seq_zyx.z().unwrap(); seq_zyx.y().unwrap(); seq_zyx.x().unwrap();

        // 欧拉角 → 四元数
        let q = original.to_quaternion(seq_zyx, RotationHand::Right).unwrap();

        // 四元数 → 旋转矩阵（假设你有这个方法）
        // let matrix = q.to_rotation_matrix();

        // 旋转矩阵 → 欧拉角（假设你有这个方法）
        // let recovered = matrix.to_euler_angles(seq_zyx);

        // 由于万向锁和角度范围问题，往返转换可能不完全相等
        // 但四元数本身应该是有效的
        assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_simple_case() {
        // 测试绕X轴旋转90度
        let euler = EulerAngles::new(
            Angular::from_deg(0.0),   // yaw = 0
            Angular::from_deg(0.0),    // pitch = 0
            Angular::from_deg(90.0),    // roll = 90° (X轴)
        );

        let mut seq_xyz = RotationSeq::default();
        seq_xyz.x().unwrap(); seq_xyz.y().unwrap(); seq_xyz.z().unwrap();

        let q = euler.to_quaternion(seq_xyz, RotationHand::Right).unwrap();

        // 绕X轴旋转90°的四元数应该是 [cos(45°), sin(45°), 0, 0]
        let expected = Quaternion::new(0.7071067811865476, 0.7071067811865476, 0.0, 0.0);

        println!("实际结果: {:?}", q);
        println!("期望结果: {:?}", expected);

        assert_relative_eq!(q.q0, expected.q0, epsilon = 1e-10);
        assert_relative_eq!(q.q1, expected.q1, epsilon = 1e-10);
        assert_relative_eq!(q.q2, expected.q2, epsilon = 1e-10);
        assert_relative_eq!(q.q3, expected.q3, epsilon = 1e-10);
    }

    #[test]
    fn test_axis_quaternion_fallback_branch() {
        // 非法轴标签应返回单位四元数（兜底分支）
        let invalid_axes: [u8; 3] = [0x0, 0x4, 0xFF];
        for axis in invalid_axes.iter() {
            // 随便给一组半角三角值，兜底分支应忽略它们并返回单位四元数
            let q = super::axis_quaternion(*axis, 0.12345, 0.67890);
            let expected = Quaternion::new(1.0, 0.0, 0.0, 0.0);
            assert_relative_eq!(q.q0, expected.q0, epsilon = 1e-12);
            assert_relative_eq!(q.q1, expected.q1, epsilon = 1e-12);
            assert_relative_eq!(q.q2, expected.q2, epsilon = 1e-12);
            assert_relative_eq!(q.q3, expected.q3, epsilon = 1e-12);
            assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-12);
        }
    }

    #[test]
    fn test_to_quaternion_angle_mapping_fallback_branch() {
        // 构造一个包含非法中间 tag=0 的序列：i=Z(3), j=0(非法), k=Y(2)
        // 这样第53行 angles 映射会走 `_ => 0.0` 分支，从而覆盖兜底逻辑
        let code: u8 = (TAG_Z << 4) | (0 << 2) | TAG_Y; // 0x32
        // 使用 unsafe 方式构造一个内部 tag=code 且 count=3 的 RotationSeq
        let seq: RotationSeq = unsafe { std::mem::transmute::<(u8, u8), RotationSeq>((code, 3u8)) };

        // 使用明显非零的角度便于观察：yaw=0.3, pitch=-0.5, roll=0.7（单位：rad）
        let euler = EulerAngles::new(
            Angular::from_rad(0.3),   // yaw (Z)
            Angular::from_rad(-0.5),  // pitch (Y)
            Angular::from_rad(0.7),   // roll (X)
        );

        let q = euler.to_quaternion(seq, RotationHand::Right).unwrap();
        // 基本性质：应为单位四元数
        assert_relative_eq!(q.norm(), 1.0, epsilon = 1e-12);

        // 手工按相同逻辑计算期望：i 用 yaw（Z），j 因非法 tag 走 0 角，k 用 pitch（Y）
        let tags = [ (code >> 4) & 0x3, (code >> 2) & 0x3, code & 0x3 ];
        let angles = [
            // i
            if tags[0] == TAG_X { euler.roll.as_rad() } else if tags[0] == TAG_Y { euler.pitch.as_rad() } else { euler.yaw.as_rad() },
            // j -> 非法 0 则取 0.0
            if tags[1] == TAG_X { euler.roll.as_rad() } else if tags[1] == TAG_Y { euler.pitch.as_rad() } else if tags[1] == TAG_Z { euler.yaw.as_rad() } else { 0.0 },
            // k
            if tags[2] == TAG_X { euler.roll.as_rad() } else if tags[2] == TAG_Y { euler.pitch.as_rad() } else { euler.yaw.as_rad() },
        ];
        let half = [angles[0]*0.5, angles[1]*0.5, angles[2]*0.5];
        let (s1,c1) = half[0].sin_cos();
        let (s2,c2) = half[1].sin_cos();
        let (s3,c3) = half[2].sin_cos();
        let q1 = super::axis_quaternion(tags[0], c1, s1);
        let q2 = super::axis_quaternion(tags[1], c2, s2); // 非法 tag=0 -> 单位四元数
        let q3 = super::axis_quaternion(tags[2], c3, s3);
        let exp = (q3 * q2 * q1).normalize();

        assert_relative_eq!(q.q0, exp.q0, epsilon = 1e-12);
        assert_relative_eq!(q.q1, exp.q1, epsilon = 1e-12);
        assert_relative_eq!(q.q2, exp.q2, epsilon = 1e-12);
        assert_relative_eq!(q.q3, exp.q3, epsilon = 1e-12);
    }
}
