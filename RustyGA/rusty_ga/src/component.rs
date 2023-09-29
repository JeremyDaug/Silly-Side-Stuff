use std::{ops, collections::HashSet};

use crate::basis::ONBasis;

/// # Component
/// 
/// A Component is the mathimatical construct which in formed out of
/// a combination of a magnitude and a basis k-vector.
/// 
/// Components are guaranteed to have their bases in order
/// 
/// Contains 2 part of data. The magnitude (mag) and the bases.
/// Magnitude * Bases is the component.
#[derive(Debug, Clone)]
pub struct Component {
    /// # Magnitude
    /// 
    /// The size of the component.
    pub mag: f64,
    /// # Basis
    /// 
    /// The basis of the component. IE the e_{bases} of the component.
    /// These should always be in id order after being new'd up.
    bases: Vec<ONBasis>
}

const ZERO: Component = Component { mag: 0.0, bases: vec![] };

impl Component {
    /// # New
    /// 
    /// creates a new component based on a magnitude and basis.
    /// 
    /// If basis is not ordered correctly, it reorders the bases appropriately.
    pub fn new(mag: f64, bases: Vec<ONBasis>) -> Component {
        let mut unique = HashSet::new();
        for basis in bases.iter() {
            unique.insert(basis);
        }
        // if duplicate ids, cleare out duplicate bases.
        if unique.len() != bases.len() {
            return Component {mag, bases}.reorder_bases();
        }
        for idx in 0..(bases.len()-1) {
            // if any disorder, make then reorder bases immediately.
            if bases[idx].unwrap() > bases[idx+1].unwrap() {
                return Component {mag, bases}.reorder_bases();
            }
        }
        Component { mag, bases }
    }

    /// # Reorder Bases
    /// 
    /// creates a copy of the component with it's basis vectors organized by
    /// id. Magnitude flips based on how many 
    fn reorder_bases(&self) -> Component {
        let mut result = Component { mag: self.mag, bases: self.bases.clone() };

        // loop until no movement.
        loop {
            let mut change = false;
            let mut current = 0;
            while current+1 < result.bases.len() {
                // if the two match, break then multiply by mag by square
                if result.bases[current] == result.bases[current+1] {
                    let basis = result.bases.remove(current);
                    result.bases.remove(current);
                    result.mag *= basis.sqr();
                    change = true;
                    // if after the removal we would step out of the list, get out of the current loop.
                    if current+1 >= result.bases.len() {
                        break;
                    }
                }
                // swap them if the two are out of order.
                if result.bases[current] > result.bases[current+1] {
                    result.bases.swap(current, current+1);
                    result.mag *= -1.0;
                    change = true;
                }
                current += 1; // end by stepping up.
            }
            if !change {
                break;
            }
        }

        result
    }

    /// # Base Add
    /// 
    /// Adds two components if they share the same basis vectors.
    pub fn comp_add(&self, rhs: &Component) -> Component {
        // if the same length, organize their bases
        if self.bases.len() == rhs.bases.len() {
            // if same length, check that each basis is the same
            for idx in 0..self.bases.len() {
                if self.bases[idx] != rhs.bases[idx] {
                    return ZERO; // if any mismatch, return 0.
                }
            }
            // if bases match, then add magnitudes
            return Component { mag: self.mag + rhs.mag, bases: self.bases.clone()};
        } else {
            ZERO
        }
    }

    /// # Scalar Multiplication
    /// 
    /// Multiplies the component by a scalar.
    pub fn scalar_mult(&self, rhs: f64) -> Component {
        Component::new(self.mag * rhs, self.bases)
    }

    /// # geometric Product
    /// 
    /// Multiplies two components together via Geometric Product.
    /// 
    /// Bases are comibned directly, reordered, then if any squares, 
    /// those are applied.
    /// 
    /// If any results in the result being 0, then it shortcuts out.
    pub fn geo_product(&self, rhs: &Component) -> Component {
        // combine bases first
        let mut bases = self.bases.clone();
        bases.extend(rhs.bases.clone());
        // get component
        Component::new(self.mag*rhs.mag, bases)
    }

