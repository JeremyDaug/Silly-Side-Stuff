use std::{rc::Rc, ops, clone};

use crate::orthonormalbases::OrthonormalBases;

/// # K-Component
/// 
/// Called a Komponent, a komponent is a component of any K-vector and
/// includes a number value and the bases attached to it.
#[derive(Debug, Clone)]
pub struct Komponent {
    pub value: f64,
    pub bases: Vec<usize>,
    onb_data: Rc<OrthonormalBases>
}

impl Komponent {
    /// # New
    /// 
    /// Quick new function. Creates a Komponent with no bases and a value of
    /// 0.0
    pub fn new(onb_data: OrthonormalBases) -> Self {
        Self { value: 0.0, bases: vec![], onb_data: onb_data.into() }
    }

    /// # New With
    /// 
    /// Creates a new Komponent with value and Bases input.
    pub fn new_with(value: f64, bases: Vec<usize>, onb_data: Rc<OrthonormalBases>) -> Self {
        Self {
            value,
            bases,
            onb_data
        }
    }

    /// # New From
    /// 
    /// Creates new Komponent from another Komponent as a copy.
    pub fn new_from(other: &Self) -> Self {
        Self {
            value: other.value,
            bases: other.bases.clone(),
            onb_data: other.onb_data.clone()
        }
    }

    /// # Zero
    /// 
    /// Creates a Zero Komponent. Zero Components have no basis vectors.
    pub fn zero(onb: Rc<OrthonormalBases>) -> Self {
        Self { value: 0.0, bases: vec![], onb_data: onb.clone() }
    }

    /// # Order Bases
    /// 
    /// Given a Komponent with some Bases victors, this will produce
    /// a new Komponent with it's bases ordered by Ascending ID.
    /// 
    /// This may or may not cause the Sign to flip in the result,
    /// depending on the number of swaps which had to be done.
    /// 
    /// It does this through simple bubble sort, so may not be fastest.
    /// 
    /// ## Example:
    /// 
    /// Komponent 10 e_3 e_2 e_1 => -10 e_1 e_2 e_3
    pub fn order_bases(&self) -> Komponent {
        let mut result = Komponent::new_from(self);

        let mut swaps = 0;
        // push highest reach down
        for max in (1..result.bases.len()).rev() {
            // iterate up to max-1
            for curr in 0..(max) {
                if result.bases[curr] > result.bases[curr+1] {
                    result.bases.swap(curr, curr+1);
                    swaps += 1;
                }
            } 
        }

        result.value = result.value * (-1.0_f64).powi(swaps);

        result
    }

    /// # Is Parallel
    /// 
    /// Checks if two components are parallel with each other
    pub fn is_parallel(&self, other: &Self) -> bool {
        if self.bases.len() == 0 || other.bases.len() == 0 || 
        self.value == 0.0 || other.value == 0.0 {
            // if either is a Zero Komponent, they are parallel.
            return true;
        }
        if self.bases.len() != other.bases.len() {
            return false; // if number of bases is not equal, cannot be parallel.
        }
        if self.bases.iter().any(|x| other.bases.contains(x)) {
            return false; // if any basis isn't contained, return false.
        }
        true
    }

    pub fn omb_data(&self) -> &OrthonormalBases {
        self.onb_data.as_ref()
    }

    /// # Is Zero
    /// 
    /// A helper to check if the Komponent is equal to 0.0.
    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    /// # Komponent Add
    /// 
    /// Adds two components together.
    pub fn komponent_add(&self, rhs: &Self) -> Self {
        if !self.is_parallel(rhs) {
            Komponent::zero(self.onb_data)
        } else {
            Komponent::new_with(self.value + rhs.value, self.bases.clone(), self.onb_data.clone())
        }
    }
}

impl ops::Add for Komponent {
    type Output = Komponent;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Add<&'_ Komponent> for Komponent {
    type Output = Komponent;

    fn add(self, rhs: &'_ Komponent) -> Self::Output {
        todo!()
    }
}

impl ops::Add<Komponent> for &'_ Komponent {
    type Output = Komponent;

    fn add(self, rhs: Komponent) -> Self::Output {
        todo!()
    }
}

impl ops::Add for &'_ Komponent {
    type Output = Komponent;

    fn add(self, rhs: &'_ Komponent) -> Self::Output {

    }
}

impl ops::Neg for &'_ Komponent {
    type Output = Komponent;

    fn neg(self) -> Self::Output {
        Komponent::new_with(-self.value, self.bases.clone(), self.onb_data.clone())
    }
}

impl ops::Neg for Komponent {
    type Output = Komponent;

    fn neg(self) -> Self::Output {
        Self::new_with(-self.value, self.bases.clone(), self.onb_data.clone())
    }
}