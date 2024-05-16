use std::{ops::{self, Add}, collections::HashSet};

use regex::Regex;

use crate::{basis::ONBasis, multivector::{Multivector, self}};

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

/// # Zero Component
///
/// Returns a component with a magnitude of 0.0 and no bases.
pub const ZERO: Component = Component { mag: 0.0, bases: vec![] };

impl Component {
    /// # Default
    /// 
    /// Returns a default (zero) component.
    pub fn default() -> Component {
        ZERO
    }

    /// # Grade
    ///
    /// Retieves what the grade of the component is, IE how many bases it has.
    pub fn grade(&self) -> usize {
        self.bases.len()
    }

    /// # Bases
    /// 
    /// Gets a reference to the component's bases.
    pub fn bases(&self) -> &[ONBasis] {
        self.bases.as_ref()
    }

    /// # New
    ///
    /// creates a new component based on a magnitude and basis.
    ///
    /// If basis is not ordered correctly, it reorders the bases appropriately.
    pub fn new(mag: f64, bases: Vec<ONBasis>) -> Component {
        if bases.len() > 0 {
            for idx in 0..(bases.len()-1) {
                // if any disorder or duplicates, make then reorder bases immediately.
                if bases[idx].unwrap() >= bases[idx+1].unwrap() {
                    return Component {mag, bases}.reorder_bases();
                }
            }
        }
        Component { mag, bases }
    }

    /// # Reorder Bases
    ///
    /// Creates a copy of the component with it's basis vectors organized 
    /// properly. Deals with duplicates as well.
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

    /// # Force Component Addition
    ///
    /// Adds two components if they share the same basis vectors.
    ///
    /// Useful for adding components but maintaining the component struct.
    ///
    /// If components don't add at all, it returns None. If they do add
    /// it returns the result.
    pub fn force_comp_add(&self, rhs: &Component) -> Option<Component> {
        // if the same length, organize their bases
        if self.bases.len() == rhs.bases.len() {
            // if same length, check that each basis is the same
            for idx in 0..self.bases.len() {
                if self.bases[idx] != rhs.bases[idx] {
                    return None; // if any mismatch, return 0.
                }
            }
            // if bases match, then add magnitudes
            let res = Component { mag: self.mag + rhs.mag,
                bases: self.bases.clone()};
            // if magnitude is zero, then return zero component for simplicity reasons.
            return if res.mag == 0.0 { Some(ZERO) } else { Some(res) };
        } else {
            None
        }
    }

    /// # To Multivector
    ///
    /// A quick conversion from Component to Multivector.
    pub fn to_mv(&self) -> Multivector {
        Multivector::new(vec![self.clone()])
    }

    /// # Standard Addition
    ///
    /// Adds two components and always results in a multivector.
    ///
    /// If the components share the same basis, the resulting Multivector
    /// will only have a singlular component.
    pub fn std_comp_add(&self, rhs: &Component) -> Multivector {
        let temp: Multivector = self.to_mv();
        temp.component_add(rhs)
    }

    /// # Scalar Multiplication
    ///
    /// Multiplies the component by a scalar.
    pub fn scalar_mult(&self, rhs: f64) -> Component {
        Component::new(self.mag * rhs, self.bases.clone())
    }

