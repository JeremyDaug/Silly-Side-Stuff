use crate::{basis, component::Component, multivector::Multivector};

const ZERO: D1Vector = D1Vector { components: vec![] };

/// # D1 Vector
/// 
/// D1 Vector (1-Vector) is a mathematical vector, ie a 1 dimensional structure.
/// 
/// This is useful to have as it allows us to define and use them securely elsewhere.
/// 
/// These vectors are a list of grade 1 components. 
/// 
/// Vectors are always blades.
/// 
/// Components are organized by basis in the component vector for consistency and 
/// possible later improvements.
/// 
/// TODO: Consider making component into a special restricted variant for D1Vector. Would reduce size, increase speed, and reduce extraction code for the basis.
#[derive(Debug, Clone, PartialEq)]
pub struct D1Vector {
    /// The components that make up our D1Vector
    pub components: Vec<Component>,
}

impl D1Vector {
    /// # Length
    /// 
    /// Gets how many components are in the D1Vector.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// # New
    /// 
    /// Creates a new D1Vector.
    /// 
    /// # Note
    /// 
    /// If a component is not of grade 1, it skips that component, so do be aware of
    /// that.
    pub fn new(components: &Vec<Component>) -> Self {
        let mut result = ZERO;
        for component in components {
            result = result.comp_add(component);
        }
        result
    }

    /// # Component Wise Add
    /// 
    /// Adds a given component to a D1Vector and returns the result.
    /// 
    /// If component is not of grade 1, it returns the original D1Vector safely.
    pub fn comp_add(&self, component: &Component) -> Self {
        let mut result = self.clone();
        if component.grade() != 1 {
            return result;
        }
        // binary search for a match.
        let mut low = 0;
        let mut high = self.len().saturating_sub(1);
        let other_basis = component.bases()[0];
        while low <= high {
            let mid = low + (high - low) / 2;
            let current_basis = self.components[mid].bases()[0];
            if current_basis == other_basis { // found basis match, add and break out.
                result.components.get_mut(mid).unwrap().mag += component.mag;
                return result;
            } else if current_basis < other_basis  { // too low
                low = mid + 1;
            } else { // too high
                high = mid.saturating_sub(1);
            }
        }
        // fi we get here, just insert the component properly and return that.
        result.sorted_insert(component);
        result
    }

    /// # Sorted Insert
    /// 
    /// Used to insert a component into a D1Vector in a way consistent with our 
    /// pre-defined ordering.
    fn sorted_insert(&mut self, component: &Component) {
        debug_assert_eq!(component.grade(), 1, "Component must be of grade 1! -> Component {:?}", component);

        let mut idx = 0;
        let comp_b = component.bases().first().unwrap();
        while idx < self.len() {
            let self_b = self.components.get(idx).unwrap().bases().first().unwrap();
            match comp_b.cmp(&self_b) {
                std::cmp::Ordering::Less => idx += 1,
                std::cmp::Ordering::Equal => panic!("Should never reach here as components being added should already be guaranteed not duplicated."),
                std::cmp::Ordering::Greater => {
                    self.components.insert(idx, component.clone());
                    break;
                },
            }
        }
    }

    /// # Vector Addition
    /// 
    /// Takes in two vectors and adds them together.
    /// 
    /// Does so intelligently by zipping them together.
    pub fn vec_add(&self, other: &Self) -> Self {
        let mut result = ZERO;
        let mut s_idx = 0;
        let mut o_idx = 0;
        while s_idx < self.len() && o_idx < other.len() {
            let self_comp = self.components.get(s_idx).unwrap();
            let other_comp = self.components.get(o_idx).unwrap();
            match self_comp.bases()[0].cmp(&other_comp.bases()[0]) {
                std::cmp::Ordering::Less => {
                    // if self less than other, push the self_comp and increment self idx
                    s_idx += 1;
                    result.components.push(self_comp.clone());
                },
                std::cmp::Ordering::Equal => {
                    // if they are equal, increment both and add together.
                    s_idx += 1;
                    o_idx += 1;
                    result.components.push(self_comp.force_comp_add(other_comp)
                        .expect("Basis mismatch in components?"));
                },
                std::cmp::Ordering::Greater => {
                    // if self greater than other, push other and increment other_idx
                    o_idx += 1;
                    result.components.push(other_comp.clone());
                },
            }
        }
        // once the lockstepping is done, with one, march through the remainder and just push them.
        while s_idx < self.len() {
            result.components.push(self.components[s_idx].clone());
            s_idx += 1;
        }
        while o_idx< self.len() {
            result.components.push(other.components[o_idx].clone());
            o_idx += 1;
        }

        result
    }

    /// # Reverse
    /// 
    /// Reverses the vector, which is just the vector.
    pub fn reverse(&self) -> D1Vector {
        self.clone()
    }

    /// # Scalar Product
    /// 
    /// Multiplies a vector by a scalar value.
    pub fn scalar_product(&self, scalar: f64) -> D1Vector {
        let mut result = ZERO;
        for comp in self.components.iter() {
            result.components.push(comp.scalar_mult(scalar));
        }
        result
    }

    /// # Dot Product
    /// 
    /// Dot product between two vectors.
    pub fn dot(&self, rhs: &Self) -> f64 {

    }

    /// # Norm Squared
    /// 
    /// It gets the norm of the vector squared.
    pub fn norm_sqrd(&self) -> f64 {
        // Norm_sqrd is effectively the dot product of a vector with itself.
        todo!()
    }
}