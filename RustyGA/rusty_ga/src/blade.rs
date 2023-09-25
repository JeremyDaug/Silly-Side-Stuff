/// # Multivector
/// 
/// A Multivector is a collection of k-vectors. 
/// 
/// It stores these vectors in the form of their components.
/// while components are organized from lowest grade to highest, the order
/// within 
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
    /// # Component
    /// 
    /// The Components of the blade.
    component: Vec<Component>
}

/// # Component
/// 
/// A Component is the mathimatical construct which in formed out of
/// a combination of a magnitude and a basis k-vector.
/// 
/// This component is not guaranteed to have it's basis ordered from lowest to
/// highest.
/// 
/// Contains 2 part of data. The magnitude (mag) and the bases.
/// Magnitude * Bases is the component.
#[derive(Debug)]
pub struct Component {
    /// # Magnitude
    /// 
    /// The size of the component.
    mag: f64,
    /// # Basis
    /// 
    /// The basis of the component. IE the e_{bases} of the component
    bases: Vec<usize>
}