    /// # Geometric Product
    ///
    /// Multiplies two components together via Geometric Product.
    ///
    /// Bases are comibned directly, reordered, then if any squares,
    /// those are applied.
    ///
    /// If any results in the result being 0, then it shortcuts out.
    pub fn geo_product(&self, rhs: &Component) -> Component {
        let res_mag = self.mag * rhs.mag;
        if res_mag == 0.0 {
            return ZERO;
        }
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

    /// # Norm squared
    ///
    /// Norm^2 of a vector (a) is equal to a . a
    ///
    /// We peek ahead a bit and for any blade define
    /// the norm^2 as A . A.inverse()
    ///
    /// This may be reworked in the future to be more
    /// efficient as it creates a new component, and applies
    /// reversion to it, just so it can drop it.
    pub fn norm_sqrd(&self) -> f64 {
        self.inner_product_f64(&self.reversion())
    }

    /// # Norm
    ///
    /// As Norm Squared, but includes the square Root.
    ///
    /// Remember, negative values will return Nan.
    pub fn norm(&self) -> f64 {
        self.inner_product_f64(self).sqrt()
    }

    /// # Involution
    ///
    /// Involutes the the component based on it's grade.
    /// +-+-+-+-
    pub fn involution(&self) -> Component {
        Component::new(self.mag * (-1.0_f64).powf(self.grade() as f64) , self.bases.clone())
    }

    /// # Scalar Product
    ///
    /// Scalar Product multiplies blades of like grade. Since all components
    /// are blades, they always go. If the blades are of different grades
    /// it returns 0. If they are of the same it takes the determinant of them
    /// if they are both scalars, it multiplies them.
    pub fn scalar_product(&self, rhs: &Component) -> f64 {
        let result = self * rhs;
        if result.grade() == 0 { // geometric product, select grade 0 result.
            result.mag
        } else {
            0.0
        }
    }

    /// # Inverse
    ///
    /// Returns the standardized Inverse of this component.
    ///
    /// # Note
    ///
    /// Blades with Degenerate bases (bases^2 = 0) do not have
    /// an inverse value.
    pub fn inverse(&self) -> Component {
        let rev = self.reversion();
        let norm = self.norm_sqrd();
        let result = rev / norm;
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
        // shortcircuit if self.grade > rhs.grade
        if self.grade() > rhs.grade() {
            return ZERO;
        }
        // Geometric Product, select grade rhs.grade - self.grade
        let result = self * rhs;
        if result.grade() == (rhs.grade() - self.grade()) {
            result
        } else {
            ZERO
        }
    }

    /// # Right Contraction
    ///
    /// Same as left, but reversed.
    pub fn right_cont(&self, rhs: &Component) -> Component {
        // shortcircuit if rhs.grade > self.grade
        if self.grade() < rhs.grade() {
            return  ZERO;
        }
        // otherwise geometric product, select grade self.grade - rhs.grade
        let result = self * rhs;
        if result.grade() == (self.grade() - rhs.grade()) {
            result
        } else {
            ZERO
        }
    }

    /// # Inner Product
    ///
    /// Performs a inner/dot product, but only returns when the grades are equal.
    ///
    /// Returns 0.0 if there is any mismatched basis.
    ///
    /// Note: This is only meant for blades, all components are blades,
    /// but multivectors or other k-vectors may not be blades.
    pub fn inner_product_f64(&self, rhs: &Component) -> f64 {
        let result = self * rhs;
        if result.grade() > 0 {
            0.0
        } else {
            result.mag
        }
    }

    /// # Component Product
    /// 
    /// Performs an inner/dot product, but only returns a value if the grade 
    /// resulting is zero.
    /// 
    /// This function forces the result to be a component instead of a float.
    /// 
    /// Note: This is only meant for blades, all components are blades,
    /// but multivectors or other k-vectors may not be blades.
    pub fn inner_product(&self, rhs: &Component) -> Component {
        let result = self * rhs;
        if result.grade() > 0 {
            ZERO
        } else {
            result
        }
    }

    /// # Dualization
    ///
    /// Returns the Dual of this component.
    ///
    /// Must be given the Pseudoscalar of the geometry.
    ///
    /// The Dual of a Dual is not always the same as the original blade, as such
    /// there is also an Undual function.
    ///
    /// The dual of the Dual results in an alterating pattern dependent on the
    /// dimensions of the space. ++--++--
    /// 0, 1, 4,5 ... A = A.dual().dual()
    /// 2,3, 6,7 ... -A = A.dual().dual()
    pub fn dual(&self, i: &Component) -> Component {
        self << &i.inverse()
    }

    /// # Undualization
    ///
    /// Depending on the geometry, the Dual of a Dual is not always the same
    /// as the original.
    ///
    /// As such, Undualization guarantees the property that for a blade A,
    /// with pseudoscalar i that
    ///
    /// A = A.dual(i).undual(i)
    ///
    /// in all cases.
    pub fn undual(&self, i: &Component) -> Component {
        self << i
    }

    /// # Project Onto
    ///
    /// Projects the component blade onto another component.
    ///
    /// Currently does not work for degenerate dimensions due to the basis
    /// squaring to 0.
    pub fn project_onto(&self, rhs: &Component) -> Component {
        self << rhs.inverse() << rhs
    }

    /// # Reciprocal Frame
    ///
    /// Treats the current component as a pseudoscalar of a subspace
    /// then returns the given reciprocal bases.
    ///
    /// IE, if the component is B = b1 ^ b2 ^ b3 ^ b4
    /// then B.reciprocal_frame(2) = - b1 ^ b3 ^ b4 << B.inverse
    ///
    /// Returns zero if the vector selected is beyond our array.
    pub fn reciprocal_frame(&self, i: usize) -> Component {
        if i >= self.bases.len() {
            ZERO
        } else {
            let mut rep_bas = self.bases.clone();
            rep_bas.remove(i);
            (-1.0_f64).powi(i as i32) * Component::new(self.mag,
                rep_bas) << self.inverse()
        }
    }

    /// # Same Bases
    ///
    /// Checks if two components share the exact same bases, returns false if
    /// not.
    pub fn same_bases(&self, other: Component) -> bool {
        if self.bases.len() != other.bases.len() {
            return false;
        }
        for basis in self.bases.iter() {
            if !other.bases.contains(&basis) {
                return false;
            }
        }
        return true;
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

    /// # From Float
    ///
    /// Creates a grade 0 component from a float.
    pub fn from_float(rhs: &f64) -> Component {
        Component::new(*rhs, vec![])
    }

    /// # Geometric Product between Multivectors
    /// 
    /// Takes the Geometric product between a component and a Multivector with
    /// the Component on the left and multivector on the right.
    pub fn geo_product_mv(&self, rhs: &Multivector) -> Multivector {
        let mut result = multivector::ZERO;
        for comp in rhs.components().iter() {
            result = result + self * comp;
        }
        result
    }

    /// # From String
    /// 
    /// Translates a string into a Component. Returns an Err result if invalid
    /// formatting.
    /// 
    /// All of them are in the format of #.#B(id)B(id)
    pub fn from_string(val: &String) -> Result<Component, String> {
        let COMPONENT_REGEX: &str = r"(?<val>[+-]\d*[.]?\d*)"; //(?<b>(?<e>[PNZ])\((?<id>0|[1-9][0-9]*)\))";
        let re = Regex::new(COMPONENT_REGEX).unwrap();
        //let Some(caps) = re.captures(val) else {Err};
        // get value from the first, checking to ensure nothing is wrong.
        if val.len() == 0 {
            return Ok(ZERO);
        }
        let mut whole = String::new();
        let mut decimal_found = false;
        let mut decimal = String::new();
        let mut bases = String::new();
        for c in val.chars() {
            if c.is_ascii_digit() ||
            c == '-' ||
            c == '+' {
                if !decimal_found {
                    whole.push(c);
                } else { // past decimal
                    decimal.push(c);
                }
            } else if c == '.' { // if decimal
                if decimal_found {
                    return Err(String::from("Extra Decimal Found"));
                } else {
                    decimal_found = true;
                }
            } else { // if character found should be the start of the bases.
                let prefix = if decimal.len() > 0 {
                    whole.len() + 1 + decimal.len()
                } else { whole.len() };
                bases = val.clone().split_at(prefix).1.to_string();
                break;
            }
        }
        let whole_len = whole.len();
        let num_res = whole + "." + if decimal.len() > 0 { decimal.as_str() } else { "0" };
        let poss_number = num_res.parse::<f64>();
        let number = if let Ok(num) = poss_number {
            if whole_len == 0 && decimal.len() == 0 {
                1.0
            } else if num_res.get(0..1).unwrap() == "-" &&
            bases.len() > 0 { // if just negative and basis, -1.0
                -1.0
            } else if num_res.get(0..1).unwrap() == "+" &&
            bases.len() > 0 { // if just + and basis, 
                1.0
            } else {
                num
            }
        } else {
            return Err(String::from("Invalid number given."));
        };

        // with number gotten, break up the bases and convert them.
        let mut split_bases = vec![];
        for section in bases.split(")") {
            if section == "" {
                break; // if empty, then last basis past, gtfo
            }
            let basis = String::from(section) + ")";
            let res = ONBasis::from_string(&basis);
            if let Ok(b) = res {
                split_bases.push(b);
            } else if let Err(e) = res {
                return Err(e);
            }
        }
        // we've done all the work, return the result.
        Ok(Component::new(number, split_bases))
    }
}


impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.mag == other.mag && self.bases == other.bases
    }
}

