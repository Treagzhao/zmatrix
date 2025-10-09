use super::*;
use crate::constant::FLT64_ZERO;
use crate::dense::error::OperationError;
use crate::physics::basic::AngularType;
use crate::spatial_geometry::cos_matrix::CosMatrix;
use crate::spatial_geometry::quaternion::Quaternion;
#[derive(Default, Debug, Copy, Clone)]
pub struct RotationSeq {
    tag: u8,
    count: u8,
}

const TAG_X: u8 = 0x1;
const TAG_Y: u8 = 0x2;
const TAG_Z: u8 = 0x3;

#[derive(Debug, Copy, Clone)]
pub enum RotationHand {
    Right,
    Left,
}

impl RotationSeq {
    pub fn x(&mut self) -> Result<u8, OperationError> {
        self.set_tag(TAG_X)
    }

    pub fn y(&mut self) -> Result<u8, OperationError> {
        self.set_tag(TAG_Y)
    }

    pub fn z(&mut self) -> Result<u8, OperationError> {
        self.set_tag(TAG_Z)
    }

    fn set_tag(&mut self, v: u8) -> Result<u8, OperationError> {
        if self.count >= 3 {
            return Err(OperationError::new("RotationSeq:: count exceeds 3"));
        }
        self.tag = (self.tag << 2) | v;
        self.count += 1;
        Ok(self.count)
    }

    pub fn value(&self) -> Result<u8, OperationError> {
        if self.count != 3 {
            return Err(OperationError::new("RotationSeq:: value requires 3 axes"));
        }
        Ok(self.tag)
    }
}

impl Vector3<Angular> {
    pub fn to_f32_array(&self) -> [f32; 3] {
        let result: [f32; 3] = [
            self.x.as_rad() as f32,
            self.y.as_rad() as f32,
            self.z.as_rad() as f32,
        ];
        result
    }

    pub fn to_quaternion(&self) -> Quaternion {
        let norm = self.norm();
        if norm.as_rad() < FLT64_ZERO {
            return Quaternion::default();
        } else {
            let tmp = (norm * 0.5).sin() / norm.as_rad();
            let mut q = Quaternion::default();
            q.q0 = (norm * 0.5).cos();
            q.q1 = (self.x * tmp).as_rad();
            q.q2 = (self.y * tmp).as_rad();
            q.q3 = (self.z * tmp).as_rad();
            return q;
        }
    }

    pub fn sin(&self) -> Vector3<Coef> {
        let x = self.x.sin();
        let y = self.y.sin();
        let z = self.z.sin();
        Vector3 {
            x: Coef::new(x),
            y: Coef::new(y),
            z: Coef::new(z),
        }
    }

    pub fn cos(&self) -> Vector3<Coef> {
        let x = self.x.cos();
        let y = self.y.cos();
        let z = self.z.cos();
        Vector3 {
            x: Coef::new(x),
            y: Coef::new(y),
            z: Coef::new(z),
        }
    }

    pub fn mod_to_round(&self) -> Vector3<Angular> {
        let mut result: Vector3<Angular> = Vector3::new(
            self.x.mod_to_round(),
            self.y.mod_to_round(),
            self.z.mod_to_round(),
        );
        return result;
    }

    pub fn mod_to_round_half(&self) -> Vector3<Angular> {
        let mut result: Vector3<Angular> = Vector3::new(
            self.x.mod_to_round_half(),
            self.y.mod_to_round_half(),
            self.z.mod_to_round_half(),
        );
        return result;
    }

    pub fn to_vector3_coef(&self, angular_type: AngularType) -> Vector3<Coef> {
        match angular_type {
            AngularType::Rad => {
                let (x, y, z) = (self.x.as_rad(), self.y.as_rad(), self.z.as_rad());
                Vector3::<Coef>::from_array([x, y, z])
            }
            AngularType::Deg => {
                let (x, y, z) = (self.x.as_deg(), self.y.as_deg(), self.z.as_deg());
                Vector3::<Coef>::from_array([x, y, z])
            }
        }
    }

