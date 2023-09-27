use crate::{basis::ONBasis, component::Component};

/// # Multivector
/// 
/// A Multivector is a collection of k-vectors. 
/// 
/// It stores these vectors in the form of their components.
/// while components are organized from lowest grade to highest, the order
/// of bases within components are guranteed to be in order from lowest id
/// to highest id. Any other ordering is not applied.
#[derive(Debug)]
pub struct Multivector {
    components: Vec<Component>
}

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
#[derive(Debug)]
pub struct Blade {
    // # Component
    // 
    // The Components of the blade.
    //component: Vec<Component>
}

/// Geometry, the context of our 
pub struct Geometry {
    /// The Orthonormal Bases of this geometry.
    onbases: Vec<ONBasis>,
    /// Other available spaces/bases in this Geometry. These are not 
    /// guaranteed to be orthonormal.
    spaces: Vec<Vec<ONBasis>>,
}