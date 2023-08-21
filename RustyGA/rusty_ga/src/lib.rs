pub mod komponent;
pub mod orthonormalbases;

#[cfg(test)]
mod tests {
    mod komponent_tests {
        mod order_bases_should {
            use std::rc::Rc;

            use crate::orthonormalbases::OrthonormalBases;
            use crate::komponent::Komponent;

            #[test]
            pub fn correctly_reorder_bases() {
                let mut onb = OrthonormalBases::new();
                onb.add_basis(0, 1.0, "e_0".to_string(), String::new()).unwrap();
                onb.add_basis(1, 1.0, "e_1".to_string(), String::new()).unwrap();
                onb.add_basis(2, 1.0, "e_2".to_string(), String::new()).unwrap();
                onb.add_basis(3, 1.0, "e_3".to_string(), String::new()).unwrap();
                let omb = Rc::new(onb);

                let k1 =  Komponent::new_with(1.0, vec![0,1,2,3], omb.clone());
                let kf1 = k1.order_bases();
                assert_eq!(kf1.value, 1.0);
                assert_eq!(kf1.bases[0], 0);
                assert_eq!(kf1.bases[1], 1);
                assert_eq!(kf1.bases[2], 2);
                assert_eq!(kf1.bases[3], 3);

                let k4 =  Komponent::new_with(1.0, vec![1,2,3,0], omb.clone());
                let kf4 = k4.order_bases();
                assert_eq!(kf4.value, -1.0);
                assert_eq!(kf4.bases[0], 0);
                assert_eq!(kf4.bases[1], 1);
                assert_eq!(kf4.bases[2], 2);
                assert_eq!(kf4.bases[3], 3);

                let k7 =  Komponent::new_with(1.0, vec![3,2,1,0], omb.clone());
                let kf7 = k7.order_bases();
                assert_eq!(kf7.value, 1.0);
                assert_eq!(kf7.bases[0], 0);
                assert_eq!(kf7.bases[1], 1);
                assert_eq!(kf7.bases[2], 2);
                assert_eq!(kf7.bases[3], 3);
            }
        }
    }
}