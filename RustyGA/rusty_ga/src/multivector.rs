use std::{ops::{self, Index}, collections::{HashSet, btree_map::Values}, sync::Mutex};

use crate::component::{Component, self};

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
    components: Vec<Component>,
}

/// # Zero Multivector
/// 
/// Has No components.
pub const ZERO: Multivector = Multivector { components: vec![] };

impl Multivector {
    /// # New
    /// 
    /// Generates a multivector based on a list of components.
    /// 
    /// When created, it sorts the components by grade before puting them into the
    /// resulting multivector.
    /// 
    /// # Note
    /// 
    /// Components may have duplicate component bases and currently this function
    /// does not automatically consolidate them for efficiency reasons.
    /// 
    /// If you want to ensure that consolidation occurs, use alternative methods
    /// to create this such as addition 
    pub fn new(components: Vec<Component>) -> Multivector {
        let mut components = components
            .clone();
        components.sort_by(|a, b| a.grade().cmp(&b.grade()));
        Multivector { components }
    }

    /// # Length
    /// 
    /// How many components the multivector contains.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// # Is Blade
    /// 
    /// Checks whether this multivector is a blade. IE, it
    /// can be decomposed as the outer product (^) of Vectors.
    /// 
    /// ## Example
    /// 
    /// The multivector 
    /// 
    /// 6p1p2 + 2p2p3
    /// 
    /// can be broken into the vectors
    /// 
    /// 2p1 + 1p2, 3p2 + 2p3
    /// 
    /// ## Logic
    /// 
    /// A Multivector must have a single grade, if it has more than 1, it is
    /// by definition, not a blade, but a sum of k-blades.
    /// 
    /// Likewise, the components of a multivector must overlap such that they all
    /// have at least one basis in common between them. 
    /// 
    /// IE,
    /// The multivector p1p2 + p2p3 is a blade because the two components 
    /// share a basis p2.
    /// 
    /// The multivector p1p2 + p3p4 is not a blade because they do not share
    /// a basis.
    pub fn is_blade(&self) -> bool {
        todo!("Come back here later.");
        let mut grades = HashSet::new();
        for comp in self.components.iter() {
            grades.insert(comp.grade());
        }
        if grades.len() > 1 {
            // if more than 1 grade, then we cannot be a blade.
            return false;
        }
        // get the grade we have to quick check for a vector.
        let mut grade = grades.into_iter().next().unwrap();
        // if of grade 1 or 0, it must be a blade, regardless of space.
        if grade < 2 {
            return true;
        }
        // For items to add together, they must share a basis, connecting all of them together.
        // To ensure they all connect together either directly or indirectly, 
        // we group them together using the groups vec. The idx in group is the
        // idx of the component. 
        // we initialize them to zero, then set the first as group 1
        let mut groups = vec![0; self.components.len()];
        groups[0] = 1;
        for (idx, comp) in self.components.iter().enumerate() {
            for (oidx, other) in self.components.iter().enumerate() {

                if comp.bases().iter().any(|x| other.bases().contains(x)) {

                }
            }
        }
        // if we got past the overlap filter, none are isolate from any o
        true
    }

    /// # Add Component
    /// 
    /// Adds a component to the multivector (nondestructively).
    /// 
    /// If the resulting component is zero, it removes it from the 
    /// resulting multivector.
    pub fn component_add(&self, rhs: &Component) -> Multivector {
        // if the right hand size is zero, then skip adding.
        if rhs.mag == 0.0 {
            return self.clone();
        }
        // get the grade and split off those parts
        let mut result = vec![];
        let mut contracted = false;
        for comp in self.components.iter() {
            if contracted { 
                // if we've already added to a component, just add 
                // to the result and continue.
                result.push(comp.clone());
                continue;
            }
            let temp = comp.force_comp_add(rhs);
            if let Some(val) = temp {
                // if it added, then we have a contraction
                if val != component::ZERO {
                    // only include if it's not a zero component.
                    result.push(val);
                }
                contracted = true;
            } else { // if no value returned, then keep the original.
                result.push(comp.clone());
            }
        }
        if !contracted { // if no addition at any time, add to end.
            result.push(rhs.clone());
        }
        Multivector::new(result)
    }

