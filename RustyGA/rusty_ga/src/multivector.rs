use std::ops;

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