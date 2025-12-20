use crate::{basis::ONBasis, component::Component, multivector::Multivector, vector::Vector};

pub const ZERO: Blade = Blade{ components: vec![], vectors: vec![] };

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
    // # Component
    // 
    // The Components of the blade. Straight interpretation.
    pub components: Vec<Component>,
    /// # Vectors
    /// 
    /// The vectors which make up the blade. Makes decomposition easy.
    pub vectors: Vec<Vector>,
}

impl Blade {
    pub const ZERO: Self = Self { components: vec![], vectors: vec![] };

    /// # Component length
    /// 
    /// How many components the Blade has.
    pub fn comp_len(&self) -> usize {
        self.components.len()
    }

    /// # Vector Count
    /// 
    /// How many vectors the Blade has.
    pub fn vector_count(&self) -> usize {
        self.vectors.len()
    }

    /// # Grade
    /// 
    /// The grade of the blade.
    pub fn grade(&self) -> usize {
        self.vectors.len()
    }

    /// # New
    /// 
    /// Creates a new blade from a list of vectors.
    pub fn new(vectors: &Vec<Vector>) -> Self {
        let mut result = ZERO;
        result.vectors = vectors.clone();
        let mut mv = Multivector::ZERO;
        for vec in vectors.iter() {
            for comp in vec.components.iter() {

            }
        }
        result
    }

    

    /// # From Component
    /// 
    /// Converts a Component to A Blade. All components are blades of grade N.
    pub fn from_component(component: &Component) -> Self {
        Self { components: vec![component.clone()], vectors: component.vector_decomposition()}
    }

    /// # From Vector 
    /// 
    /// Converts a Vector to a Blade, valid as all vectors are blades.
    pub fn from_vector(vector: &Vector) -> Self {
        Self { components: vector.components.clone(), vectors: vec![vector.clone()] }
    }

    /// # To Multivector
    /// 
    /// Converts the Blade to a Multivector.
    pub fn to_mv(&self) -> Multivector {
        Multivector { 
            components: self.components.clone(), 
            blades: vec![self.clone()]
        }
    }
}

/// Geometry, the context of our 
pub struct Geometry {
    /// The Orthonormal Bases of this geometry.
    _onbases: Vec<ONBasis>,
    /// Other available spaces/bases in this Geometry. These are not 
    /// guaranteed to be orthonormal.
    _spaces: Vec<Vec<ONBasis>>,
}