pub mod blade;
pub mod basis;
pub mod component;
pub mod multivector;
pub mod interpreter;

#[cfg(test)]
mod tests {
    mod basis_tests {

        mod from_string_should {
            use crate::basis::ONBasis;

            #[test]
            pub fn correctly_parse_string_values() {
                let result = ONBasis::from_string(&String::from("P(0)")).expect("Didn't work!");
                assert_eq!(result, ONBasis::P(0));

                let result = ONBasis::from_string(&String::from("N(0)")).expect("Didn't work!");
                assert_eq!(result, ONBasis::N(0));

                let result = ONBasis::from_string(&String::from("Z(0)")).expect("Didn't work!");
                assert_eq!(result, ONBasis::Z(0));

                let result = ONBasis::from_string(&String::from("P(1000000)")).expect("Didn't work!");
                assert_eq!(result, ONBasis::P(1000000));

                let result = ONBasis::from_string(&String::from("N(1000000)")).expect("Didn't work!");
                assert_eq!(result, ONBasis::N(1000000));

                let result = ONBasis::from_string(&String::from("Z(1000000)")).expect("Didn't work!");
                assert_eq!(result, ONBasis::Z(1000000));
            }

            #[test]
            pub fn correctly_fail_invalid_strings() {
                let _result = ONBasis::from_string(&String::from("PP(0)"))
                    .err().expect("No err for PP(0)");
                let _result = ONBasis::from_string(&String::from("Q(0)"))
                    .err().expect("No err for Q(0)");
                let _result = ONBasis::from_string(&String::from("P(00)"))
                    .err().expect("No err for P(00)");
                let _result = ONBasis::from_string(&String::from("P(01)"))
                    .err().expect("No err for P(01)");
                let _result = ONBasis::from_string(&String::from("P(-0)"))
                    .err().expect("No err for P(-0)");
                let _result = ONBasis::from_string(&String::from("P(1.0)"))
                    .err().expect("No err for P(1.0)");
                let _result = ONBasis::from_string(&String::from("P(A)"))
                    .err().expect("No err for P(A)");
            }
        }
    }

    mod component_tests {
        mod from_string_should {
            use regex::Regex;

            use crate::{basis::ONBasis, component::{Component, ZERO}};

            #[test]
            pub fn correctly_regex_string() {
                let float_re = r"(?<val>[-+]?[0]|[1-9][0-9]*|[1-9][0-9]*[.][0-9]+|[1-9][0-9]*[.]|[.][0-9]+([eE]\^[+-]?[0-9]+)?)";
                let _component_regex: &str = r"(?<val>[+-]?)"; //(?<b>(?<e>[PNZ])\((?<id>0|[1-9][0-9]*)\))";

                let re = Regex::new(&float_re).unwrap();
                assert!(re.captures(".").is_none());

                let Some(caps) = re.captures("+0") else {
                    assert!(false);
                    return;
                };
                println!("{}", &caps["val"]);

                let Some(caps) = re.captures("1.") else {
                    assert!(false);
                    return;
                };
                println!("{}", &caps["val"]);

                let Some(caps) = re.captures(".1") else {
                    assert!(false);
                    return;
                };
                println!("{}", &caps["val"]);

                let Some(caps) = re.captures("1.e^10") else {
                    assert!(false);
                    return;
                };
                println!("{}", &caps["val"]);
            }

