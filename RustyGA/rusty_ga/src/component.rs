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
#[derive(Debug)]
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
            unique.insert(basis.id);
        }
        // if duplicate ids, cleare out duplicate bases.
        if unique.len() != bases.len() {
            return Component {mag, bases}.reorder_bases();
        }
        for idx in 0..(bases.len()-1) {
            // if any disorder, make then reorder bases immediately.
            if bases[idx].id > bases[idx+1].id {
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
                if result.bases[current].id == result.bases[current+1].id {
                    let basis = result.bases.remove(current);
                    result.bases.remove(current);
                    result.mag *= basis.sqr;
                    change = true;
                    // if after the removal we would step out of the list, get out of the current loop.
                    if current+1 >= result.bases.len() {
                        break;
                    }
                }
                // swap them if the two are out of order.
                if result.bases[current].id > result.bases[current+1].id {
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
                if self.bases[idx].id != rhs.bases[idx].id {
                    return ZERO; // if any mismatch, return 0.
                }
            }
            // if bases match, then add magnitudes
            return Component { mag: self.mag + rhs.mag, bases: self.bases.clone()};
        } else {
            ZERO
        }
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
    /// basis produce a zero
    /// 
    pub fn outer_product(&self, rhs: &Component) -> Component {
        // check for overlapping bases
        let mut uniques = HashSet::new();
        for basis in self.bases.iter() {
            uniques.insert(basis.id);
        }
        for basis in rhs.bases.iter() {
            uniques.insert(basis.id);
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

// Multiplication

// real + real
impl ops::Mul for Component {
    type Output = Component;

    fn mul(self, rhs: Self) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// ref + ref
impl ops::Mul<&Component> for &Component {
    type Output = Component;

    fn mul(self, rhs: &Component) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// ref + real
impl ops::Mul<Component> for &Component {
    type Output = Component;

    fn mul(self, rhs: Component) -> Self::Output {
        self.comp_add(&-rhs)
    }
}

// real + ref
impl ops::Mul<&Component> for Component {
    type Output = Component;

    fn mul(self, rhs: &Component) -> Self::Output {
        self.comp_add(&-rhs)
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

mod reorder_bases_should {
    use crate::basis::ONBasis;

    use super::Component;

    #[test]
    pub fn correctly_square_bases() {
        let ep = ONBasis {
            id: 1,
            name: String::new(),
            sqr: 1.0,
        };
        let e0 = ONBasis {
            id: 2,
            name: String::new(),
            sqr: 0.0,
        };
        let em = ONBasis {
            id: 3,
            name: String::new(),
            sqr: -1.0,
        };

        // check positive
        let c1 = Component::new(2.0, vec![ep.clone(), ep.clone()]);
        assert_eq!(c1.mag, 2.0);
        assert_eq!(c1.bases.len(), 0);

        // check negative
        let c1 = Component::new(2.0, vec![em.clone(), em.clone()]);
        assert_eq!(c1.mag, -2.0);
        assert_eq!(c1.bases.len(), 0);

        // check zero
        let c1 = Component::new(2.0, vec![e0.clone(), e0.clone()]);
        assert_eq!(c1.mag, 0.0);
        assert_eq!(c1.bases.len(), 0);
    }

    #[test]
    pub fn correctly_reorder_bases() {
        let e1 = ONBasis {
            id: 1,
            name: String::new(),
            sqr: 1.0,
        };
        let e2 = ONBasis {
            id: 2,
            name: String::new(),
            sqr: 1.0,
        };
        let e3 = ONBasis {
            id: 3,
            name: String::new(),
            sqr: 1.0,
        };

        // Pseudoscalar * Pseudoscalar
        let c1 = Component::new(1.0, vec![e1.clone(), e2.clone(), e3.clone(), 
            e1.clone(), e2.clone(), e3.clone()]);
        assert_eq!(c1.mag, -1.0);
    }

    #[test]
    pub fn correctly_reorder_and_flip_sign() {
        let e1 = ONBasis {
            id: 1,
            name: String::new(),
            sqr: 1.0,
        };
        let e2 = ONBasis {
            id: 2,
            name: String::new(),
            sqr: 1.0,
        };
        let e3 = ONBasis {
            id: 3,
            name: String::new(),
            sqr: 1.0,
        };

        let c1 = Component {
            mag: 1.0,
            bases: vec![e2.clone(), e1.clone(), e3.clone()],
        };
        let r1 = c1.reorder_bases();
        assert_eq!(r1.mag, -1.0);
        assert_eq!(r1.bases()[0].id, 1);
        assert_eq!(r1.bases[1].id, 2);
        assert_eq!(r1.bases[2].id, 3);

        let c2 = Component {
            mag: 1.0,
            bases: vec![e2.clone(), e3.clone(), e1.clone()],
        };
        let r2 = c2.reorder_bases();
        assert_eq!(r2.mag, 1.0);
        assert_eq!(r2.bases[0].id, 1);
        assert_eq!(r2.bases[1].id, 2);
        assert_eq!(r2.bases[2].id, 3);
    }
}