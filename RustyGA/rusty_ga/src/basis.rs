use std::{cmp::Ordering::{self}, hash::Hash};

use regex::Regex;


pub const BASIS_REGEX: &str = r"^(?<e>[PNZ])[(](?<id>0|[1-9][0-9]*)[)]$";

/// # ONBasis (Orthonomal Basis)
/// 
/// Basis data storage. Contains the id of the basis and
/// whether it multiplies to **P**ositive, **N**egative, or **Z**ero.
/// 
/// Names of the bases are stored in the Geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ONBasis {
    P(usize),
    Z(usize),
    N(usize)
}

impl ONBasis {
    pub fn unwrap(&self) -> usize {
        match self {
            ONBasis::P(id) |
            ONBasis::Z(id) |
            ONBasis::N(id) => *id
        }
    }

    pub fn is_positive(&self) -> bool {
        match self {
            ONBasis::P(_) => true,
            ONBasis::Z(_) => false,
            ONBasis::N(_) => false
        }
    }

    pub fn is_negative(&self) -> bool {
        match self {
            ONBasis::P(_) => false,
            ONBasis::Z(_) => false,
            ONBasis::N(_) => true
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            ONBasis::P(_) => false,
            ONBasis::Z(_) => true,
            ONBasis::N(_) => false
        }
    }

    /// # Is Degenerate
    /// 
    /// Checks if the Orthonormal Basis is Degenerate (squares to 0)
    pub fn is_degenerate(&self) -> bool {
        self.is_zero()
    }

    /// # Squared
    /// 
    /// Gets the square of an ONBasis
    pub fn sqr(&self) -> f64 {
        match self {
            ONBasis::P(_) => 1.0,
            ONBasis::Z(_) => 0.0,
            ONBasis::N(_) => -1.0,
        }
    }

    /// # Dot Product
    /// 
    /// Gets the dot between two ONBasis.
    pub fn dot(&self, rhs: &Self) -> f64 {
        if self == rhs {
            self.sqr()
        } else {
            0.0
        }
    }
}

impl PartialOrd for ONBasis {
    /// Partial Compare
    /// 
    /// Compares 2 ONBasis, used for multibasis organization.
    /// 
    /// Organizes them by Positive, Zero, and Negative magnitude, then by Basis Id
    /// between them.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let val = self.sqr().partial_cmp(&other.sqr());
        if let Some(Ordering::Equal) = val {
            // if they are the same kind, compare ids
            let l = self.unwrap();
            let r = other.unwrap();
            return l.partial_cmp(&r);
        } else {
            // otherwise, return the kind ordering. +, 0, -
            return val;
        }
    }
}

impl Ord for ONBasis {
    /// # Compare ONBases
    /// 
    /// Positives -> Zeroes -> Negatives
    /// then ID order.
    fn cmp(&self, other: &Self) -> Ordering {
        let val = self.sqr().total_cmp(&other.sqr());
        if Ordering::Equal == val {
            // if they are the same kind, compare ids
            let l = self.unwrap();
            let r = other.unwrap();
            return l.cmp(&r);
        } else {
            // otherwise, return the kind ordering.
            return val;
        }
    }
}