    pub fn from_vector_coef(coef: Vector3<Coef>, angular_type: AngularType) -> Vector3<Angular> {
        let x = Angular {
            v: coef.x.v,
            default_type: angular_type,
        };
        let y = Angular {
            v: coef.y.v,
            default_type: angular_type,
        };
        let z = Angular {
            v: coef.z.v,
            default_type: angular_type,
        };
        Vector3::new(x, y, z)
    }

    pub fn to_cos_matrix(&self, seq: RotationSeq, hand: RotationHand) -> Result<CosMatrix, OperationError> {
        // 解码序列：i(高2位)、j(中2位)、k(低2位)
        let code = seq.value()?;
        let tags = [ (code >> 4) & 0x3, (code >> 2) & 0x3, code & 0x3 ];

        // 三步角值（按调用顺序 i, j, k 对应 self.x, self.y, self.z）
        let arr = [self.x.as_rad(), self.y.as_rad(), self.z.as_rad()];

        // 分别构造 Ai, Aj, Ak
        let mut result: [CosMatrix; 3] = [CosMatrix::default(); 3];
        for t in 0..3 {
            result[t] = match tags[t] {
                TAG_X => rx_hand(arr[t], hand),
                TAG_Y => ry_hand(arr[t], hand),
                _     => rz_hand(arr[t], hand),
            };
        }

        // 组合：Ak · Aj · Ai
        let aj_ai = result[1].product(&result[0]);
        Ok(result[2].product(&aj_ai))
    }
}

// 基础旋转矩阵构造
fn rx_right(a: f64) -> CosMatrix { // 右手主动
    let (c, s) = (a.cos(), a.sin());
    CosMatrix::new([[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s,  c]])
}
fn ry_right(a: f64) -> CosMatrix { // 右手主动
    let (c, s) = (a.cos(), a.sin());
    CosMatrix::new([[ c, 0.0,  s], [0.0, 1.0, 0.0], [-s, 0.0,  c]])
}
fn rz_right(a: f64) -> CosMatrix { // 右手主动
    let (c, s) = (a.cos(), a.sin());
    CosMatrix::new([[ c, -s, 0.0], [ s,  c, 0.0], [0.0, 0.0, 1.0]])
}

fn rx_left(a: f64) -> CosMatrix { // 左手主动（等价右手被动）
    let (c, s) = (a.cos(), a.sin());
    CosMatrix::new([[1.0, 0.0, 0.0], [0.0, c,  s], [0.0, -s, c]])
}
fn ry_left(a: f64) -> CosMatrix { // 左手主动（等价右手被动）
    let (c, s) = (a.cos(), a.sin());
    CosMatrix::new([[ c, 0.0, -s], [0.0, 1.0, 0.0], [ s, 0.0,  c]])
}
fn rz_left(a: f64) -> CosMatrix { // 左手主动（等价右手被动）
    let (c, s) = (a.cos(), a.sin());
    CosMatrix::new([[ c,  s, 0.0], [-s,  c, 0.0], [0.0, 0.0, 1.0]])
}

