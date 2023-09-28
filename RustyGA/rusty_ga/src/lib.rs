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