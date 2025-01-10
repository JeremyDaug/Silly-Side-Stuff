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
    /// # Unwrap
    /// 
    /// Get's the id of the basis.
    pub fn unwrap(&self) -> usize {
        match self {
            ONBasis::P(i) | 
            ONBasis::Z(i) |
            ONBasis::N(i) => *i,
        }
    }

    /// # Square
    /// 
    /// Used when a basis is multiplied against itself.
    /// 
    /// Produces the result of the multiplication.
    pub fn sqr(&self) -> f64 {
        match self {
            ONBasis::P(_) => 1.0,
            ONBasis::Z(_) => 0.0,
            ONBasis::N(_) => -1.0,
        }
    }

    /// # Is Positive
    /// 
    /// Checks if the basis squares to +1
    pub fn is_pos(&self) -> bool {
        match self {
            ONBasis::P(_) => true,
            ONBasis::Z(_) => false,
            ONBasis::N(_) => false,
        }
    }

    /// # Is Zero
    /// 
    /// Checks if the basis squares to 0
    pub fn is_zero(&self) -> bool {
        match self {
            ONBasis::P(_) => false,
            ONBasis::Z(_) => true,
            ONBasis::N(_) => false,
        }
    }

    /// # Is Negative
    /// 
    /// Checks if the basis squares to -1
    pub fn is_neg(&self) -> bool {
        match self {
            ONBasis::P(_) => false,
            ONBasis::Z(_) => false,
            ONBasis::N(_) => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ONBasis::P(id) => format!("p{}", id),
            ONBasis::Z(id) => format!("z{}", id),
            ONBasis::N(id) => format!("n{}", id),
        }
    }

    /// # Dot function
    /// 
    /// Used to take the inner product between two bases.
    pub fn dot(&self, rhs: &ONBasis) -> f64 {
        if self == rhs {
            self.sqr()
        } else {
            0.0
        }
    }

    /// # From Str
    /// 
    /// Takes a str, processes it, and outputs an ONBasis, if possible.
    /// 
    /// If fails, it returns an error.
    pub fn from_string(val: &String) -> Result<ONBasis, String> {
        // Use Regex to break it up nicely.
        let re = Regex::new(BASIS_REGEX).unwrap();
        // If it doesn't match the regex, return an error.
        // This also checks if the Basis is P,N, or Z.
        let Some(caps) = re.captures(val) else {
            return Err(format!("'{val}' does not match the form P(#), N(#), or Z(#)."));
        };
        // Get the basis name and basis ID.
        let e = &caps["e"];
        let id = &caps["id"];

        // If ID can't be parsed into uint, return error.
        let Ok(id_val) = id.parse::<usize>() else {
            return Err(String::from(format!("Id in '{val}' could not parse into integer.")));
        };

        // turns e and ID into an ON Basis and returns.
        Ok(match e {
            "P" => ONBasis::P(id_val),
            "N" => ONBasis::N(id_val),
            "Z" => ONBasis::Z(id_val),
            _ => unreachable!()
        })
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