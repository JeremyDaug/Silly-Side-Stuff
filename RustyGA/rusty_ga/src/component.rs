use std::collections::HashSet;
use std::ops;

use gl::ZERO;
use regex::Regex;

use crate::basis::ONBasis;
use crate::multivector::Multivector;

/// # Component
///
/// A Component is the mathimatical construct which in formed out of
/// a combination of a magnitude and a basis k-vector.
///
/// Components are guaranteed to have their bases in order and are also always blades.
///
/// Contains 2 part of data. The magnitude (mag) and the bases.
/// Magnitude * Bases is the component.
#[derive(Debug, Clone)]
pub struct Component {
    /// # Magnitude
    ///
    /// The size of the component.
    mag: f64,
    /// # Basis
    ///
    /// The basis of the component. IE the e_{bases} of the component.
    /// These should always be in id order after being new'd up.
    bases: Vec<ONBasis>
}


impl Default for Component {
    fn default() -> Self {
        Self { mag: Default::default(), bases: Default::default() }
    }
}

impl Component {
    /// # Zero Component
    ///
    /// Returns a component with a magnitude of 0.0 and no bases.
    pub const ZERO: Component = Component { mag: 0.0, bases: vec![] };

    /// # Magnitude
    /// 
    /// The magnitude of the Component.
    pub fn mag(&self) -> f64 {
        self.mag
    }

    /// # Basis
    /// 
    /// Gets a basis from the component.
    pub fn basis(&self, idx: usize) -> Option<&ONBasis> {
        self.bases.get(idx)
    }

    /// # From Basis
    /// 
    /// Takes a basis and returns a Component of Magnitude 1.0 with that basis.
    pub fn from_basis(basis: ONBasis) -> Self {
        Self { mag: 1.0, bases: vec![basis] }
    }

    /// # From Bases
    /// 
    /// Takes a list of ONBases and makes a Component of Magnitude 1.0 with those bases.
    pub fn from_bases(bases: Vec<ONBasis>) -> Self {
        Self { mag: 1.0, bases: bases}.sort_bases()
    }

    /// # Length
    /// 
    /// How many bases the component has.
    pub fn len(&self) -> usize {
        self.bases.len()
    }

    /// # Sort Bases
    /// 
    /// Sorts the bases of a component, if any matches are found, it just takes the 
    /// square of that basis and removes the matches.
    /// 
    /// If bases are not in standardized order, we swap them until they are.
    pub fn sort_bases(mut self) -> Self {
        if self.len() <= 1 {
            // if can't be sorted skip.
            return self;
        }
        loop {// repeat until sorted
            let mut sorted = true;
            let mut idx = 0;
            // go over bases.
            while idx < (self.len()-1) {
                match self.bases[idx].cmp(&self.bases[idx+1]) {
                    // if in order, just increment.
                    std::cmp::Ordering::Less => { idx += 1; },
                    // if equal, multiply by the square, and remove the bases.
                    std::cmp::Ordering::Equal => {
                        self.mag *= self.bases[idx].sqr();
                        self.bases.remove(idx);
                        self.bases.remove(idx);
                        sorted = false;
                    },
                    // out of order, so swap.
                    std::cmp::Ordering::Greater => {
                        self.mag *= -1.0;
                        self.bases.swap(idx, idx+1);
                        idx += 1;
                        sorted = false;
                    },
                }
                if self.mag == 0.0 { // if magnitude becomes zero, just drop out.
                    return Component::ZERO;
                }
            }
            if sorted { // If we stepped through with no swaps or deletions, get out.
                break;
            }
        }

        self
    }

    /// # Same Bases
    /// 
    /// A check between two Components that they have the same Bases vectors.
    pub fn same_bases(&self, rhs: &Component) -> bool {
        if self.len() != rhs.len() {
            return false;
        }
        for (l, r) in self.bases.iter().zip(rhs.bases.iter()) {
            if l != r {
                return false;
            }
        }
        true
    }

    /// # New
    /// 
    /// News up a Component with a defined Magnitude and list of Bases.
    /// 
    /// Bases given will be sorted, with duplicates multiplying to their unit.
    pub fn new(mag: f64, bases: Vec<ONBasis>) -> Component {
        Component { mag, bases }.sort_bases()
    }

    /// # Scalar Mult
    /// 
    /// Does Scalar Multiplication between a component and a Scalar value.
    pub fn scalar_mult(&self, rhs: f64) -> Component {
        Component { mag: self.mag * rhs, bases: self.bases.clone() }
    }

    /// # Component Add
    /// 
    /// Adds two components and guarantees a component is output.
    /// 
    /// If two components cannot add (they don't have the same bases) then it returns
    /// None.
    pub fn component_add(&self, rhs: Self) -> Option<Self> {
        if self.same_bases(&rhs) {
            Some( Self { mag: self.mag + rhs.mag, bases: self.bases.clone() })
        } else {
            None
        }
    }

    /// # Add
    /// 
    /// Adds two components together, producing a Multivector.
    pub fn add(&self, rhs: Self) -> Multivector {
        todo!()
    }

    /// # Geometric Product
    /// 
    /// Takes the geometric product of two components, and returns a component.
    pub fn geometric_product(&self, rhs: Self) -> Self {
        let mut bases = self.bases.clone();
        bases.extend(rhs.bases.clone());
        Self::new(self.mag * rhs.mag, bases)
    }

}


impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.mag == other.mag && self.same_bases(other)
    }
}
