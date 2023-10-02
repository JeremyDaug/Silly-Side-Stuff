use std::{ops, collections::HashSet, fmt::Display};

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
        if bases.len() > 0 {
            for idx in 0..(bases.len()-1) {
                // if any disorder, make then reorder bases immediately.
                if bases[idx].unwrap() > bases[idx+1].unwrap() {
                    return Component {mag, bases}.reorder_bases();
                }
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
                    // Note: e_11 == e_1 . e_1 == e_1.norm()^2
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
        Component::new(self.mag * rhs, self.bases.clone())
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
            Component::new(self.mag, self.bases.clone())
        } else { // 2, 3, 6, 7
            Component::new(-self.mag, self.bases.clone())
        }
    }

    /// # Involution
    /// 
    /// Involutes the the component based on it's grade.
    /// +-+-+-+-
    pub fn involution(&self) -> Component {
        Component::new(self.mag * (-1.0_f64).powf(self.grade() as f64) , self.bases.clone())
    }

    /// # Inverse
    /// 
    /// Returns the standardized Inverse of this component
    pub fn inverse(&self) -> Component {
        // Component::new(self.mag / self.norm())
        todo!()
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
        let matrix = self.scalar_product_matrix_form(rhs);
    }

    /// # Determinant function
    /// 
    /// Takes a square matrix and returns the determinant of it.
    /// 
    /// # Panics
    /// 
    /// If matrix isn't square it panics.
    fn determinant(matrix: Vec<Vec<f64>>) -> f64 {
        let n = matrix.len();
        if matrix.iter().any(|x| x.len() != n) {
            panic!("Matrix is not square.");
        }
        let mut result = 0.0;
        for col in 0..matrix.len() {
            let mut diag = 0.0;
            for offset in 0..matrix.len() {
                let column = (col + offset) % matrix.len();
                let row = offset;
                
            }
        }
        result
    }

    /// # Scalar Product Matrix Form
    /// 
    /// Takes two components of the same grade, and generates a matrix form 
    /// based on it.
    /// 
    /// a_1 . b_k --> a_1 . b_1
    /// v                   v
    /// v                   v
    /// v                   v
    /// a_k . b_k --> a_k . b_1
    /// 
    /// This assumes they are the same grade, and will not work if they are not.
    /// 
    /// This does not include the magnitude, just the basis multiplications.
    fn scalar_product_matrix_form(&self, rhs: &Component) -> Vec<Vec<f64>> {
        // build out the initial matrix
        let mut result = vec![];
        for lidx in 0..self.grade() {
            result.push(vec![]);
            for ridx in (0..rhs.grade()).rev() {
                result[lidx].push(self.bases[lidx].dot(&rhs.bases[ridx]));
            }
        }
        result
    }

    /// # Left Contraction
    /// 
    /// Left Contraction (>>) removes the lhs from the rhs and returns the 
    /// remaining vectors.
    /// 
    /// When the the left grade is greater than the right, it returns 0.
    /// 
    /// When the Right Grade returns left, it returns a blade.
    /// 
    /// When the two grades are equal it is equivalent to the inner product
    /// of the two components.
    /// 
    /// Because these are components, each basis vector is guaranteed to be
    /// othogonal to all others. So, unless all of lhs is within rhs, then it
    /// will result in a zero.
    /// 
    /// ## Left Contraction Properties
    /// 
    /// Given Scalar a, vectors b and c, and Blades A,B, and C.
    /// 
    /// - a>>B = aB.
    /// - A>>B = 0 if grade(A) > grade(B)
    /// - a>>b = a.inner_product(b)
    /// - a>>(B ^ C) = (a>>B) ^ C + (-1)^grade(B) B ^ (a>>C)
    /// - (A^B)>>C = A>>(B>>C)
    /// - (A + B)>>C = A>>C + B>>C
    /// - A>>(B+C) = A>>B + A>>C
    /// - (aA)>>B = a(A>>B) = A>>(aB)
    /// - b>>A = 0 if b is perpendicular to all vectors in A.
    /// - The result of A>>B is perpendicular to A
    /// - The norm of A>>B is proportional to the norms of A and B and the
    ///     cosine between A and it's projection on B.
    pub fn left_cont(&self, rhs: &Component) -> Component {
        // since we are components, lhs must be totally contained in rhs, else it is Zero.
        if self.bases.iter().any(|x| !rhs.bases.contains(x)) {
            return ZERO;
        }
        // since all of lhs bases are contained in rhs's bases, combine the magnitudes
        let mut mag = self.mag * rhs.mag;
        // and iterate over the bases, reordering to combine to their squares.
        let mut final_bases = rhs.bases.clone();
        for basis in self.bases.iter().rev() {
            // get the position of the first basis which matches
            let idx = final_bases.iter().position(|&x| x == *basis).unwrap();
            // multiply the magnitude by -1^swaps and the square of the basis.
            mag *= (-1.0_f64).powf(idx as f64) * basis.sqr(); 
            if mag == 0.0 { // if magnitude is now zero, just return zero.
                return ZERO;
            }
            // with it squared, remove that basis from our final bases.
            final_bases.remove(idx);
        }
        // since we've removed all the lhs bases and didn't run into any zeroes, 
        // return the final_bases and the new magnitude as a component.
        Component::new(mag, final_bases)
    }

    /// # Right Contraction
    /// 
    /// Same as left, but reversed.
    pub fn right_cont(&self, rhs: &Component) -> Component {
        (-1.0_f64).powf((self.grade() * (1 + rhs.grade())) as f64) * rhs.left_cont(self)
    }

    /// # Inner Product
    /// 
    /// Performs a inner/dot product, but only returns when the grades are equal.
    /// 
    /// Returns 0.0 if there is any mismatched basis.
    /// 
    /// Note: This is only meant for blades, all components are blades, 
    /// but multivectors or other k-vectors may not be blades.
    pub fn inner_product(&self, rhs: &Component) -> f64 {
        if self.grade() == rhs.grade() {
            for (b1, b2) in self.bases
            .iter().zip(rhs.bases.iter()) {
                if b1 != b2 { // if any basis mismatch, return 0
                    return 0.0;
                }
                // if they match, make a new component so that the bases 
                // vectors are reordered and consolidated.
                let mut bases = self.bases.clone();
                bases.extend(rhs.bases.clone());
                let temp = Component::new(self.mag * rhs.mag, bases);
                return temp.mag;
            }
        }
        0.0
    }

    /// # Norm squared
    /// 
    /// Norm^2 of a component (a) is equal to a . a
    pub fn norm_sqrd(&self) -> f64 {
        self.inner_product(self)
    }

    /// # Norm
    /// 
    /// As Norm Squared, but includes the square Root.
    /// 
    /// Remember, negative values will return Nan.
    pub fn norm(&self) -> f64 {
        self.inner_product(self).sqrt()
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

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        // add magnitude.
        result.push_str(self.mag.to_string().as_str());
        // add each basis
        for basis in self.bases.iter() {
            result.push_str(basis.to_string().as_str());
        }
        result
    }
}


impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.mag == other.mag && self.bases == other.bases
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
