pub mod blade;
pub mod basis;
pub mod component;


#[cfg(test)]
mod tests {

    mod component {
        mod reorder_bases_should {
            use crate::{basis::ONBasis, component::Component};
        
            #[test]
            pub fn correctly_square_bases() {
                let ep = ONBasis::P(0);
                let e0 = ONBasis::Z(0);
                let em = ONBasis::N(0);
        
                // check positive
                let c1 = Component::new(2.0, vec![ep.clone(), ep.clone()]);
                assert_eq!(c1.mag, 2.0);
                assert_eq!(c1.grade(), 0);
        
                // check negative
                let c1 = Component::new(2.0, vec![em.clone(), em.clone()]);
                assert_eq!(c1.mag, -2.0);
                assert_eq!(c1.grade(), 0);
        
                // check zero
                let c1 = Component::new(2.0, vec![e0.clone(), e0.clone()]);
                assert_eq!(c1.mag, 0.0);
                assert_eq!(c1.grade(), 0);
            }
        
            #[test]
            pub fn correctly_reorder_bases() {
                let e1 = ONBasis::P(0);
                let e2 = ONBasis::P(1);
                let e3 = ONBasis::P(2);
        
                // Pseudoscalar * Pseudoscalar
                let c1 = Component::new(1.0, vec![e1.clone(), e2.clone(), e3.clone(), 
                    e1.clone(), e2.clone(), e3.clone()]);
                assert_eq!(c1.mag, -1.0);
            }
        
            #[test]
            pub fn correctly_reorder_and_flip_sign() {
                let e1 = ONBasis::P(0);
                let e2 = ONBasis::P(1);
                let e3 = ONBasis::P(2);
        
                let c1 = Component::new(
                    1.0,
                    vec![e2.clone(), e1.clone(), e3.clone()],
                );
                assert_eq!(c1.mag, -1.0);
                assert_eq!(c1.bases()[0].unwrap(), 0);
                assert_eq!(c1.bases()[1].unwrap(), 1);
                assert_eq!(c1.bases()[2].unwrap(), 2);
        
                let c2 = Component::new(
                    1.0,
                    vec![e2.clone(), e3.clone(), e1.clone()],
                );
                assert_eq!(c2.mag, 1.0);
                assert_eq!(c2.bases()[0].unwrap(), 0);
                assert_eq!(c2.bases()[1].unwrap(), 1);
                assert_eq!(c2.bases()[2].unwrap(), 2);
            }
        }

        mod equality_check_should {
            use crate::{basis::ONBasis, component::Component};

            #[test]
            pub fn match_with_bases_correctly() {
                let e_1 = ONBasis::P(1);
                let e_2 = ONBasis::P(2);
                let e_3 = ONBasis::P(3);

                let c0 = Component::new(1.0, vec![]);
                let c1 = Component::new(1.0, vec![e_1]);
                let c2 = Component::new(1.0, vec![e_2]);
                let c3 = Component::new(1.0, vec![e_3]);

                assert_eq!(c0, c0);
                assert_eq!(c1, c1);
                assert_eq!(c2, c2);
                assert_eq!(c3, c3);

                let c123 = &c1 ^ &c2 ^ &c3;

                assert_eq!(c123, c123);
                assert_ne!(c0, c1);
                assert_ne!(c0, c123);
                assert_ne!(c1, c123);
            }
        }

        mod inner_product_should {
            use crate::{basis::ONBasis, component::{Component, self}};

