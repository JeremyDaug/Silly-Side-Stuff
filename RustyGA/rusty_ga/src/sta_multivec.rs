pub struct STAMultivec {
    pub val: [f64; 16],
}

impl STAMultivec {
    pub fn new(val: [f64; 16]) -> Self { 
        Self { val } 
    }

}