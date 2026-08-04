use crate::{component::Component};


/// # Blade
/// 
/// A Blade is the simplest form of K-Vector and is
/// capable of being formed purely through the outer product
/// of a series of vectors.
/// 
/// An example of a blade (R^3)
///     (4 e_1 + 2 e_2) ^ (2 e_2 + e_3) 
///     = 8 e_12 + 2 e_23 - 4 e_31 
/// 
/// An Example of a Non-blade (R^4)
///     2 e_12 + 3 e_34
#[derive(Debug, Clone, PartialEq)]
pub struct Blade {
    /// The magnitude of the blade, calculated only at start.
    pub mag: f64,
    // # Component
    // 
    // The Components of the blade. Straight interpretation.
    pub components: Vec<Component>,
    // # Vectors
    // 
    // The vectors which make up the blade. Makes decomposition easy.
    //pub vectors: Vec<Vector>,
}

impl Blade {
    pub const ZERO: Self = Self { mag: 0.0, components: vec![], };

}