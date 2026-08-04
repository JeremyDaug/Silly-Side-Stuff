use std::ops::Mul;

use crate::component::Component;

/// # D1 Vector
/// 
/// D1 Vector (1-Vector) is a mathematical vector, ie a 1 dimensional structure.
/// 
/// This is useful to have as it allows us to define and use them securely elsewhere.
/// 
/// These vectors are a list of grade 1 components. 
/// 
/// Vectors are always blades.
/// 
/// Components are organized by basis in the component vector for consistency and 
/// possible later improvements.
/// 
/// TODO: Consider making component into a special restricted variant for Vector. Would reduce size, increase speed, and reduce extraction code for the basis.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    /// The magnitude of the vector. Should be calculated only once, when generated.
    pub mag: f64,
    /// The components that make up our Vector
    pub components: Vec<Component>,
}

impl Vector {
    pub const ZERO: Vector = Vector { mag: 0.0, components: vec![] };
}