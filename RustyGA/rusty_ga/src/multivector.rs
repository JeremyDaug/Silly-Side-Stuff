use crate::komponent::Komponent;

/// # Multivector
/// 
/// A Multivector is a collection of Komponents in a particular basis. 
pub struct Multivector {
    /// The Components which make up this vector.
    pub komponents: Vec<Komponent>,
    
    grade: usize
}