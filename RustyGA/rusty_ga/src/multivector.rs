use std::{ops::{self, Index}, collections::{HashSet, btree_map::Values}};

use crate::component::Component;

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
    pub fn new(components: Vec<Component>) -> Multivector {
        let mut components = components
            .clone();
        components.sort_by(|a, b| a.grade().cmp(&b.grade()));
        // sanity check if there are components with duplicate bases.
        for comp in components.iter() {
            
        }
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
    pub fn add_component(&self, rhs: &Component) -> Multivector {
        // get the grade and split off those parts
        let rhsgrade = rhs.grade();
        let mut lhsgrades = HashSet::new();
        for comp in self.components.iter() {
            lhsgrades.insert(comp.grade());
        }
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
                result.push(val);
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
    pub fn base_add(&self, rhs: &Multivector) -> Multivector {
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
        self.add_component(&Component::from_float(rhs))
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

    pub fn norm_sqrd(&self) -> f64 {
        todo!()
    }

    pub fn scalar_mult(&self, rhs: f64) -> Multivector {
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

// Addition

// mv + mv
impl ops::Add for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Self) -> Self::Output {
        self.base_add(&rhs)
    }
}
// mv + &mv
impl ops::Add<&Multivector> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        self.base_add(&rhs)
    }
}
// &mv + mv
impl ops::Add<Multivector> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        self.base_add(&rhs)
    }
}
// &mv + &mv
impl ops::Add<&Multivector> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        self.base_add(&rhs)
    }
}

// mv + comp
impl ops::Add<Component> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: Component) -> Self::Output {
        self.add_component(&rhs)
    }
}
// &mv + comp
impl ops::Add<Component> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: Component) -> Self::Output {
        self.add_component(&rhs)
    }
}
// mv + &comp
impl ops::Add<&Component> for Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Component) -> Self::Output {
        self.add_component(rhs)
    }
}
// &mv + &comp
impl ops::Add<&Component> for &Multivector {
    type Output = Multivector;

    fn add(self, rhs: &Component) -> Self::Output {
        self.add_component(rhs)
    }
}


// comp + mv
impl ops::Add<Multivector> for Component {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        rhs.add_component(&self)
    }
}
// &comp + mv
impl ops::Add<Multivector> for &Component {
    type Output = Multivector;

    fn add(self, rhs: Multivector) -> Self::Output {
        rhs.add_component(&self)
    }
}
// comp + &mv
impl ops::Add<&Multivector> for Component {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        rhs.add_component(&self)
    }
}
// &comp + &mv
impl ops::Add<&Multivector> for &Component {
    type Output = Multivector;

    fn add(self, rhs: &Multivector) -> Self::Output {
        rhs.add_component(&self)
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
        self.base_add(&-rhs)
    }
}
// mv - &mv
impl ops::Sub<&Multivector> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        self.base_add(&-rhs)
    }
}
// &mv - mv
impl ops::Sub<Multivector> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        self.base_add(&-rhs)
    }
}
// &mv - &mv
impl ops::Sub<&Multivector> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        self.base_add(&-rhs)
    }
}

// mv - comp
impl ops::Sub<Component> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Component) -> Self::Output {
        self.add_component(&-rhs)
    }
}
// &mv - comp
impl ops::Sub<Component> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: Component) -> Self::Output {
        self.add_component(&-rhs)
    }
}
// mv - &comp
impl ops::Sub<&Component> for Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.add_component(&-rhs)
    }
}
// &mv - &comp
impl ops::Sub<&Component> for &Multivector {
    type Output = Multivector;

    fn sub(self, rhs: &Component) -> Self::Output {
        self.add_component(&-rhs)
    }
}

// comp - mv
impl ops::Sub<Multivector> for Component {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        -rhs.add_component(&-self)
    }
}
// &comp - mv
impl ops::Sub<Multivector> for &Component {
    type Output = Multivector;

    fn sub(self, rhs: Multivector) -> Self::Output {
        -rhs.add_component(&-self)
    }
}
// comp - &mv
impl ops::Sub<&Multivector> for Component {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        -rhs.add_component(&-self)
    }
}
// &comp - &mv
impl ops::Sub<&Multivector> for &Component {
    type Output = Multivector;

    fn sub(self, rhs: &Multivector) -> Self::Output {
        -rhs.add_component(&-self)
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