    /// # Base Add
    /// 
    /// The actual add function.
    /// 
    /// A simple add method. Any like components are combined, 
    /// if they add to 0, they are removed.
    /// 
    /// TODO consider improving to reduce computation cost.
    pub fn multivector_add(&self, rhs: &Multivector) -> Multivector {
        let mut result = self.clone();
        for comp in rhs.components.iter() {
            result = result + comp;
        }
        result
    }

    /// # Scalar Add
    /// 
    /// Multivector Addition between a Multivector and a Scalar value.
    pub fn scalar_add(&self, rhs: &f64) -> Multivector {
        self.component_add(&Component::from_float(rhs))
    }

    /// # Take Grade
    /// 
    /// Takes those parts of a multivector of a particular grade.
    pub fn take_grade(&self, grade: usize) -> Multivector {
        let mut result = vec![];
        for comp in self.components.iter()
        .filter(|x| x.grade() == grade) {
            result.push(comp.clone());
        }
        Multivector::new(result)
    }

    /// # Component Geometric Product
    /// 
    /// Does geometric product between a multivector and a component.
    /// 
    /// Multiplies the component with all components in the multivector.
    pub fn geo_prod_comp(&self, rhs: &Component) -> Multivector {
        let mut result = ZERO;
        for comp in self.components.iter() {
            let temp = comp * rhs;
            result = result + temp;
        }
        result
    }

    /// # Multivector Geometric Product
    /// 
    /// Geometric Product between two multivectors.
    pub fn geo_prod_mv(&self, rhs: &Multivector) -> Multivector {
        let mut accumulator = ZERO;
        for comp in rhs.components.iter() {
            let temp = self.geo_prod_comp(comp);
            accumulator = accumulator + temp;
        }
        accumulator
    }

    pub fn norm_sqrd(&self) -> f64 {
        todo!()
    }

    /// # Scalar Multiplication
    /// 
    /// Exactly what it says. Only speical case is that if rhs is 0.0, then
    /// it returns the Zero Multivector.
    pub fn scalar_mult(&self, rhs: f64) -> Multivector {
        if rhs == 0.0 {
            return ZERO;
        }
        let mut result = vec![];
        for comp in self.components.iter() {
            result.push(comp.clone() * rhs);
        }
        Multivector::new(result)
    }

    /// # Negative 
    /// 
    /// Negates the Multivector.
    pub fn negative(&self) -> Multivector {
        let mut result = vec![];
        for comp in self.components.iter() {
            result.push(-comp);
        }
        Multivector::new(result)
    }

    /// # Float Outer Product
    /// 
    /// Outer Product between a multivector and a scalar.
    /// 
    /// Equivalent to scalar mult.
    pub fn outer_prod_scalar(&self, rhs: f64) -> Multivector {
        self.scalar_mult(rhs)
    }

    /// # Component Outer Product
    /// 
    /// Outer Product between a multivector and a component.
    pub fn outer_prod_comp(&self, rhs: &Component) -> Multivector {
        let mut result = ZERO;
        for comp in self.components.iter() {
            result = result + (comp ^ rhs);
        }
        result
    }

    /// # Multivector Outer Product
    /// 
    /// Takes the Outer Product between two multivectors
    pub fn outer_product_mv(&self, rhs: &Multivector) -> Multivector {
        let mut result = ZERO;
        for lcomp in self.components.iter() {
            result = result + lcomp ^ rhs;
        }
        result
    }

    /// # Multivector Inverse
    /// 
    /// Inverts the multivector, same as taking it to the power -1.
    pub fn inverse(&self) -> Multivector {
        let mut result = ZERO;
        for comp in self.components.iter() {
            result = result + comp.inverse();
        }
        result
    }

