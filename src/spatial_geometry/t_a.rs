pub struct AAAA{
    pub a: i32,
}

impl AAAA{
    pub fn to_test_string(&self) -> String{
        format!("test aaaa {}",self.a)
    }
}