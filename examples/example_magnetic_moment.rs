use zmatrix::physics::basic::{MagneticMoment, MagneticInduction, Energy, PhysicalQuantity};

fn main() {
    println!("=== 磁矩物理量示例 ===\n");

    // 1. 创建不同单位的磁矩
    println!("1. 创建不同单位的磁矩:");
    let mm1 = MagneticMoment::from_am2(1.0); // 1 A·m²
    let mm2 = MagneticMoment::from_mill_am2(1000.0); // 1000 mA·m² = 1 A·m²
    let mm3 = MagneticMoment::from_micro_am2(1e6); // 1e6 μA·m² = 1 A·m²
    let mm4 = MagneticMoment::from_nano_am2(1e9); // 1e9 nA·m² = 1 A·m²

    println!("   mm1 = {} A·m²", mm1.as_am2());
    println!("   mm2 = {} mA·m² = {} A·m²", mm2.as_mill_am2(), mm2.as_am2());
    println!("   mm3 = {} μA·m² = {} A·m²", mm3.as_micro_am2(), mm3.as_am2());
    println!("   mm4 = {} nA·m² = {} A·m²", mm4.as_nano_am2(), mm4.as_am2());

    // 2. J/T 单位的磁矩
    println!("\n2. J/T 单位的磁矩:");
    let mm5 = MagneticMoment::from_j_per_tesla(1.0); // 1 J/T
    let mm6 = MagneticMoment::from_mill_j_per_tesla(1000.0); // 1000 mJ/T = 1 J/T
    let mm7 = MagneticMoment::from_micro_j_per_tesla(1e6); // 1e6 μJ/T = 1 J/T
    let mm8 = MagneticMoment::from_nano_j_per_tesla(1e9); // 1e9 nJ/T = 1 J/T

    println!("   mm5 = {} J/T", mm5.as_j_per_tesla());
    println!("   mm6 = {} mJ/T = {} J/T", mm6.as_mill_j_per_tesla(), mm6.as_j_per_tesla());
    println!("   mm7 = {} μJ/T = {} J/T", mm7.as_micro_j_per_tesla(), mm7.as_j_per_tesla());
    println!("   mm8 = {} nJ/T = {} J/T", mm8.as_nano_j_per_tesla(), mm8.as_j_per_tesla());

    // 3. 单位转换
    println!("\n3. 单位转换:");
    println!("   1 A·m² = {} J/T", mm1.as_j_per_tesla());
    println!("   1 J/T = {} A·m²", mm5.as_am2());
    println!("   1 A·m² = {} mA·m²", mm1.as_mill_am2());
    println!("   1 A·m² = {} μA·m²", mm1.as_micro_am2());
    println!("   1 A·m² = {} nA·m²", mm1.as_nano_am2());

    // 4. 磁矩运算
    println!("\n4. 磁矩运算:");
    let mm_sum = mm1 + mm2; // 1 A·m² + 1 A·m² = 2 A·m²
    let mm_diff = mm_sum - mm1; // 2 A·m² - 1 A·m² = 1 A·m²
    let mm_scale = mm1 * 3.0; // 1 A·m² * 3 = 3 A·m²
    let mm_div = mm_scale / 2.0; // 3 A·m² / 2 = 1.5 A·m²

    println!("   mm1 + mm2 = {} A·m²", mm_sum.as_am2());
    println!("   mm_sum - mm1 = {} A·m²", mm_diff.as_am2());
    println!("   mm1 * 3 = {} A·m²", mm_scale.as_am2());
    println!("   mm_scale / 2 = {} A·m²", mm_div.as_am2());

                    // 5. 磁矩与磁感应强度的乘积（得到能量）
                println!("\n5. 磁矩与磁感应强度的乘积（得到能量）:");
                let b = MagneticInduction::from_tesla(2.0); // 2 T
                let energy = mm1 * b; // 1 A·m² × 2 T = 2 J
                println!("   磁矩: {} A·m²", mm1.as_am2());
                println!("   磁感应强度: {} T", b.as_tesla());
                println!("   能量: {} J", energy.as_joule());

                    // 6. 不同单位之间的运算
                println!("\n6. 不同单位之间的运算:");
                let mm_micro = MagneticMoment::from_micro_am2(1e6); // 1 A·m²
                let mm_mill = MagneticMoment::from_mill_j_per_tesla(1000.0); // 1 J/T = 1 A·m²
                let mm_result = mm_micro + mm_mill; // 1 A·m² + 1 A·m² = 2 A·m²
                println!("   {} μA·m² + {} mJ/T = {} A·m²",
                         mm_micro.as_micro_am2(),
                         mm_mill.as_mill_j_per_tesla(),
                         mm_result.as_am2());

                // 7. 零值检查
                println!("\n7. 零值检查:");
                let mm_zero = MagneticMoment::from_am2(0.0);
                let mm_nonzero = MagneticMoment::from_am2(1.0);
                println!("   mm_zero.is_zero() = {}", mm_zero.is_zero());
                println!("   mm_nonzero.is_zero() = {}", mm_nonzero.is_zero());

    println!("\n=== 示例完成 ===");
}