    /// # Outer Product
    /// 
    /// Takes the outer product of two components. Components which share a 
    /// basis produce a zero.
    pub fn outer_product(&self, rhs: &Component) -> Component {
        // check for overlapping bases
        let mut uniques = HashSet::new();
        for basis in self.bases.iter() {
            uniques.insert(basis);
        }
        for basis in rhs.bases.iter() {
            uniques.insert(basis);
        }
        // if the number of unique bases is not equal to the sum of the grades
        // then it must have at least one similar basis, thus the outer product
        // is zero
        if uniques.len() != (self.grade() + rhs.grade()) {
            ZERO
        } else {
            let mut bases = self.bases.clone();
            bases.extend(rhs.bases.clone());
            Component {mag: self.mag * rhs.mag, bases }
        }
    }

    /// # Reversion
    /// 
    /// Produces the reverse of a blade in the grade ordering of 
    /// ++--++--...
    pub fn reversion(&self) -> Component {
        let grade = self.grade() / 2;
        if grade % 2 == 0 { // 0, 1, 4, 5
            Component::new(self.mag, self.bases)
        } else { // 2, 3, 6, 7
            Component::new(-self.mag, self.bases)
        }
    }

    /// # Involution
    /// 
    /// Involutes the the component based on it's grade.
    /// +-+-+-+-
    pub fn involution(&self) -> Component {
        Component::new(self.mag * (-1).pow(self.grade()) , self.bases)
    }

    /// # Scalar Product
    /// 
    /// Scalar Product multiplies blades of like grade. Since all components 
    /// are blades, they always go. If the blades are of different grades
    /// it returns 0. If they are of the same it takes the determinant of them
    /// if they are both scalars, it multiplies them.
    pub fn scalar_product(&self, rhs: &Component) -> f64 {
        if self.grade() != rhs.grade() {
            return 0.0;
        }
        // if 1 or zero, multiply the bases and end there.
        if self.grade() < 2 {
            return self.mag * rhs.mag;
        }
        // if not zero, then do a determinant on it

        return 0.0;
    }

    /// # Scalar Dot Product
    /// 
    /// Performs a dot product, but only returns when the grades are equal.
    /// 
    /// Returns 0.0 if there is any mismatched basis.
    /// 
    /// Note: This is only meant for blades, all components are blades, 
    /// but multivectors or other k-vectors may not be blades.
    pub fn scalar_dot(&self, rhs: &Component) -> f64 {
        if self.grade() == rhs.grade() {
            for (b1, b2) in self.bases
            .iter().zip(rhs.bases.iter()) {
                if b1 != b2 { // if any basis mismatch, return 0
                    return 0.0;
                }
                // if they match, return the product of the magnitudes
                 // TODO come back here!
                return self.mag * rhs.mag;
            }

        }
        0.0
    }

    /// # Grade
    /// 
    /// Retieves what the grade of the component is, IE how many bases it has.
    pub fn grade(&self) -> usize {
        self.bases.len()
    }

    pub fn bases(&self) -> &[ONBasis] {
        self.bases.as_ref()
    }
}

// Left Contraction >>

// real + real
impl ops::Shl for Component {
    type Output = Component;

    fn shl(self, rhs: Self) -> Self::Output {
        self.outer_product(&rhs)
    }
}

// ref + ref
impl ops::BitXor<&Component> for &Component {
    type Output = Component;

    fn bitxor(self, rhs: &Component) -> Self::Output {
        self.outer_product(rhs)
    }
}

// ref + real
impl ops::BitXor<Component> for &Component {
    type Output = Component;

    fn bitxor(self, rhs: Component) -> Self::Output {
        self.outer_product(&rhs)
    }
}

// real + ref
impl ops::BitXor<&Component> for Component {
    type Output = Component;

    fn bitxor(self, rhs: &Component) -> Self::Output {
        self.outer_product(rhs)
    }
}

