use std::{collections::HashSet, ops::{self, Add}};

use crate::{blade::Blade, component::{self, Component}};

/// # Multivector
/// 
/// A Multivector is a collection of k-vectors. 
/// 
/// It stores these vectors in the form of their components.
/// while components are organized from lowest grade to highest, the order
/// of bases within components are guranteed to be in order from lowest id
/// to highest id. Any other ordering is not applied.
#[derive(Debug, Clone)]
pub struct Multivector {
    /// The Component form of the Multivector
    pub components: Vec<Component>,
    /// The blade form of the multivector.
    pub blades: Vec<Blade>
}

impl Multivector {
    /// # Zero Multivector
    /// 
    /// Has No components.
    pub const ZERO: Multivector = Multivector { components: vec![], blades: vec![] };

}


// Equality
impl PartialEq for Multivector {
    fn eq(&self, other: &Self) -> bool {
        // check that we have the same number of components
        if self.components.len() != other.components.len() {
            return false;
        }
        // Check that all components in lhs are in rhs, order doesn't matter.
        for comp in self.components.iter() {
            if !other.components.contains(comp) {
                return false;
            }
        }
        return true;
    }
}