    pub fn components(&self) -> &[Component] {
        self.components.as_ref()
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for comp in self.components.iter() {
            result.push_str(comp.to_string().as_str());
            result.push_str(" + ");
        }
        result.truncate(result.len()-3);
        result
    }
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

// Addition
// mv + mv
impl ops::Add for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Self) -> Self::Output {
        self.multivector_add(&rhs)
    }
}
// mv + &mv
impl ops::Add<&Multivector> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        self.multivector_add(&rhs)
    }
}
// &mv + mv
impl ops::Add<Multivector> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        self.multivector_add(&rhs)
    }
}
// &mv + &mv
impl ops::Add<&Multivector> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        self.multivector_add(&rhs)
    }
}

// mv + comp
impl ops::Add<Component> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Component) -> Self::Output {
        self.component_add(&rhs)
    }
}
// &mv + comp
impl ops::Add<Component> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: Component) -> Self::Output {
        self.component_add(&rhs)
    }
}
// mv + &comp
impl ops::Add<&Component> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Component) -> Self::Output {
        self.component_add(rhs)
    }
}
// &mv + &comp
impl ops::Add<&Component> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Component) -> Self::Output {
        self.component_add(rhs)
    }
}

// comp + mv
impl ops::Add<Multivector> for Component {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        rhs.component_add(&self)
    }
}
// &comp + mv
impl ops::Add<Multivector> for &Component {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        rhs.component_add(&self)
    }
}
// comp + &mv
impl ops::Add<&Multivector> for Component {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        rhs.component_add(&self)
    }
}
// &comp + &mv
impl ops::Add<&Multivector> for &Component {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        rhs.component_add(&self)
    }
}

// mv  + scalar
impl ops::Add<f64> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Self::Output {
        self.scalar_add(&rhs)
    }
}
// &mv + scalar
impl ops::Add<f64> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: f64) -> Self::Output {
        self.scalar_add(&rhs)
    }
}
// mv  + &scalar
impl ops::Add<&f64> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: &f64) -> Self::Output {
        self.scalar_add(rhs)
    }
}
// &mv + &scalar
impl ops::Add<&f64> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: &f64) -> Self::Output {
        self.scalar_add(rhs)
    }
}

// scalar  + mv 
impl ops::Add<Multivector> for f64 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        rhs.scalar_add(&self)
    }
}
// scalar  + &mv
impl ops::Add<&Multivector> for f64 {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        rhs.scalar_add(&self)
    }
}
// &scalar + mv 
impl ops::Add<Multivector> for &f64 {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        rhs.scalar_add(&self)
    }
}
// &scalar + &mv
impl ops::Add<&Multivector> for &f64 {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        rhs.scalar_add(&self)
    }
}

// subtraction
// mv - mv
impl ops::Sub for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.multivector_add(&-rhs)
    }
}
// mv - &mv
impl ops::Sub<&Multivector> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        self.multivector_add(&-rhs)
    }
}
// &mv - mv
impl ops::Sub<Multivector> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        self.multivector_add(&-rhs)
    }
}
// &mv - &mv
impl ops::Sub<&Multivector> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        self.multivector_add(&-rhs)
    }
}

// mv - comp
impl ops::Sub<Component> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Component) -> Self::Output {
        self.component_add(&-rhs)
    }
}
// &mv - comp
impl ops::Sub<Component> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Component) -> Self::Output {
        self.component_add(&-rhs)
    }
}
// mv - &comp
impl ops::Sub<&Component> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.component_add(&-rhs)
    }
}
// &mv - &comp
impl ops::Sub<&Component> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.component_add(&-rhs)
    }
}

// comp - mv
impl ops::Sub<Multivector> for Component {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        -rhs.component_add(&-self)
    }
}
// &comp - mv
impl ops::Sub<Multivector> for &Component {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        -rhs.component_add(&-self)
    }
}
// comp - &mv
impl ops::Sub<&Multivector> for Component {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        -rhs.component_add(&-self)
    }
}
// &comp - &mv
impl ops::Sub<&Multivector> for &Component {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        -rhs.component_add(&-self)
    }
}

