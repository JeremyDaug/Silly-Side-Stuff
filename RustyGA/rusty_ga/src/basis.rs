/// # Orthonormal Basis
/// 
/// The storage information for a basis' information.
/// 
/// Includes a unique id for quick searching. A name for the basis and
/// what it squares to.
/// 
/// These basis vectors are considered to be othorgonal and 
#[derive(Debug, Clone)]
pub struct ONBasis {
    pub id: usize,
    pub name: String,
    pub sqr: f64,
}