// Outer Product (^)

// real + real
impl ops::BitXor for Component {
    type Output = Component;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.outer_product(&rhs)
    }
}

// Addition

// real + real
impl ops::Add for Component {
    type Output = Component;

    fn add(self, rhs: Self) -> Self::Output {
        self.comp_add(&rhs)
    }
}

// ref + ref
impl ops::Add<&Component> for &Component {
    type Output = Component;

    fn add(self, rhs: &Component) -> Self::Output {
        self.comp_add(rhs)
    }
}

// ref + real
impl ops::Add<Component> for &Component {
    type Output = Component;

    fn add(self, rhs: Component) -> Self::Output {
        self.comp_add(&rhs)
    }
}

// real + ref
impl ops::Add<&Component> for Component {
    type Output = Component;

    fn add(self, rhs: &Component) -> Self::Output {
        self.comp_add(rhs)
    }
}

// Subtraction

// real + real
impl ops::Sub for Component {
    type Output = Component;

    fn sub(self, rhs: Self) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// ref + ref
impl ops::Sub<&Component> for &Component {
    type Output = Component;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// ref + real
impl ops::Sub<Component> for &Component {
    type Output = Component;

    fn sub(self, rhs: Component) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// real + ref
impl ops::Sub<&Component> for Component {
    type Output = Component;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// Geometric Product

// real + real
impl ops::Mul for Component {
    type Output = Component;

    fn mul(self, rhs: Self) -> Self::Output {
        self.geo_product(&rhs)
    }
}

// ref + ref
impl ops::Mul<&Component> for &Component {
    type Output = Component;

    fn mul(self, rhs: &Component) -> Self::Output {
        self.geo_product(&rhs)
    }
}

// ref + real
impl ops::Mul<Component> for &Component {
    type Output = Component;

    fn mul(self, rhs: Component) -> Self::Output {
        self.geo_product(&rhs)
    }
}

// real + ref
impl ops::Mul<&Component> for Component {
    type Output = Component;

    fn mul(self, rhs: &Component) -> Self::Output {
        self.geo_product(&rhs)
    }
}

// Scalar Multiplication

// f64 * component
impl ops::Mul<f64> for Component {
    type Output = Component;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scalar_mult(rhs)
    }
}

// f64 * &component
impl ops::Mul<f64> for &Component {
    type Output = Component;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scalar_mult(rhs)
    }
}

// &f64 * component
impl ops::Mul<&f64> for Component {
    type Output = Component;

    fn mul(self, rhs: &f64) -> Self::Output {
        self.scalar_mult(*rhs)
    }
}

// &f64 * &component
impl ops::Mul<&f64> for &Component {
    type Output = Component;

    fn mul(self, rhs: &f64) -> Self::Output {
        self.scalar_mult(*rhs)
    }
}

// Component * f64
impl ops::Mul<Component> for f64 {
    type Output = Component;

    fn mul(self, rhs: Component) -> Self::Output {
        rhs.scalar_mult(self)
    }
}

// &Component * f64
impl ops::Mul<&Component> for f64 {
    type Output = Component;

    fn mul(self, rhs: &Component) -> Self::Output {
        rhs.scalar_mult(self)
    }
}

// Component * &f64
impl ops::Mul<Component> for &f64 {
    type Output = Component;

    fn mul(self, rhs: Component) -> Self::Output {
        rhs.scalar_mult(*self)
    }
}

// &Component * &f64
impl ops::Mul<&Component> for &f64 {
    type Output = Component;

    fn mul(self, rhs: &Component) -> Self::Output {
        rhs.scalar_mult(*self)
    }
}


// -real
impl ops::Neg for Component {
    type Output = Component;

    fn neg(self) -> Self::Output {
        Component { mag: -self.mag, bases: self.bases.clone() }
    }
}

// -ref
impl ops::Neg for &Component {
    type Output = Component;

    fn neg(self) -> Self::Output {
        Component { mag: -self.mag, bases: self.bases.clone()}
    }
}