            #[test]
            pub fn check_truth_table_with_all_positive_bases() {
                // 1    |  e_1  | e_2 | e_3 | e_12 | e_13 | e_23 | e_123
                // e_1  |   1   |  0  |  0  |  0   |   0  |  0   |   0
                // e_2  |   0   |  1  |  0  |  0   |   0  |  0   |   0
                // e_3  |   0   |  0  |  1  |  0   |   0  |  0   |   0
                // e_12 |   0   |  0  |  0  | -1   |   0  |  0   |   0
                // e_13 |   0   |  0  |  0  |  0   |  -1  |  0   |   0
                // e_23 |   0   |  0  |  0  |  0   |   0  | -1   |   0
                // e_123|   0   |  0  |  0  |  0   |   0  |  0   |  -1

                let e_1 = ONBasis::P(1);
                let e_2 = ONBasis::P(2);
                let e_3 = ONBasis::P(3);

                let c0 = Component::new(1.0, vec![]);
                let c1 = Component::new(1.0, vec![e_1]);
                let c2 = Component::new(1.0, vec![e_2]);
                let c3 = Component::new(1.0, vec![e_3]);
                let mut comps = vec![];
                let mut i = 0;
                // use bitmask method to select values.
                while i < 8 {
                    let mut res = c0.clone();
                    if i & 1 > 0 { // if first bit
                        res = res ^ &c1;
                    }
                    if i & 2 > 0 { // if second bit
                        res = res ^ &c2;
                    }
                    if i & 4 > 0 { // if third bit
                        res = res ^ &c3;
                    }
                    comps.push(res);
                    i += 1;
                }

                for lhs in comps.iter() {
                    for rhs in comps.iter() {
                        let result = lhs.inner_product(rhs);
                        if lhs == rhs {
                            // if along the diagonal, it should have a value.
                            if lhs.grade() / 2 % 2 > 0 { // magnitude flips every 2 grades.
                                // positive grades
                                assert_eq!(1.0, result, "Testing inner product on {} and {}", lhs.to_string(), rhs.to_string());
                            } else {
                                // negative grades
                                assert_eq!(-1.0, result, "Testing inner product on {} and {}", lhs.to_string(), rhs.to_string());
                            }
                        } else {
                            assert_eq!(0.0, result);
                        }
                    }
                }
            }
        }

        mod determinant_should {
            use crate::{component::Component, basis::ONBasis};

            #[test]
            pub fn correctly_calculate_determinant() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                // simple vector components
                let c1 = Component::new(
                    1.0,
                    vec![e1.clone()],
                );
                let c2 = Component::new(
                    1.0,
                    vec![e2.clone()],
                );
                let c3 = Component::new(
                    1.0,
                    vec![e3.clone()],
                );
            }
        }

        mod outer_proudct_should {
            use crate::{component::Component, basis::ONBasis};

            #[test]
            pub fn correctly_combine_bases() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                // simple vector components
                let c1 = Component::new(
                    1.0,
                    vec![e1.clone()],
                );
                let c2 = Component::new(
                    1.0,
                    vec![e2.clone()],
                );
                let c3 = Component::new(
                    1.0,
                    vec![e3.clone()],
                );

                let c12 = &c1 ^ &c2;
                assert_eq!(c12.mag, 1.0);
                assert_eq!(c12.grade(), 2);
                assert_eq!(c12.bases()[0].unwrap(), 1);
                assert_eq!(c12.bases()[1].unwrap(), 2);

                let c123 = c1 ^ c2 ^ c3;
                assert_eq!(c123.mag, 1.0);
                assert_eq!(c123.grade(), 3);
                assert_eq!(c123.bases()[0].unwrap(), 1);
                assert_eq!(c123.bases()[1].unwrap(), 2);
                assert_eq!(c123.bases()[2].unwrap(), 3);
            }

            #[test]
            pub fn zero_out_on_overlapping_bases() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                // simple vector components
                let c1 = Component::new(
                    1.0,
                    vec![e1.clone()],
                );
                let c2 = Component::new(
                    1.0,
                    vec![e2.clone()],
                );
                let c3 = Component::new(
                    1.0,
                    vec![e3.clone()],
                );

                let c11 = &c1 ^ &c1;
                assert_eq!(c11.mag, 0.0);
                assert_eq!(c11.grade(), 0);
            }
        }
    }
}