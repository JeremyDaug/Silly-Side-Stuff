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
const ZERO: Multivector = Multivector { components: vec![] };

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
        Multivector { components }
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
        let mut lhscomps = self.components.clone();
        let mut editedidx = usize::MAX;
        for (idx, comp) in lhscomps.iter_mut()
        .filter(|x| x.grade() == rhsgrade)
        .enumerate() {
            if comp.bases() == rhs.bases() {
                comp.mag += rhs.mag;
                editedidx = idx;
                break;
            }
        }
        if editedidx == usize::MAX { // if not added to anything, push to the end.
            lhscomps.push(rhs.clone());
        } else { // if added to something, check to see if it went to 0.
            if lhscomps[editedidx].mag == 0.0 {
                lhscomps.remove(editedidx);
            }
        }
        Multivector { components: lhscomps }
    }

    /// # Base Add
    /// 
    /// The actual add function.
    /// 
    /// A simple add method. Any like components are combined, 
    /// if they add to 0, they are removed.
    pub fn base_add(&self, rhs: &Multivector) -> Multivector {
        let mut result = ZERO;
        for comp in rhs.components.iter() {
            result = result + rhs;
        }
        result
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