// scalar division

// &comp / f64
impl ops::Div<f64> for &Component {
    type Output = Component;

    fn div(self, rhs: f64) -> Self::Output {
        self.scalar_mult(1.0 / rhs)
    }
}

// comp / f64
impl ops::Div<f64> for Component {
    type Output = Component;

    fn div(self, rhs: f64) -> Self::Output {
        self.scalar_mult(1.0 / rhs)
    }
}

// Left Contraction <<

// real + real
impl ops::Shl for Component {
    type Output = Component;

    /// # Left Contraction
    ///
    /// Removes the left side bases from the right side.
    /// If lhs.grade() > rhs.grade(), then it returns 0.
    /// if lhs.grade() <= rhs.grade(),
    /// then it returns a component of grade rhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shl(self, rhs: Self) -> Self::Output {
        self.left_cont(&rhs)
    }
}

// ref + ref
impl ops::Shl<&Component> for &Component {
    type Output = Component;

    /// # Left Contraction
    ///
    /// Removes the left side bases from the right side.
    /// If lhs.grade() > rhs.grade(), then it returns 0.
    /// if lhs.grade() <= rhs.grade(),
    /// then it returns a component of grade rhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shl(self, rhs: &Component) -> Self::Output {
        self.left_cont(rhs)
    }
}