// mv - scalar
impl ops::Sub<f64> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Self::Output {
        self.scalar_add(&-rhs)
    }
}
// &mv - scalar
impl ops::Sub<f64> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: f64) -> Self::Output {
        self.scalar_add(&-rhs)
    }
}
// mv - &scalar
impl ops::Sub<&f64> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &f64) -> Self::Output {
        self.scalar_add(&-rhs)
    }
}
// &mv - &scalar
impl ops::Sub<&f64> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &f64) -> Self::Output {
        self.scalar_add(&-rhs)
    }
}

// scalar - mv
impl ops::Sub<Multivector> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        -rhs + self
    }
}
// &scalar - mv
impl ops::Sub<Multivector> for &f64 {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        -rhs + self
    }
}
// scalar - &mv
impl ops::Sub<&Multivector> for f64 {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        -rhs + self
    }
}
// &scalar - &mv
impl ops::Sub<&Multivector> for &f64 {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        -rhs + self
    }
}

// scalar multiplication
// f64  * mv
impl ops::Mul<f64> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scalar_mult(rhs)
    }
}
// &f64 * mv
impl ops::Mul<&f64> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &f64) -> Self::Output {
        self.scalar_mult(*rhs)
    }
}
// f64  * &mv
impl ops::Mul<f64> for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: f64) -> Self::Output {
        self.scalar_mult(rhs)
    }
}
// &f64 * &mv
impl ops::Mul<&f64> for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &f64) -> Self::Output {
        self.scalar_mult(*rhs)
    }
}

// mv  * f64 
impl ops::Mul<Multivector> for f64 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Self::Output {
        rhs.scalar_mult(self)
    }
}
// mv  * &f64
impl ops::Mul<Multivector> for &f64 {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Self::Output {
        rhs.scalar_mult(*self)
    }
}
// &mv * f64 
impl ops::Mul<&Multivector> for f64 {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        rhs.scalar_mult(self)
    }
}
// &mv * *f64
impl ops::Mul<&Multivector> for &f64 {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        rhs.scalar_mult(*self)
    }
}

// Component Mult
// comp * mv
impl ops::Mul<Multivector> for Component {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Self::Output {
        self.geo_product_mv(&rhs)
    }
}
// &comp * mv
impl ops::Mul<Multivector> for &Component {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Self::Output {
        self.geo_product_mv(&rhs)
    }
}
// comp * &mv
impl ops::Mul<&Multivector> for Component {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        self.geo_product_mv(rhs)
    }
}
// &comp * &mv
impl ops::Mul<&Multivector> for &Component {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        self.geo_product_mv(rhs)
    }
}

// mv * comp
impl ops::Mul<Component> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Component) -> Self::Output {
        self.geo_prod_comp(&rhs)
    }
}
// &mv * comp
impl ops::Mul<Component> for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Component) -> Self::Output {
        self.geo_prod_comp(&rhs)
    }
}
// mv * &comp
impl ops::Mul<&Component> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &Component) -> Self::Output {
        self.geo_prod_comp(&rhs)
    }
}
// &mv * &comp
impl ops::Mul<&Component> for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &Component) -> Self::Output {
        self.geo_prod_comp(&rhs)
    }
}

// Geometric Product
// mv * mv
impl ops::Mul<Multivector> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Self::Output {
        self.geo_prod_mv(&rhs)
    }
}
// &mv * mv
impl ops::Mul<&Multivector> for Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        self.geo_prod_mv(rhs)
    }
}
// mv * &mv
impl ops::Mul<Multivector> for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: Multivector) -> Self::Output {
        self.geo_prod_mv(&rhs)
    }
}
// &mv * &mv
impl ops::Mul<&Multivector> for &Multivector {
    type Output = Multivector;

    fn mul(self, rhs: &Multivector) -> Self::Output {
        self.geo_prod_mv(rhs)
    }
}

// scalar outer product
// f64 ^ mv
impl ops::BitXor<Multivector> for f64 {
    type Output = Multivector;