fn rx_hand(a: f64, hand: RotationHand) -> CosMatrix {
    match hand { RotationHand::Right => rx_right(a), RotationHand::Left => rx_left(a) }
}
fn ry_hand(a: f64, hand: RotationHand) -> CosMatrix {
    match hand { RotationHand::Right => ry_right(a), RotationHand::Left => ry_left(a) }
}
fn rz_hand(a: f64, hand: RotationHand) -> CosMatrix {
    match hand { RotationHand::Right => rz_right(a), RotationHand::Left => rz_left(a) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::PI;

    fn build_and_value(seq: &[u8; 3]) -> u8 {
        let mut r = RotationSeq::default();
        for &axis in seq {
            match axis {
                TAG_X => {
                    r.x().unwrap();
                }
                TAG_Y => {
                    r.y().unwrap();
                }
                TAG_Z => {
                    r.z().unwrap();
                }
                _ => unreachable!(),
            }
        }
        r.value().unwrap()
    }

    #[test]
    #[should_panic]
    fn test_build_and_value_unreachable_branch_panics() {
        // 触发默认分支，覆盖第227行的 unreachable!()
        // 非法轴标记 0x4 将导致 panic
        let _ = build_and_value(&[0x4, TAG_X, TAG_Y]);
    }

    #[test]
    fn test_rotationseq_tait_bryan_sequences() {
        // XYZ
        assert_eq!(
            build_and_value(&[TAG_X, TAG_Y, TAG_Z]),
            (TAG_X << 4) | (TAG_Y << 2) | TAG_Z
        );
        // XZY
        assert_eq!(
            build_and_value(&[TAG_X, TAG_Z, TAG_Y]),
            (TAG_X << 4) | (TAG_Z << 2) | TAG_Y
        );
        // YXZ
        assert_eq!(
            build_and_value(&[TAG_Y, TAG_X, TAG_Z]),
            (TAG_Y << 4) | (TAG_X << 2) | TAG_Z
        );
        // YZX
        assert_eq!(
            build_and_value(&[TAG_Y, TAG_Z, TAG_X]),
            (TAG_Y << 4) | (TAG_Z << 2) | TAG_X
        );
        // ZXY
        assert_eq!(
            build_and_value(&[TAG_Z, TAG_X, TAG_Y]),
            (TAG_Z << 4) | (TAG_X << 2) | TAG_Y
        );
        // ZYX
        assert_eq!(
            build_and_value(&[TAG_Z, TAG_Y, TAG_X]),
            (TAG_Z << 4) | (TAG_Y << 2) | TAG_X
        );
    }



    #[test]
    fn test_rotationseq_proper_euler_sequences() {
        // ZXZ
        assert_eq!(
            build_and_value(&[TAG_Z, TAG_X, TAG_Z]),
            (TAG_Z << 4) | (TAG_X << 2) | TAG_Z
        );
        // ZYZ
        assert_eq!(
            build_and_value(&[TAG_Z, TAG_Y, TAG_Z]),
            (TAG_Z << 4) | (TAG_Y << 2) | TAG_Z
        );
        // XYX
        assert_eq!(
            build_and_value(&[TAG_X, TAG_Y, TAG_X]),
            (TAG_X << 4) | (TAG_Y << 2) | TAG_X
        );
        // XZX
        assert_eq!(
            build_and_value(&[TAG_X, TAG_Z, TAG_X]),
            (TAG_X << 4) | (TAG_Z << 2) | TAG_X
        );
        // YXY
        assert_eq!(
            build_and_value(&[TAG_Y, TAG_X, TAG_Y]),
            (TAG_Y << 4) | (TAG_X << 2) | TAG_Y
        );
        // YZY
        assert_eq!(
            build_and_value(&[TAG_Y, TAG_Z, TAG_Y]),
            (TAG_Y << 4) | (TAG_Z << 2) | TAG_Y
        );
    }

    #[test]
    fn test_rotationseq_value_error_when_incomplete() {
        let mut r = RotationSeq::default();
        // 仅两个轴，未满三次
        r.x().unwrap();
        r.y().unwrap();
        let err = r.value();
        assert!(err.is_err());
    }

    #[test]
    fn test_rotationseq_set_tag_error_when_exceeds() {
        let mut r = RotationSeq::default();
        // 三次正常
        r.x().unwrap();
        r.y().unwrap();
        r.z().unwrap();
        // 第四次应报错
        let err = r.x();
        assert!(err.is_err());
    }
    #[test]
    fn test_to_f32_array() {
        let vector = Vector3 {
            x: Angular::from_rad(1.0),
            y: Angular::from_rad(2.0),
            z: Angular::from_rad(3.0),
        };
        let result = vector.to_f32_array();
        assert_eq!(result, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_to_quaternion() {
        let vector: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(-0.002357),
            y: Angular::from_rad(0.0144422),
            z: Angular::from_rad(0.0065734),
        };
        let q = vector.to_quaternion();
        assert_relative_eq!(q.q0, 0.9999678, epsilon = 1e-7);
        assert_relative_eq!(q.q1, -0.0011784874, epsilon = 1e-7);
        assert_relative_eq!(q.q2, 0.007221023, epsilon = 1e-7);
        assert_relative_eq!(q.q3, 0.0032866648, epsilon = 1e-7);

        let vecotr: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(0.0),
            y: Angular::from_rad(0.0),
            z: Angular::from_rad(0.0),
        };
        let q = vecotr.to_quaternion();
        assert_relative_eq!(q.q0, 1.0, epsilon = 1e-7);
        assert_relative_eq!(q.q1, 0.0, epsilon = 1e-7);
        assert_relative_eq!(q.q2, 0.0, epsilon = 1e-7);
        assert_relative_eq!(q.q3, 0.0, epsilon = 1e-7);
    }

    #[test]
    fn test_sin() {
        let vector: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(PI / 6.0),
            y: Angular::from_rad(PI / 6.0),
            z: Angular::from_rad(PI / 6.0),
        };
        let result = vector.sin();
        assert_relative_eq!(result.x.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.y.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.z.get_value(), 0.5, epsilon = 1e-7);
    }

    #[test]
    fn test_cos() {
        let vector: Vector3<Angular> = Vector3 {
            x: Angular::from_rad(PI / 3.0),
            y: Angular::from_rad(PI / 3.0),
            z: Angular::from_rad(PI / 3.0),
        };
        let result = vector.cos();
        assert_relative_eq!(result.x.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.y.get_value(), 0.5, epsilon = 1e-7);
        assert_relative_eq!(result.z.get_value(), 0.5, epsilon = 1e-7);
    }

    // Test data generators
    fn generate_test_vectors() -> Vec<Vector3<Angular>> {
        vec![
            // Basic cases
            Vector3::new(
                Angular::from_rad(0.0),
                Angular::from_rad(0.0),
                Angular::from_rad(0.0),
            ),
            Vector3::new(
                Angular::from_rad(PI),
                Angular::from_rad(PI),
                Angular::from_rad(PI),
            ),
            // Edge cases
            Vector3::new(
                Angular::from_rad(2.0 * PI),
                Angular::from_rad(2.0 * PI),
                Angular::from_rad(2.0 * PI),
            ),
            Vector3::new(
                Angular::from_rad(PI / 2.0),
                Angular::from_rad(3.0 * PI / 2.0),
                Angular::from_rad(PI / 4.0),
            ),
            // Extreme values
            Vector3::new(
                Angular::from_rad(1000.0 * PI),
                Angular::from_rad(-500.0 * PI),
                Angular::from_rad(123.456 * PI),
            ),
        ]
    }

    #[test]
    fn test_mod_to_round() {
        // Test normal rounding behavior
        let vectors = generate_test_vectors();
        for vector in vectors {
            let result = vector.mod_to_round();

            // Check each component is properly rounded
            assert_relative_eq!(
                result.x.as_rad(),
                vector.x.mod_to_round().as_rad(),
                epsilon = 1e-7
            );
            assert_relative_eq!(
                result.y.as_rad(),
                vector.y.mod_to_round().as_rad(),
                epsilon = 1e-7
            );
            assert_relative_eq!(
                result.z.as_rad(),
                vector.z.mod_to_round().as_rad(),
                epsilon = 1e-7
            );

            // Verify results are within expected range [0, 2π)
            assert!(result.x.as_rad() >= 0.0 && result.x.as_rad() < 2.0 * PI);
            assert!(result.y.as_rad() >= 0.0 && result.y.as_rad() < 2.0 * PI);
            assert!(result.z.as_rad() >= 0.0 && result.z.as_rad() < 2.0 * PI);
        }
    }

    #[test]
    fn test_mod_to_round_half() {
        // Test rounding to nearest half interval behavior
        let vectors = generate_test_vectors();
        for vector in vectors {
            let result = vector.mod_to_round_half();

            // Check each component is properly rounded to half intervals
            assert_relative_eq!(
                result.x.as_rad(),
                vector.x.mod_to_round_half().as_rad(),
                epsilon = 1e-7
            );
            assert_relative_eq!(
                result.y.as_rad(),
                vector.y.mod_to_round_half().as_rad(),
                epsilon = 1e-7
            );
            assert_relative_eq!(
                result.z.as_rad(),
                vector.z.mod_to_round_half().as_rad(),
                epsilon = 1e-7
            );

            // Verify results are within expected range [0, 2π)
            assert!(result.x.as_rad() >= -PI && result.x.as_rad() < PI);
            assert!(result.y.as_rad() >= -PI && result.y.as_rad() < PI);
            assert!(result.z.as_rad() >= -PI && result.z.as_rad() < PI);
        }
    }

    #[test]
    fn test_zero_vector() {
        // Special test for zero vector
        let zero = Vector3::new(
            Angular::from_rad(0.0),
            Angular::from_rad(0.0),
            Angular::from_rad(0.0),
        );

        let rounded = zero.mod_to_round();
        assert_eq!(rounded.x.as_rad(), 0.0);
        assert_eq!(rounded.y.as_rad(), 0.0);
        assert_eq!(rounded.z.as_rad(), 0.0);

        let rounded_half = zero.mod_to_round_half();
        assert_eq!(rounded_half.x.as_rad(), 0.0);
        assert_eq!(rounded_half.y.as_rad(), 0.0);
        assert_eq!(rounded_half.z.as_rad(), 0.0);
    }

    #[test]
    fn test_full_rotation() {
        // Test vectors representing full rotations
        let full_rotation = Vector3::new(
            Angular::from_rad(2.0 * PI),
            Angular::from_rad(4.0 * PI),
            Angular::from_rad(6.0 * PI),
        );

        let rounded = full_rotation.mod_to_round();
        assert_relative_eq!(rounded.x.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded.y.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded.z.as_rad(), 0.0, epsilon = 1e-7);

        let rounded_half = full_rotation.mod_to_round_half();
        assert_relative_eq!(rounded_half.x.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded_half.y.as_rad(), 0.0, epsilon = 1e-7);
        assert_relative_eq!(rounded_half.z.as_rad(), 0.0, epsilon = 1e-7);
    }

    #[test]
    fn test_to_vector3_coef_rad() {
        let angular_vec = Vector3::new(
            Angular::from_rad(1.0),
            Angular::from_rad(2.0),
            Angular::from_rad(3.0),
        );

        let coef_vec = angular_vec.to_vector3_coef(AngularType::Rad);

        assert_relative_eq!(coef_vec.x, 1.0);
        assert_relative_eq!(coef_vec.y, 2.0);
        assert_relative_eq!(coef_vec.z, 3.0);
    }

    #[test]
    fn test_to_vector3_coef_deg() {
        let angular_vec = Vector3::new(
            Angular::from_rad(PI),       // π rad = 180°
            Angular::from_rad(PI / 2.0), // π/2 rad = 90°
            Angular::from_rad(PI / 4.0), // π/4 rad = 45°
        );

        let coef_vec = angular_vec.to_vector3_coef(AngularType::Deg);

        assert_relative_eq!(coef_vec.x, 180.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.y, 90.0, epsilon = 1e-10);
        assert_relative_eq!(coef_vec.z, 45.0, epsilon = 1e-10);
    }

    #[test]
    fn test_from_vector_coef_rad() {
        let coef_vec = Vector3::new(Coef::new(1.0), Coef::new(2.0), Coef::new(3.0));

        let angular_vec = Vector3::<Angular>::from_vector_coef(coef_vec, AngularType::Rad);

        assert_relative_eq!(angular_vec.x.as_rad(), 1.0);
        assert_relative_eq!(angular_vec.y.as_rad(), 2.0);
        assert_relative_eq!(angular_vec.z.as_rad(), 3.0);
    }

    #[test]
    fn test_from_vector_coef_deg() {
        let coef_vec = Vector3::new(Coef::new(180.0), Coef::new(90.0), Coef::new(45.0));

        let angular_vec = Vector3::<Angular>::from_vector_coef(coef_vec, AngularType::Deg);

        assert_relative_eq!(angular_vec.x.as_deg(), 180.0);
        assert_relative_eq!(angular_vec.y.as_deg(), 90.0);
        assert_relative_eq!(angular_vec.z.as_deg(), 45.0);
    }

    #[test]
    fn test_from_vector_coef_roundtrip() {
        let original_angular_vec = Vector3::new(
            Angular::from_rad(PI),
            Angular::from_rad(PI / 2.0),
            Angular::from_rad(PI / 4.0),
        );

        let coef_vec = original_angular_vec.to_vector3_coef(AngularType::Rad);
        let reconstructed_angular_vec =
            Vector3::<Angular>::from_vector_coef(coef_vec, AngularType::Rad);

        assert_relative_eq!(
            original_angular_vec.x.as_rad(),
            reconstructed_angular_vec.x.as_rad()
        );
        assert_relative_eq!(
            original_angular_vec.y.as_rad(),
            reconstructed_angular_vec.y.as_rad()
        );
        assert_relative_eq!(
            original_angular_vec.z.as_rad(),
            reconstructed_angular_vec.z.as_rad()
        );
    }

    #[test]
    fn test_to_cos_matrix_left_hand_matches_angle2c_table() {
        // 固定输入角
        let bi = 0.1234567890123_f64;
        let bj = -0.9876543210987_f64;
        let bk = 0.4567891234567_f64;
        let angles = Vector3::new(Angular::from_rad(bi), Angular::from_rad(bj), Angular::from_rad(bk));

        // 仅覆盖 12 个转序（6 个 Tait-Bryan + 6 个 Proper Euler）
        // 表项: (code, [a00,a01,a02,a10,a11,a12,a20,a21,a22])
        let cases: &[(u32, [f64; 9])] = &[
            // Proper Euler (i=k)
            (0x121, [0.5506493978, -0.1027923194, 0.8283833532, -0.3681762079, 0.8607342051, 0.3515436076, -0.7491538698, -0.4985683175, 0.4361170853]),
            (0x131, [0.5506493978, -0.8283833532, -0.1027923194, 0.7491538698, 0.4361170853, 0.4985683175, -0.3681762079, -0.3515436076, 0.8607342051]),
            (0x212, [0.8607342051, -0.3681762079, -0.3515436076, -0.1027923194, 0.5506493978, -0.8283833532, 0.4985683175, 0.7491538698, 0.4361170853]),
            (0x232, [0.4361170853, -0.7491538698, -0.4985683175, 0.8283833532, 0.5506493978, -0.1027923194, 0.3515436076, -0.3681762079, 0.8607342051]),
            (0x313, [0.8607342051, 0.3515436076, -0.3681762079, -0.4985683175, 0.4361170853, -0.7491538698, -0.1027923194, 0.8283833532, 0.5506493978]),
            (0x323, [0.4361170853, 0.4985683175, 0.7491538698, -0.3515436076, 0.8607342051, -0.3681762079, -0.8283833532, -0.1027923194, 0.5506493978]),
            // Tait-Bryan (i,j,k 全不同)
            (0x123, [0.4941931534, 0.3454583188, 0.7977666808, -0.2428742193, 0.9359810416, -0.2548560446, -0.8347366296, -0.0678088474, 0.5464583420]),
            (0x132, [0.4941931534, -0.6891372664, -0.5299650509, 0.8347366296, 0.5464583420, 0.0678088474, 0.2428742193, -0.4758919085, 0.8453040903]),
            (0x213, [0.8453040903, 0.2428742193, -0.4758919085, -0.5299650509, 0.4941931534, -0.6891372664, 0.0678088474, 0.8347366296, 0.5464583420]),
            (0x231, [0.5464583420, -0.8347366296, -0.0678088474, 0.7977666808, 0.4941931534, 0.3454583188, -0.2548560446, -0.2428742193, 0.9359810416]),
            (0x312, [0.9359810416, -0.2548560446, -0.2428742193, -0.0678088474, 0.5464583420, -0.8347366296, 0.3454583188, 0.7977666808, 0.4941931534]),
            (0x321, [0.5464583420, 0.0678088474, 0.8347366296, -0.4758919085, 0.8453040903, 0.2428742193, -0.6891372664, -0.5299650509, 0.4941931534]),
        ];

        fn build_seq(code: u32) -> RotationSeq {
            let i = ((code >> 8) & 0x3) as u8;
            let j = ((code >> 4) & 0x3) as u8;
            let k = (code & 0x3) as u8;
            let mut seq = RotationSeq::default();
            match i { 1 => { seq.x().unwrap(); }, 2 => { seq.y().unwrap(); }, _ => { seq.z().unwrap(); } }
            match j { 1 => { seq.x().unwrap(); }, 2 => { seq.y().unwrap(); }, _ => { seq.z().unwrap(); } }
            match k { 1 => { seq.x().unwrap(); }, 2 => { seq.y().unwrap(); }, _ => { seq.z().unwrap(); } }
            seq
        }

        for (code, expect) in cases.iter() {
            let seq = build_seq(*code);
            let m = angles.to_cos_matrix(seq, RotationHand::Left).unwrap();
            let a = m.to_array();
            let flat = [a[0][0], a[0][1], a[0][2], a[1][0], a[1][1], a[1][2], a[2][0], a[2][1], a[2][2]];
            for i in 0..9 {
                assert_relative_eq!(flat[i], expect[i], epsilon = 1e-9);
            }
        }
    }

    #[test]
    fn test_to_cos_matrix_left_hand_all_sequences_consistency() {
        // 另一组角度，覆盖 12 个转序，断言与基于左手 rx/ry/rz 手工构造的一致
        let bi = -0.3141592653589_f64;   // -π/10
        let bj = 0.7891234567890_f64;
        let bk = -1.2345678901234_f64;

        let angles = Vector3::new(Angular::from_rad(bi), Angular::from_rad(bj), Angular::from_rad(bk));

        // 12 个序：6 个 Tait-Bryan + 6 个 Proper Euler
        let seq_codes: [u32; 12] = [
            0x123, 0x132, 0x213, 0x231, 0x312, 0x321, // Tait-Bryan
            0x121, 0x131, 0x212, 0x232, 0x313, 0x323, // Proper Euler
        ];

        fn tags_from_code(code: u32) -> (u8, u8, u8) {
            (((code >> 8) & 0x3) as u8, ((code >> 4) & 0x3) as u8, (code & 0x3) as u8)
        }

        for code in seq_codes.iter() {
            // 构序
            let mut seq = RotationSeq::default();
            let (ti, tj, tk) = tags_from_code(*code);
            match ti { 1 => { seq.x().unwrap(); }, 2 => { seq.y().unwrap(); }, _ => { seq.z().unwrap(); } }
            match tj { 1 => { seq.x().unwrap(); }, 2 => { seq.y().unwrap(); }, _ => { seq.z().unwrap(); } }
            match tk { 1 => { seq.x().unwrap(); }, 2 => { seq.y().unwrap(); }, _ => { seq.z().unwrap(); } }

            // 被测矩阵
            let m = angles.to_cos_matrix(seq, RotationHand::Left).unwrap();

            // 期望：Ak · Aj · Ai，使用左手基元旋转（Ai 用 bi，Aj 用 bj，Ak 用 bk）
            let ai = match ti { 1 => rx_left(bi), 2 => ry_left(bi), _ => rz_left(bi) };
            let aj = match tj { 1 => rx_left(bj), 2 => ry_left(bj), _ => rz_left(bj) };
            let ak = match tk { 1 => rx_left(bk), 2 => ry_left(bk), _ => rz_left(bk) };
            let expected = ak.product(&aj.product(&ai));

            let a = m.to_array();
            let e = expected.to_array();
            for r in 0..3 { for c in 0..3 { assert_relative_eq!(a[r][c], e[r][c], epsilon = 1e-12); } }
        }
    }

    #[test]
    fn test_to_cos_matrix_right_hand_zero_angles_identity() {
        // 零角 -> 单位阵（任意转序）
        let angles = Vector3::new(Angular::from_rad(0.0), Angular::from_rad(0.0), Angular::from_rad(0.0));
        let mut seq = RotationSeq::default();
        seq.x().unwrap(); seq.y().unwrap(); seq.z().unwrap(); // XYZ
        let m = angles.to_cos_matrix(seq, RotationHand::Right).unwrap();
        let a = m.to_array();
        for r in 0..3 { for c in 0..3 { let exp = if r==c {1.0} else {0.0}; assert_relative_eq!(a[r][c], exp, epsilon = 1e-12); } }
    }

    #[test]
    fn test_to_cos_matrix_right_hand_single_axis_matches_primitives() {
        // 仅第一步角非零（roll），XYZ: R = Rz(0)*Ry(0)*Rx(bi) = Rx_right(bi)
        let bi = 0.3456789012345_f64;
        let angles = Vector3::new(Angular::from_rad(bi), Angular::from_rad(0.0), Angular::from_rad(0.0));
        let mut seq = RotationSeq::default();
        seq.x().unwrap(); seq.y().unwrap(); seq.z().unwrap(); // XYZ
        let m = angles.to_cos_matrix(seq, RotationHand::Right).unwrap();
        let expected = rx_right(bi);
        let a = m.to_array();
        let e = expected.to_array();
        for r in 0..3 { for c in 0..3 { assert_relative_eq!(a[r][c], e[r][c], epsilon = 1e-12); } }

        // 仅第二步角非零（pitch），XYZ: R = Rz(0)*Ry(bj)*Rx(0) = Ry_right(bj)
        let bj = -0.4567890123456_f64;
        let angles = Vector3::new(Angular::from_rad(0.0), Angular::from_rad(bj), Angular::from_rad(0.0));
        let mut seq = RotationSeq::default();
        seq.x().unwrap(); seq.y().unwrap(); seq.z().unwrap();
        let m = angles.to_cos_matrix(seq, RotationHand::Right).unwrap();
        let expected = ry_right(bj);
        let a = m.to_array();
        let e = expected.to_array();
        for r in 0..3 { for c in 0..3 { assert_relative_eq!(a[r][c], e[r][c], epsilon = 1e-12); } }

        // 仅第三步角非零（yaw），XYZ: R = Rz(bk)*Ry(0)*Rx(0) = Rz_right(bk)
        let bk = 1.2345678901234_f64;
        let angles = Vector3::new(Angular::from_rad(0.0), Angular::from_rad(0.0), Angular::from_rad(bk));
        let mut seq = RotationSeq::default();
        seq.x().unwrap(); seq.y().unwrap(); seq.z().unwrap();
        let m = angles.to_cos_matrix(seq, RotationHand::Right).unwrap();
        let expected = rz_right(bk);
        let a = m.to_array();
        let e = expected.to_array();
        for r in 0..3 { for c in 0..3 { assert_relative_eq!(a[r][c], e[r][c], epsilon = 1e-12); } }
    }

    #[test]
    fn test_to_cos_matrix_right_vs_left_transpose_relation() {
        // Right 主动 == Left 被动（转置关系）
        let bi = 0.2_f64; let bj = -0.4_f64; let bk = 0.6_f64;
        let angles = Vector3::new(Angular::from_rad(bi), Angular::from_rad(bj), Angular::from_rad(bk));
        let mut seq = RotationSeq::default();
        seq.y().unwrap(); seq.z().unwrap(); seq.x().unwrap(); // YZX 序
        let mr = angles.to_cos_matrix(seq, RotationHand::Right).unwrap();
        // Left 被动应满足：R_right = (Ai_left · Aj_left · Ak_left)^T
        // 因此需要手工按 Ai·Aj·Ak 顺序构造 Left，再取转置比较
        let (ti, tj, tk) = (1u8, 2u8, 3u8); // 这里与上面 seq 一致为 Y(2) Z(3) X(1)? 修正如下:
        let ti = 2u8; let tj = 3u8; let tk = 1u8; // YZX
        let ai_l = match ti { 1 => rx_left(bi), 2 => ry_left(bi), _ => rz_left(bi) };
        let aj_l = match tj { 1 => rx_left(bj), 2 => ry_left(bj), _ => rz_left(bj) };
        let ak_l = match tk { 1 => rx_left(bk), 2 => ry_left(bk), _ => rz_left(bk) };
        let left_ai_aj_ak = ai_l.product(&aj_l).product(&ak_l);
        let ar = mr.to_array();
        let al = left_ai_aj_ak.to_array();
        // ar == (Ai_l · Aj_l · Ak_l)^T
        for r in 0..3 { for c in 0..3 { assert_relative_eq!(ar[r][c], al[c][r], epsilon = 1e-12); } }
    }
}
