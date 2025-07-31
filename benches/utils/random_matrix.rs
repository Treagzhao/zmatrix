/// Generates a random array of 9 f64 values between 0.0 and 1.0
/// use rand::Rng;
pub fn random_f64_array_9() -> [f64; 9] {
    let mut rng = rand::thread_rng();
    let mut arr = [0.0; 9];
    for item in &mut arr {
        *item = rng.gen();
    }
    arr
}
