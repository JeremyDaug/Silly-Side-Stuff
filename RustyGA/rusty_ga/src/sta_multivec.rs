pub struct STAMultivec {
    pub val: [f64; 16],
}

impl sta_multivec {
    pub fn new(val: [f64; 16]) -> Self { 
        Self { val } 
    }

    /// a Zero Multivector
    pub fn zero() -> Self {
        Self { val: vec![] }
    }
    
    pub fn mag(&self) -> f64 {
        
    }
}