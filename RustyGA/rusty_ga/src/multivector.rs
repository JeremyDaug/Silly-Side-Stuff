use crate::{komponent::Komponent, orthonormalbases::OrthonormalBases};

/// # Multivector
/// 
/// A Multivector is a collection of Komponents in a particular basis. 
#[derive(Debug, Clone)]
pub struct Multivector {
    /// The Components which make up this vector.
    pub komponents: Vec<Komponent>,
    /// The Basis Data available to the multivector.
    onb: Rc<OrthonormalBases>,
    /// The Magnitude, stored to make calculations easier.
    magnitude: f64,
}

impl Multivector {
    /// # New
    /// 
    /// Creates a new, empty, multivector.
    pub fn new(onb: Rc<OrthonormalBases>) -> Multivector {
        Multivector { 
            komponents: vec![],
            onb: onb.clone(),
            magnitude: 0.0
        }
    }
}