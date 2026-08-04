pub mod blade;
pub mod basis;
pub mod component;
pub mod multivector;
pub mod interpreter;
pub mod vector;

#[cfg(test)]
mod tests {
    mod basis_tests {

    }

    mod component_tests {
        mod sort_bases_should {
            use crate::{basis::ONBasis, component::{self, Component}};

            #[test]
            pub fn properly_sort() {
                let p1 = ONBasis::P(1);
                let p2 = ONBasis::P(2);
                let p3 = ONBasis::P(3);
                let p4 = ONBasis::P(4);

                let test = Component::from_bases(vec![p4, p3, p2, p1]);

                assert_eq!(*test.basis(0).unwrap(), p1);
                assert_eq!(*test.basis(1).unwrap(), p2);
                assert_eq!(*test.basis(2).unwrap(), p3);
                assert_eq!(*test.basis(3).unwrap(), p4);
                assert_eq!(test.mag(), 1.0);

                let test = Component::from_bases(vec![p2, p1]);

                assert_eq!(*test.basis(0).unwrap(), p1);
                assert_eq!(*test.basis(1).unwrap(), p2);
                assert_eq!(test.mag(), -1.0);
            }
        }

        mod scalar_mult_should {
            use crate::{basis::ONBasis, component::Component};

            #[test]
            pub fn correctly_multiply() {
                let test1 = Component::new(2.0, vec![]);

                let res1 = test1.scalar_mult(5.0);
                assert_eq!(res1.mag(), 10.0);
                assert_eq!(res1.len(), 0);

                let test2 = Component::new(1.0, vec![ONBasis::P(1)]);

                let res2 = test2.scalar_mult(5.0);
                assert_eq!(res2.mag(), 5.0);
                assert_eq!(res2.len(), 1);
                assert_eq!(*res2.basis(0).unwrap(), ONBasis::P(1));

                let test3 = Component::new(-1.0, vec![ONBasis::Z(1)]);

                let res3 = test3.scalar_mult(5.0);
                assert_eq!(res3.mag(), -5.0);
                assert_eq!(res3.len(), 1);
                assert_eq!(*res3.basis(0).unwrap(), ONBasis::Z(1));
            }
        }

        mod component_add_should {
            use crate::{basis::ONBasis, component::Component};

            #[test]
            pub fn add_when_bases_match() {
                let test1 = Component::new(1.0, vec![ONBasis::P(1)]);
                let test2 = Component::new(1.0, vec![ONBasis::P(1)]);

                if let Some(res) = test1.component_add(test2) {
                    assert_eq!(res.mag(), 2.0);
                    assert_eq!(*res.basis(0).unwrap(), ONBasis::P(1));
                } else {
                    assert!(false);
                }
            }

            #[test]
            pub fn not_add_when_bases_dont_match() {
                let test1 = Component::new(1.0, vec![ONBasis::P(1)]);
                let test2 = Component::new(1.0, vec![ONBasis::P(2)]);

                assert!(test1.component_add(test2).is_none());
            }
        }
    }
}