// ref + real
impl ops::Shl<Component> for &Component {
    type Output = Component;

    /// # Left Contraction
    ///
    /// Removes the left side bases from the right side.
    /// If lhs.grade() > rhs.grade(), then it returns 0.
    /// if lhs.grade() <= rhs.grade(),
    /// then it returns a component of grade rhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shl(self, rhs: Component) -> Self::Output {
        self.left_cont(&rhs)
    }
}

// real + ref
impl ops::Shl<&Component> for Component {
    type Output = Component;

    /// # Left Contraction
    ///
    /// Removes the left side bases from the right side.
    /// If lhs.grade() > rhs.grade(), then it returns 0.
    /// if lhs.grade() <= rhs.grade(),
    /// then it returns a component of grade rhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shl(self, rhs: &Component) -> Self::Output {
        self.left_cont(rhs)
    }
}

// Right Contraction >>

// real + real
impl ops::Shr for Component {
    type Output = Component;

    /// # Right Contraction
    ///
    /// Removes the right side bases from the left side.
    /// If lhs.grade() < rhs.grade(), then it returns 0.
    /// if lhs.grade() >= rhs.grade(),
    /// then it returns a component of grade lhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shr(self, rhs: Self) -> Self::Output {
        self.right_cont(&rhs)
    }
}

// ref + ref
impl ops::Shr<&Component> for &Component {
    type Output = Component;

    /// # Right Contraction
    ///
    /// Removes the right side bases from the left side.
    /// If lhs.grade() < rhs.grade(), then it returns 0.
    /// if lhs.grade() >= rhs.grade(),
    /// then it returns a component of grade lhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shr(self, rhs: &Component) -> Self::Output {
        self.right_cont(rhs)
    }
}

// ref + real
impl ops::Shr<Component> for &Component {
    type Output = Component;

    /// # Right Contraction
    ///
    /// Removes the right side bases from the left side.
    /// If lhs.grade() < rhs.grade(), then it returns 0.
    /// if lhs.grade() >= rhs.grade(),
    /// then it returns a component of grade lhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shr(self, rhs: Component) -> Self::Output {
        self.right_cont(&rhs)
    }
}

// real + ref
impl ops::Shr<&Component> for Component {
    type Output = Component;

    /// # Right Contraction
    ///
    /// Removes the right side bases from the left side.
    /// If lhs.grade() < rhs.grade(), then it returns 0.
    /// if lhs.grade() >= rhs.grade(),
    /// then it returns a component of grade lhs.grade() - rhs.grade()
    ///
    /// Scalar values multiply.
    fn shr(self, rhs: &Component) -> Self::Output {
        self.right_cont(rhs)
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

// Addition

// real + real
impl ops::Add for Component {
    type Output = Multivector;

    fn add(self, rhs: Self) -> Self::Output {
        self.std_comp_add(&rhs)
    }
}

// ref + ref
impl ops::Add<&Component> for &Component {
    type Output = Multivector;

    fn add(self, rhs: &Component) -> Self::Output {
        self.std_comp_add(rhs)
    }
}

// ref + real
impl ops::Add<Component> for &Component {
    type Output = Multivector;

    fn add(self, rhs: Component) -> Self::Output {
        self.std_comp_add(&rhs)
    }
}

// real + ref
impl ops::Add<&Component> for Component {
    type Output = Multivector;

    fn add(self, rhs: &Component) -> Self::Output {
        self.std_comp_add(rhs)
    }
}

// Into Multivector
impl Into<Multivector> for Component {
    fn into(self) -> Multivector {
        Multivector::new(vec![self.clone()])
    }
}

// Subtraction

// real + real
impl ops::Sub for Component {
    type Output = Multivector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.std_comp_add(&-rhs)
    }
}

// ref + ref
impl ops::Sub<&Component> for &Component {
    type Output = Multivector;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.std_comp_add(&-rhs)
    }
}

// ref + real
impl ops::Sub<Component> for &Component {
    type Output = Multivector;

    fn sub(self, rhs: Component) -> Self::Output {
        self.std_comp_add(&-rhs)
    }
}

// real + ref
impl ops::Sub<&Component> for Component {
    type Output = Multivector;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.std_comp_add(&-rhs)
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