    fn bitxor(self, rhs: Multivector) -> Self::Output {
        rhs.scalar_mult(self)
    }
}
// &f64 ^ mv
impl ops::BitXor<Multivector> for &f64 {
    type Output = Multivector;

    fn bitxor(self, rhs: Multivector) -> Self::Output {
        rhs.scalar_mult(*self)
    }
}
// f64 ^ &mv
impl ops::BitXor<&Multivector> for f64 {
    type Output = Multivector;

    fn bitxor(self, rhs: &Multivector) -> Self::Output {
        rhs.scalar_mult(self)
    }
}
// &f64 ^ &mv
impl ops::BitXor<&Multivector> for &f64 {
    type Output = Multivector;

    fn bitxor(self, rhs: &Multivector) -> Self::Output {
        rhs.scalar_mult(*self)
    }
}

// mv ^ f64
impl ops::BitXor<f64> for Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: f64) -> Self::Output {
        self.scalar_mult(rhs)
    }
}
// &mv ^ f64
impl ops::BitXor<f64> for &Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: f64) -> Self::Output {
        self.scalar_mult(rhs)
    }
}
// mv ^ &f64
impl ops::BitXor<&f64> for Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: &f64) -> Self::Output {
        self.scalar_mult(*rhs)
    }
}
// &mv ^ &f64
impl ops::BitXor<&f64> for &Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: &f64) -> Self::Output {
        self.scalar_mult(*rhs)
    }
}

// component outer product
// comp ^ mv
impl ops::BitXor<Multivector> for Component {
    type Output = Multivector;

    fn bitxor(self, rhs: Multivector) -> Self::Output {
        (-rhs).outer_prod_comp(&self)
    }
}
// &comp ^ mv
impl ops::BitXor<Multivector> for &Component {
    type Output = Multivector;

    fn bitxor(self, rhs: Multivector) -> Self::Output {
        (-rhs).outer_prod_comp(&self)
    }
}
// comp ^ &mv
impl ops::BitXor<&Multivector> for Component {
    type Output = Multivector;

    fn bitxor(self, rhs: &Multivector) -> Self::Output {
        (-rhs).outer_prod_comp(&self)
    }
}
// &comp ^ &mv
impl ops::BitXor<&Multivector> for &Component {
    type Output = Multivector;

    fn bitxor(self, rhs: &Multivector) -> Self::Output {
        (-rhs).outer_prod_comp(&self)
    }
}

// mv ^ comp
impl ops::BitXor<Component> for Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: Component) -> Self::Output {
        self.outer_prod_comp(&rhs)
    }
}
// &mv ^ comp
impl ops::BitXor<Component> for &Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: Component) -> Self::Output {
        self.outer_prod_comp(&rhs)
    }
}
// mv ^ &comp
impl ops::BitXor<&Component> for Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: &Component) -> Self::Output {
        self.outer_prod_comp(&rhs)
    }
}
// &mv ^ &comp
impl ops::BitXor<&Component> for &Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: &Component) -> Self::Output {
        self.outer_prod_comp(&rhs)
    }
}

// multivector outer product
// mv ^ mv
impl ops::BitXor<Multivector> for Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: Multivector) -> Self::Output {
        self.outer_product_mv(&rhs)
    }
}
// &mv ^ mv
impl ops::BitXor<&Multivector> for Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: &Multivector) -> Self::Output {
        self.outer_product_mv(&rhs)
    }
}
// mv ^ &mv
impl ops::BitXor<Multivector> for &Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: Multivector) -> Self::Output {
        self.outer_product_mv(&rhs)
    }
}
// &mv ^ &mv
impl ops::BitXor<&Multivector> for &Multivector {
    type Output = Multivector;

    fn bitxor(self, rhs: &Multivector) -> Self::Output {
        self.outer_product_mv(&rhs)
    }
}

// Negative
impl ops::Neg for Multivector {
    type Output = Multivector;

    fn neg(self) -> Self::Output {
        self.negative()
    }
}

impl ops::Neg for &Multivector {
    type Output = Multivector;

    fn neg(self) -> Self::Output {
        self.negative()
    }
}