            #[test]
            pub fn correctly_parse_string() {

                let result = Component::from_string(
                    &String::from("1.0P(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![ONBasis::P(1)]));

                let result = Component::from_string(
                    &String::from("1.0N(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![ONBasis::N(1)]));

                let result = Component::from_string(
                    &String::from("1.0P(1)N(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![ONBasis::P(1), ONBasis::N(1)]));

                let result = Component::from_string(
                    &String::from("1.0P(1)P(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![]));

                let result = Component::from_string(
                    &String::from("1.0N(1)P(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(-1.0, vec![ONBasis::P(1), ONBasis::N(1)]));

                let result = Component::from_string(
                    &String::from("1.0N(1)N(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(-1.0, vec![]));

                let result = Component::from_string(
                    &String::from("1P(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![ONBasis::P(1)]));

                let result = Component::from_string(
                    &String::from("254P(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(254.0, vec![ONBasis::P(1)]));

                let result = Component::from_string(
                    &String::from("1.125P(1)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.125, vec![ONBasis::P(1)]));

                let result = Component::from_string(
                    &String::from("1")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![]));

                let result = Component::from_string(
                    &String::from("1.0")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![]));

                let result = Component::from_string(
                    &String::from("1.")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![]));

                let result = Component::from_string(
                    &String::from(".0")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(0.0, vec![]));

                let result = Component::from_string(
                    &String::from("-1.0")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(-1.0, vec![]));

                let result = Component::from_string(
                    &String::from("+1.0")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![]));

                let result = Component::from_string(
                    &String::from("P(0)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![ONBasis::P(0)]));

                let result = Component::from_string(
                    &String::from("+P(0)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(1.0, vec![ONBasis::P(0)]));

                let result = Component::from_string(
                    &String::from("-P(0)")
                ).expect("Invalid Conversion");
                assert_eq!(result, Component::new(-1.0, vec![ONBasis::P(0)]));

                let result = Component::from_string(
                    &String::from("")
                ).expect("Invalid Conversion");
                assert_eq!(result, ZERO);
            }

            #[test]
            pub fn correctly_fail_string() {
                Component::from_string(
                    &String::from("1..")
                ).err().expect("Double radix point.");

                Component::from_string(
                    &String::from("1.0.0")
                ).err().expect("Double Radix Point (mk 2).");

                Component::from_string(
                    &String::from("1.0P")
                ).err().expect("Bad Basis");

                Component::from_string(
                    &String::from("1.0-")
                ).err().expect("Negative After Decimal.");

                Component::from_string(
                    &String::from("1.-0")
                ).err().expect("Negative placed incorrectly.");

                Component::from_string(
                    &String::from("1.+0")
                ).err().expect("Negative placed incorrectly.");
            }
        }

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
            use crate::{basis::ONBasis, component::Component};

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
                        let result = lhs.inner_product_f64(rhs);
                        if lhs == rhs {
                            // if along the diagonal, it should have a value.
                            if lhs.grade() / 2 % 2 > 0 { // magnitude flips every 2 grades.
                                // negative grades
                                assert_eq!(-1.0, result, "Testing inner product on {} and {}", lhs.to_string(), rhs.to_string());
                            } else {
                                // positive grades
                                assert_eq!(1.0, result, "Testing inner product on {} and {}", lhs.to_string(), rhs.to_string());
                            }
                        } else {
                            assert_eq!(0.0, result, "Testing inner product on {} and {}", lhs.to_string(), rhs.to_string());
                        }
                    }
                }
            }

            #[test]
            pub fn correctly_square_matched_bases() {
                let p_1 = ONBasis::P(1);
                let n_1 = ONBasis::N(1);
                let z_1 = ONBasis::Z(1);

                let p = Component::new(1.0, vec![p_1]);
                let n = Component::new(1.0, vec![n_1]);
                let z = Component::new(1.0, vec![z_1]);

                let resp = p.inner_product_f64(&p);
                assert_eq!(resp, 1.0);

                let resn = n.inner_product_f64(&n);
                assert_eq!(resn, -1.0);

                let resz = z.inner_product_f64(&z);
                assert_eq!(resz, 0.0);
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
    
        mod left_cont_should {
            use crate::{component::Component, basis::ONBasis};

            #[test]
            pub fn calculate_correctly_for_basics() {
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
                
                //  lhs\rhs | 1 | e1 | e2 | e3 | e12 | e23 | e13 | e123
                //  1         1   e1   e2   e3   e12   e23   e13   e123
                //  e1        0   1    0    0    e2     0     e3   e23
                //  e2        0   0    1    0   -e1    e3     0   -e13
                //  e3        0   0    0    1     0   -e2   -e1    e12
                //  e12       0   0    0    0     -1    0     0   -e3
                //  e23       0   0    0    0     0    -1     0   -e1
                //  e13       0   0    0    0     0     0     -1   e2
                //  e123      0   0    0    0     0     0     0    -1

                let comps = vec![c1, c2, c3];
                let mut vals = vec![];
                for mask in 0..8 { // make the example components
                    let mut lhs = Component::new(1.0, vec![]);
                    if mask & 1 > 0 {
                        lhs = lhs ^ &comps[0];
                    }
                    if mask & 2 > 0 {
                        lhs = lhs ^ &comps[1];
                    }
                    if mask & 4 > 0 {
                        lhs = lhs ^ &comps[2];
                    }
                    vals.push(lhs);
                }

                // print the results quickly
                /*
                let mut output = String::new();
                output.push_str("lhs\\rhs | ");
                for comp in vals.iter() {
                    output.push_str(&comp.to_string());
                    output.push_str(" | ");
                }
                output.push('\n');
                for lidx in 0..vals.len() {
                    output.push_str(&vals[lidx].to_string());
                    output.push_str(" | ");
                    for ridx in 0..vals.len() {
                        let lhs = &vals[lidx];
                        let rhs = &vals[ridx];
                        let res = lhs >> rhs;
                        output.push_str(&res.to_string());
                        output.push_str(" | ");
                    }
                    output.push('\n');
                }
                print!("{}", output);
                */
                // check the truth table
                for lidx in 0..vals.len() {
                    for ridx in 0..vals.len() {
                        let lhs = &vals[lidx];
                        let rhs = &vals[ridx];
                        let res = lhs << rhs;
                        if lhs.grade() > rhs.grade() {
                            assert_eq!(res.mag, 0.0);
                            assert_eq!(res.bases().len(), 0);
                        } else if lhs.grade() == rhs.grade() {
                            assert_eq!(res.bases().len(), 0);
                            if lhs.bases().iter().any(|x| !rhs.bases().contains(x)) {
                                // if any basis in left is not in the right, should be 0
                                assert_eq!(res.mag, 0.0);
                                assert_eq!(res.bases().len(), 0);
                            } else {
                                // if lhs in rhs, should have magnitude.
                                if lhs.grade() > 1 { // for matches d2+ = -1
                                    assert_eq!(res.mag, -1.0);
                                } else { // for matches d1- = 1
                                    assert_eq!(res.mag, 1.0);
                                }
                            }
                        } else { // lhs.grade() < rhs.grade()
                            if lhs.bases().iter().any(|x| !rhs.bases().contains(x)) {
                                // if any basis in left is not in the right, should be 0
                                assert_eq!(res.mag, 0.0);
                                assert_eq!(res.bases().len(), 0);
                            } else {
                                // if all lhs bases in rhs bases, expect those not overlapping.
                                for basis in rhs.bases().iter() {
                                    if !lhs.bases().contains(basis) {
                                        assert!(res.bases().contains(basis));
                                    }
                                }
                                // magnitude should flip based on swaps.
                                // Just select +/- based on idcs
                                if (lidx == 2 && ridx == 3) ||
                                (lidx == 2 && ridx == 7) ||
                                (lidx == 3 && ridx == 5) ||
                                (lidx == 3 && ridx == 7) ||
                                (lidx == 3 && ridx == 6) ||
                                (lidx == 4 && ridx == 5) ||
                                (lidx == 6 && ridx == 7) ||
                                (lidx == 4 && ridx == 6) { // selecting the results which should be negative.
                                    assert_eq!(res.mag, -1.0, "{}:{} >> {}:{}", lidx, lhs.to_string(), ridx, rhs.to_string());
                                } else {
                                    assert_eq!(res.mag, 1.0, "{}:{} >> {}:{}", lidx, lhs.to_string(), ridx, rhs.to_string());
                                }
                            }
                        }
                    }
                }
            }

            #[test]
            pub fn correctly_multiply_magnitudes() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                // simple vector components
                let c1 = Component::new(
                    2.0,
                    vec![e1],
                );
                let c2 = Component::new(
                    3.0,
                    vec![e2],
                );
                let c3 = Component::new(
                    4.0,
                    vec![e3],
                );

                let c123 = Component::new(1.0, 
                vec![e1, e2, e3]);

                let c12 = &c1 << &c123;
                assert_eq!(c12.mag, 2.0);

                let c12 = &c2 << &c123;
                assert_eq!(c12.mag, -3.0);

                let c12 = &c3 << &c123;
                assert_eq!(c12.mag, 4.0);

                let c12 = &c2 << (&c1 << &c123);
                assert_eq!(c12.mag, 6.0);

                let c12 = &c3 << (&c1 << &c123);
                assert_eq!(c12.mag, -8.0);

                let c12 = &c3 << (&c2 << &c123);
                assert_eq!(c12.mag, 12.0);

                let c12 = &c3 << (&c2 << (&c1 << &c123));
                assert_eq!(c12.mag, 24.0);
            }
        }

        mod right_cont_should {
            use crate::{component::Component, basis::ONBasis};

            #[test]
            pub fn calculate_correctly_for_basics() {
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
                
                //  lhs\rhs | 1 | e1 | e2 | e3 | e12 | e23 | e13 | e123
                //  1         1   e1   e2   e3   e12   e23   e13   e123
                //  e1        0   1    0    0    e2     0     e3   e23
                //  e2        0   0    1    0   -e1    e3     0   -e13
                //  e3        0   0    0    1     0   -e2   -e1    e12
                //  e12       0   0    0    0     -1    0     0   -e3
                //  e23       0   0    0    0     0    -1     0   -e1
                //  e13       0   0    0    0     0     0     -1   e2
                //  e123      0   0    0    0     0     0     0    -1

                let comps = vec![c1, c2, c3];
                let mut vals = vec![];
                for mask in 0..8 { // make the example components
                    let mut lhs = Component::new(1.0, vec![]);
                    if mask & 1 > 0 {
                        lhs = lhs ^ &comps[0];
                    }
                    if mask & 2 > 0 {
                        lhs = lhs ^ &comps[1];
                    }
                    if mask & 4 > 0 {
                        lhs = lhs ^ &comps[2];
                    }
                    vals.push(lhs);
                }

                // print the results quickly
                let mut output = String::new();
                output.push_str("lhs\\rhs | ");
                for comp in vals.iter() {
                    output.push_str(&comp.to_string());
                    output.push_str(" | ");
                }
                output.push('\n');
                for lidx in 0..vals.len() {
                    output.push_str(&vals[lidx].to_string());
                    output.push_str(" | ");
                    for ridx in 0..vals.len() {
                        let lhs = &vals[lidx];
                        let rhs = &vals[ridx];
                        let res = lhs >> rhs;
                        output.push_str(&res.to_string());
                        output.push_str(" | ");
                    }
                    output.push('\n');
                }
                print!("{}", output);
                
                // check the truth table
                for lidx in 0..vals.len() {
                    for ridx in 0..vals.len() {
                        let lhs = &vals[lidx];
                        let rhs = &vals[ridx];
                        let res = lhs >> rhs;
                        if lhs.grade() < rhs.grade() {
                            assert_eq!(res.mag, 0.0);
                            assert_eq!(res.bases().len(), 0);
                        } else if lhs.grade() == rhs.grade() {
                            assert_eq!(res.bases().len(), 0);
                            if lhs.bases().iter().any(|x| !rhs.bases().contains(x)) {
                                // if any basis in left is not in the right, should be 0
                                assert_eq!(res.mag, 0.0);
                                assert_eq!(res.bases().len(), 0);
                            } else {
                                // if lhs in rhs, should have magnitude.
                                if lhs.grade() > 1 { // for matches d2+ = -1
                                    assert_eq!(res.mag, -1.0);
                                } else { // for matches d1- = 1
                                    assert_eq!(res.mag, 1.0);
                                }
                            }
                        } else { // lhs.grade() > rhs.grade()
                            if rhs.bases().iter().any(|x| !lhs.bases().contains(x)) {
                                // if any basis in left is not in the right, should be 0
                                assert_eq!(res.mag, 0.0);
                                assert_eq!(res.bases().len(), 0);
                            } else {
                                // if all lhs bases in rhs bases, expect those not overlapping.
                                for basis in lhs.bases().iter() {
                                    if !rhs.bases().contains(basis) {
                                        assert!(res.bases().contains(basis));
                                    }
                                }
                                // magnitude should flip based on swaps.
                                // Just select +/- based on idcs
                                if (lidx == 3 && ridx == 1) ||
                                   (lidx == 5 && ridx == 1) ||
                                   (lidx == 6 && ridx == 2) ||
                                   (lidx == 7 && ridx == 2) ||
                                   (lidx == 7 && ridx == 3) ||
                                   (lidx == 7 && ridx == 6){ // selecting the results which should be negative.
                                    assert_eq!(-1.0, res.mag, "{}:{} >> {}:{}", lidx, lhs.to_string(), ridx, rhs.to_string());
                                } else {
                                    assert_eq!(1.0, res.mag, "{}:{} >> {}:{}", lidx, lhs.to_string(), ridx, rhs.to_string());
                                }
                            }
                        }
                    }
                }
            }

            #[test]
            pub fn correctly_multiply_magnitudes() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                // simple vector components
                let c1 = Component::new(
                    2.0,
                    vec![e1],
                );
                let c2 = Component::new(
                    3.0,
                    vec![e2],
                );
                let c3 = Component::new(
                    4.0,
                    vec![e3],
                );

                let c123 = Component::new(1.0, 
                vec![e1, e2, e3]);

                let c12 = &c123 >> &c1;
                assert_eq!(c12.mag, 2.0);

                let c12 = &c123 >> &c2;
                assert_eq!(c12.mag, -3.0);

                let c12 = &c123 >> &c3;
                assert_eq!(c12.mag, 4.0);

                let c12 = (&c123 >> &c1) >> &c2;
                assert_eq!(c12.mag, -6.0);

                let c12 = (&c123 >> &c1) >> &c3;
                assert_eq!(c12.mag, 8.0);

                let c12 = (&c123 >> &c2) >> &c3;
                assert_eq!(c12.mag, -12.0);

                let c12 = (&c123 >> &c1) >> &c2 >> &c3;
                assert_eq!(c12.mag, -24.0);
            }
        }

        mod reversion_should {
            use crate::{component::Component, basis::ONBasis};

            #[test]
            pub fn correctly_calculate_value() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                let e4 = ONBasis::P(4);
                // simple vector components
                let c0 = Component::new(1.0, vec![]);
                let c1 = Component::new(
                    1.0,
                    vec![e1],
                );
                let c2 = Component::new(
                    1.0,
                    vec![e2],
                );
                let c3 = Component::new(
                    1.0,
                    vec![e3],
                );
                let c4 = Component::new(
                    1.0,
                    vec![e4],
                );
                let c0r = c0.reversion();
                assert_eq!(c0r.mag, 1.0);
                assert_eq!(c0r.bases().len(), 0);

                let c1r = c1.reversion();
                assert_eq!(c1r.mag, 1.0);
                assert_eq!(c1r.bases(), c1.bases());

                let c12 = &c1 ^ &c2;
                let c12r = c12.reversion();
                assert_eq!(c12r.mag, -1.0);
                assert_eq!(c12r.bases(), c12.bases());

                let c123 = &c1 ^ &c2 ^ &c3;
                let c123r = c123.reversion();
                assert_eq!(c123r.mag, -1.0);
                assert_eq!(c123r.bases(), c123.bases());

                let c1234 = &c1 ^ &c2 ^ &c3 ^ &c4;
                let c1234r = c1234.reversion();
                assert_eq!(c1234r.mag, 1.0);
                assert_eq!(c1234r.bases(), c1234.bases());
            }
        }

        mod inverse_should {
            use crate::{component::Component, basis::ONBasis};

            #[test]
            pub fn calculate_correctly() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                // simple vector components
                let c1 = Component::new(
                    2.0,
                    vec![e1],
                );
                let c2 = Component::new(
                    3.0,
                    vec![e2],
                );
                let c3 = Component::new(
                    4.0,
                    vec![e3],
                );

                // vector inverse
                let inv = c1.inverse();
                assert_eq!(inv.mag, 1.0/2.0);
                assert_eq!(inv.bases(), c1.bases());
                let combo = &c1 << &inv;
                assert_eq!(combo.mag, 1.0);
                assert_eq!(combo.bases().len(), 0);

                // bivector inverse
                let c = &c1 ^ &c2;
                let inv = c.inverse();
                assert_eq!(inv.mag, -1.0/6.0);
                assert_eq!(inv.bases(), c.bases());
                let combo = &c << &inv;
                assert_eq!(combo.mag, 1.0);
                assert_eq!(combo.bases().len(), 0);

                // trivector inverse
                let c = c1 ^ c2 ^ c3;
                let inv = c.inverse();
                assert_eq!(inv.mag, -1.0/24.0);
                assert_eq!(inv.bases(), c.bases());
                let combo = &c << &inv;
                assert_eq!(combo.mag, 1.0);
                assert_eq!(combo.bases().len(), 0);
            }
        }
    
        mod dual_should {
            use crate::{basis::ONBasis, component::Component};

            #[test]
            pub fn function_correctly() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                let e4 = ONBasis::P(4);
                // simple vector components
                let c1 = Component::new(
                    1.0,
                    vec![e1],
                );
                let c2 = Component::new(
                    1.0,
                    vec![e2],
                );
                let c3 = Component::new(
                    1.0,
                    vec![e3],
                );
                let c4 = Component::new(
                    1.0,
                    vec![e4],
                );

                let i1 = &c1;
                let i2 = &(&c1 ^ &c2);
                let i3 = &(&c1 ^ &c2 ^ &c3);
                let i4 = &(&c1 ^ &c2 ^ &c3 ^ &c4);

                let val = Component::new(2.0, vec![e1.clone()]);
                let dual1 = val.dual(i1);

                // no change in magnitude
                assert_eq!(val.mag, dual1.mag);
                // no overlapping bases
                assert!(!val.bases().iter().any(|x| dual1.bases().contains(x)));
                // should combine to the full pseudoscalar
                assert_eq!((&val ^ &dual1).bases(), i1.bases());

                let dual2 = val.dual(i2);

                // no change in magnitude
                assert_eq!(-val.mag, dual2.mag);
                // no overlapping bases
                assert!(!val.bases().iter().any(|x| dual2.bases().contains(x)));
                // should combine to the full pseudoscalar
                assert_eq!((&val ^ &dual2).bases(), i2.bases());

                let dual3 = val.dual(i3);

                // no change in magnitude
                assert_eq!(-val.mag, dual3.mag);
                // no overlapping bases
                assert!(!val.bases().iter().any(|x| dual3.bases().contains(x)));
                // should combine to the full pseudoscalar
                assert_eq!((&val ^ &dual3).bases(), i3.bases());

                let dual4 = val.dual(i4);

                // no change in magnitude
                assert_eq!(val.mag, dual4.mag);
                // no overlapping bases
                assert!(!val.bases().iter().any(|x| dual4.bases().contains(x)));
                // should combine to the full pseudoscalar
                assert_eq!((&val ^ &dual4).bases(), i4.bases());

                let ddual1 = dual1.dual(i1);

                // no change in magnitude
                assert_eq!(val.mag, ddual1.mag);
                // no overlapping bases
                assert!(val.bases().iter().all(|x| ddual1.bases().contains(x)));

                let ddual2 = dual2.dual(i2);

                // no change in magnitude
                assert_eq!(-val.mag, ddual2.mag);
                // no overlapping bases
                assert!(val.bases().iter().all(|x| ddual2.bases().contains(x)));

                let ddual3 = dual3.dual(i3);

                // no change in magnitude
                assert_eq!(-val.mag, ddual3.mag);
                // no overlapping bases
                assert!(val.bases().iter().all(|x| ddual3.bases().contains(x)));

                let ddual4 = dual4.dual(i4);

                // no change in magnitude
                assert_eq!(val.mag, ddual4.mag);
                // no overlapping bases
                assert!(val.bases().iter().all(|x| ddual4.bases().contains(x)));

                let undual1 = dual1.undual(i1);

                assert_eq!(val, undual1);

                let undual2 = dual2.undual(i2);
                assert_eq!(val, undual2);

                let undual3 = dual3.undual(i3);
                assert_eq!(val, undual3);

                let undual4 = dual4.undual(i4);
                assert_eq!(val, undual4);
            }
        }

        mod reciprocal_frame_should {
            use crate::{basis::ONBasis, component::Component};

            #[test]
            pub fn be_calculated_correctly() {
                let e1 = ONBasis::P(1);
                let e2 = ONBasis::P(2);
                let e3 = ONBasis::P(3);
                let e4 = ONBasis::P(4);
                // simple vector components
                let c1 = Component::new(
                    1.0,
                    vec![e1],
                );
                let c2 = Component::new(
                    1.0,
                    vec![e2],
                );
                let c3 = Component::new(
                    1.0,
                    vec![e3],
                );
                let c4 = Component::new(
                    1.0,
                    vec![e4],
                );

                let i = &c1 ^ &c2 ^ &c3 ^ &c4;
                let cs = vec![c1, c2, c3, c4];

                // get reciprocals for each component.
                let s1 = i.reciprocal_frame(0);
                let s2 = i.reciprocal_frame(1);
                let s3 = i.reciprocal_frame(2);
                let s4 = i.reciprocal_frame(3);

                let rs = vec![s1, s2, s3, s4];

                // dot i with reciprocal of i and ensure c_i . c^i = 1
                for (cidx, c) in cs.iter().enumerate() {
                    for (ridx, r) in rs.iter().enumerate() {
                        let result = c.inner_product_f64(r);
                        println!("{}", result.to_string());
                        if cidx == ridx {
                            assert_eq!(result, 1.0, "{} . {}", c.to_string(), r.to_string());
                        } else {
                            assert_eq!(result, 0.0, "{} . {}", c.to_string(), r.to_string());
                        }
                    }
                }
                // and c_i . c^j = 0 when i != j
            }
        }
    }

    mod multivector_tests {
        mod from_string_should {
            use std::vec;

            use crate::{basis::ONBasis, component::Component, multivector::Multivector};

            #[test]
            pub fn correctly_parse_valid_multivectors() {
                let result = Multivector::from_string(
                    &String::from("P(1)")
                ).expect("Single Component");
                assert_eq!(result, Multivector::new(vec![
                    Component::new(1.0, vec![ONBasis::P(1)])
                ]));

                let result = Multivector::from_string(
                    &String::from("")
                ).expect("Empty.");
                assert_eq!(result, Multivector::new(vec![
                ]));

                let result = Multivector::from_string(
                    &String::from("P(1)+P(2)")
                ).expect("Multicomponent");
                assert_eq!(result, Multivector::new(vec![ 
                    Component::new(1.0, vec![ONBasis::P(1)]),
                    Component::new(1.0, vec![ONBasis::P(2)])
                ]));

                let result = Multivector::from_string(
                    &String::from("P(1)-P(2)")
                ).expect("Multicomponent with negative");
                assert_eq!(result, Multivector::new(vec![
                    Component::new(1.0, vec![ONBasis::P(1)]),
                    Component::new(-1.0, vec![ONBasis::P(2)])
                ]));

                let result = Multivector::from_string(
                    &String::from("-P(1)+P(2)")
                ).expect("Single Component");
                assert_eq!(result, Multivector::new(vec![
                    Component::new(-1.0, vec![ONBasis::P(1)]),
                    Component::new(1.0, vec![ONBasis::P(2)])
                ]));

                // triple components
                let result = Multivector::from_string(
                    &String::from("P(1)+P(2)+P(3)")
                ).expect("3 Components");
                assert_eq!(result, Multivector::new(vec![ 
                    Component::new(1.0, vec![ONBasis::P(1)]),
                    Component::new(1.0, vec![ONBasis::P(2)]),
                    Component::new(1.0, vec![ONBasis::P(3)])
                ]));

                // consolidate components
                let result = Multivector::from_string(
                    &String::from("P(1)+P(1)+P(1)")
                ).expect("Consolidation");
                assert_eq!(result, Multivector::new(vec![ 
                    Component::new(3.0, vec![ONBasis::P(1)])
                ]));
            }

            // TODO: No invalid Tests as all of them would trickle up from lower levels.
        }

        mod new_should {
            use crate::{basis::ONBasis, component::Component, multivector::Multivector};

            #[test]
            pub fn sort_list_of_components_buy_grade() {
                let b1 = ONBasis::P(0);
                let b2 = ONBasis::P(1);

                let comp1 = Component::new(1.0, vec![b1]);
                let comp2 = Component::new(1.0, vec![b2]);
                let comp12 = Component::new(1.0, vec![b1, b2]);

                let test = Multivector::new(vec![comp12, comp1, comp2]);
                assert_eq!(test.components()[0].grade(), 1);
                assert_eq!(test.components()[1].grade(), 1);
                assert_eq!(test.components()[2].grade(), 2);
            }
        }

        mod is_blade_should {
            use crate::basis::ONBasis;
            use crate::component::Component;
            use crate::multivector::Multivector;

            #[test]
            pub fn correctly_tell_when_it_is_blade_or_not() {
                let p1 = ONBasis::P(1);
                let p2 = ONBasis::P(2);
                let p3 = ONBasis::P(3);
                let p4 = ONBasis::P(4);
                let p5 = ONBasis::P(5);

                let vector = Multivector::new(
                    vec![Component::new(1.0, vec![p1])]);
                assert!(vector.is_blade(), "Pure Vec Component isn't blade.");

                let n_component = Multivector::new(
                    vec![Component::new(1.0, vec![p1, p2, p3, p4, p5])]
                );
                assert!(n_component.is_blade(), "Higher Dimension Component isn't blade.");

                let complex_blade = Multivector::new(
                    vec![
                        Component::new(1.0, vec![p1, p2]),
                        Component::new(1.0, vec![p2, p3]),
                        Component::new(1.0, vec![p3, p4]),
                        Component::new(1.0, vec![p4, p5])
                    ]
                );
                assert!(complex_blade.is_blade(), "Complex BLade is Not valid.");

                let not_blade = Multivector::new(
                    vec![
                        Component::new(1.0, vec![p1, p2]),
                        Component::new(1.0, vec![p3, p4]),
                        Component::new(1.0, vec![p4, p5])
                    ]
                );
                assert!(!not_blade.is_blade(), "Not Blade is returning as Blade.");

                let multigrade = Multivector::new(
                    vec![
                        Component::new(1.0, vec![p1]),
                        Component::new(1.0, vec![p2, p3]),
                    ]
                );
                assert!(!multigrade.is_blade(), "Multigrade has returned as blade.");
            }
        }
    
        mod add_component_should {
            use crate::{basis::ONBasis, component::{Component, self}, multivector::{self, Multivector}};

            #[test]
            pub fn correctly_add_component_to_multivector() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);
                let comp12 = Component::new(1.0, 
                    vec![b1, b2]);

                let mv = multivector::ZERO;

                let mv1 = mv.component_add(&comp1);
                assert_eq!(mv1.len(), 1);
                assert_eq!(mv1.components()[0].mag, 1.0);
                assert_eq!(mv1.components()[0].bases(), vec![b1]);

                let mv1p = mv1.component_add(&comp1);
                assert_eq!(mv1p.len(), 1);
                assert_eq!(mv1p.components()[0].mag, 2.0);
                assert_eq!(mv1p.components()[0].bases(), vec![b1]);

                let mv12 = mv1p.component_add(&comp2);
                assert_eq!(mv12.len(), 2);
                assert_eq!(mv12.components()[0].mag, 2.0);
                assert_eq!(mv12.components()[0].bases(), vec![b1]);
                assert_eq!(mv12.components()[1].mag, 1.0);
                assert_eq!(mv12.components()[1].bases(), vec![b2]);

                let mv12p = mv12.component_add(&comp12);
                assert_eq!(mv12p.len(), 3);
                assert_eq!(mv12p.components()[0].mag, 2.0);
                assert_eq!(mv12p.components()[0].bases(), vec![b1]);
                assert_eq!(mv12p.components()[1].mag, 1.0);
                assert_eq!(mv12p.components()[1].bases(), vec![b2]);
                assert_eq!(mv12p.components()[2].mag, 1.0);
                assert_eq!(mv12p.components()[2].bases(), vec![b1, b2]);
            }

            #[test]
            pub fn nullify_added_zero_components() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);
                let comp12 = Component::new(1.0, 
                    vec![b1, b2]);

                let mv = multivector::ZERO;

                let mv1 = mv.component_add(&component::ZERO);
                assert_eq!(mv1.len(), 0);

                let mv = Multivector::new(vec![
                    comp1.clone(), comp2.clone(), comp12.clone()
                ]);
                let mv1 = mv.component_add(&component::ZERO);
                assert_eq!(mv1.len(), 3);
                assert_eq!(mv.components()[0], comp1);
                assert_eq!(mv.components()[1], comp2);
                assert_eq!(mv.components()[2], comp12);
            }
        }

        mod base_add_should {
            use crate::{basis::ONBasis, component::Component, multivector::{self, Multivector}};

            #[test]
            pub fn add_two_multivectors_correctly() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp0 = Component::new(1.0, vec![]);
                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);
                let comp12 = Component::new(1.0, 
                    vec![b1, b2]);

                let mv1 = Multivector::new(vec![
                    comp0,
                    comp1.clone(), comp2.clone()
                ]);
                let mv2 = Multivector::new(vec![
                    comp1, comp2, comp12
                ]);

                let test = mv1 + mv2;
                assert_eq!(test.len(), 4);
                assert_eq!(test.components()[0].mag, 1.0);
                assert_eq!(test.components()[0].bases(), vec![]);
                assert_eq!(test.components()[1].mag, 2.0);
                assert_eq!(test.components()[1].bases(), vec![b1]);
                assert_eq!(test.components()[2].mag, 2.0);
                assert_eq!(test.components()[2].bases(), vec![b2]);
                assert_eq!(test.components()[3].mag, 1.0);
                assert_eq!(test.components()[3].bases(), vec![b1, b2]);
            }
        }

        mod scalar_add_should {
            use crate::{basis::ONBasis, component::Component, multivector::{self, Multivector}};

            #[test]
            pub fn add_correctly() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp0 = Component::new(1.0, vec![]);
                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);

                let mv1 = Multivector::new(vec![
                    comp0,
                    comp1.clone(), comp2.clone()
                ]);
                let mv2 = Multivector::new(vec![
                    comp1.clone()
                ]);

                let test = mv1 + 1.0;
                assert_eq!(test.len(), 3);
                assert_eq!(test.components()[0].mag, 2.0);
                assert_eq!(test.components()[0].bases(), vec![]);
                assert_eq!(test.components()[1], comp1);
                assert_eq!(test.components()[2], comp2);

                let test = 1.0 + mv2;
                assert_eq!(test.len(), 2);
                assert_eq!(test.components()[0].mag, 1.0);
                assert_eq!(test.components()[0].bases(), vec![]);
                assert_eq!(test.components()[1].mag, 1.0);
                assert_eq!(test.components()[1].bases(), vec![b1]);
            }
        }
    
        mod take_grade_should {
            use crate::{component::Component, basis::ONBasis, multivector::Multivector};

            #[test]
            pub fn correctly_take_grade_from_multivector() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp0 = Component::new(1.0, vec![]);
                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);
                let comp12 = Component::new(1.0, 
                    vec![b1, b2]);

                let mv = Multivector::new(vec![
                    comp0.clone(), comp1.clone(), comp2.clone(), comp12.clone()
                ]);
                let g0 = mv.take_grade(0);
                assert_eq!(g0.len(), 1);
                assert_eq!(g0.components()[0], comp0);

                let g1 = mv.take_grade(1);
                assert_eq!(g1.len(), 2);
                assert_eq!(g1.components()[0], comp1);
                assert_eq!(g1.components()[1], comp2);

                let g2 = mv.take_grade(2);
                assert_eq!(g2.len(), 1);
                assert_eq!(g2.components()[0], comp12);

                let g3 = mv.take_grade(3);
                assert_eq!(g3.len(), 0);
            }
        }

