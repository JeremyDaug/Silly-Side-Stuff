use std::{cmp::Ordering::{self}, hash::Hash, fmt::format};

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
    pub fn dot(&self, ridx: &ONBasis) -> f64 {
        if self == ridx {
            self.sqr()
        } else {
            0.0
        }
    }

    /// # From Str
    /// 
    /// Takes a str, processes it, and 
    pub fn from_string(val: &String) -> Result<ONBasis, &'static str> {
        if val.len() < 4 {
            return Err("Text Invalid, too short to be valid.");
        }
        let mut clone = val.clone();
        // check that it's wrapped correctly.
        let kind = clone.remove(0);
        if !(kind == 'P' || 
        kind == 'N' ||
        kind == 'Z') {
            return Err("Invalid Basis Type, must be 'P', 'N', or 'Z'.");
        }
        let open = clone.remove(0);
        let close = clone.pop().unwrap();
        if open != '(' || close != ')' {
            return Err("Invalid parens, parenthesis must close and only 1 character allowed before open.");
        }
        // all that's left is the number
        let id = clone.parse::<usize>();
        if let Ok(id) = id {
            return if kind == 'P' {
                Ok(ONBasis::P(id))
            } else if kind == 'N' {
                Ok(ONBasis::N(id))
            } else {
                Ok(ONBasis::Z(id))
            };
        } else {
            return Err("Invalid Id, must be a non-negative Integer");
        }
    }
}

impl PartialOrd for ONBasis {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let val = self.sqr().partial_cmp(&other.sqr());
        if let Some(Ordering::Equal) = val {
            // if they are the same kind, compare ids
            let l = self.unwrap();
            let r = other.unwrap();
            return l.partial_cmp(&r);
        } else {
            // otherwise, return the kind ordering.
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