        mod comp_geo_product_should {
            use crate::{basis::ONBasis, component::Component, multivector::{self, Multivector}};

            #[test]
            pub fn add_correctly() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp0 = Component::new(1.0, vec![]);
                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);

                let mv1 = Multivector::new(vec![
                    comp0,
                    comp1.clone(), comp2.clone()
                ]);
            }
        }

        mod scalar_mult_should {
            use crate::{component::Component, basis::ONBasis, multivector::{Multivector, self}};

            #[test]
            pub fn correctly_take_grade_from_multivector() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp0 = Component::new(1.0, vec![]);
                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);
                let comp12 = Component::new(1.0, 
                    vec![b1, b2]);

                let mv = Multivector::new(vec![
                    comp0.clone(), comp1.clone(), comp2.clone(), comp12.clone()
                ]);
                let test = &mv * 3.0;
                assert_eq!(test.len(), 4);
                assert_eq!(test.components()[0].mag, 3.0);
                assert_eq!(test.components()[1].mag, 3.0);
                assert_eq!(test.components()[2].mag, 3.0);
                assert_eq!(test.components()[3].mag, 3.0);
                assert_eq!(test.components()[0].bases(), vec![]);
                assert_eq!(test.components()[1].bases(), vec![b1]);
                assert_eq!(test.components()[2].bases(), vec![b2]);
                assert_eq!(test.components()[3].bases(), vec![b1, b2]);

                let test = 0.0 * mv;
                assert_eq!(test, multivector::ZERO);
            }
        }

        mod negative_should {
            use crate::{basis::ONBasis, component::Component, multivector::Multivector};

            #[test]
            pub fn work_correctly() {
                let b1 = ONBasis::P(1);
                let b2 = ONBasis::P(2);

                let comp0 = Component::new(1.0, vec![]);
                let comp1 = Component::new(1.0, 
                    vec![b1]);
                let comp2 = Component::new(1.0, 
                    vec![b2]);
                let comp12 = Component::new(1.0, 
                    vec![b1, b2]);

                let mv = Multivector::new(vec![
                    comp0.clone(), comp1.clone(), comp2.clone(), comp12.clone()
                ]);

                let test = -mv;
                assert_eq!(test.len(), 4);
                assert_eq!(test.components()[0].mag, -1.0);
                assert_eq!(test.components()[1].mag, -1.0);
                assert_eq!(test.components()[2].mag, -1.0);
                assert_eq!(test.components()[3].mag, -1.0);
                assert_eq!(test.components()[0].bases(), vec![]);
                assert_eq!(test.components()[1].bases(), vec![b1]);
                assert_eq!(test.components()[2].bases(), vec![b2]);
                assert_eq!(test.components()[3].bases(), vec![b1, b2]);
            }
        }
    }
}