use std::{collections::{HashMap, VecDeque}, env, io::{self, stdout, Stdout}};
use rusty_ga::{basis::ONBasis, multivector::Multivector};
use std::io::prelude::*;

pub mod blade;
pub mod basis;
pub mod component;
pub mod multivector;

fn main() {
    let mut bases: HashMap<String, ONBasis> = HashMap::new();
    let mut variables: HashMap<String, Multivector> = HashMap::new();
    let mut backlog: VecDeque<String> = VecDeque::new();

    // Commandline args 1-N
    let args: Vec<String> = env::args().collect();

    // input lines
    print!(">>>");
    stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(vals) = line {
            let res = process_input(vals);
            if let Err(val) = res {
                if val == "Quit" {
                    break;
                }
            }
        }
        print!(">>>");
        stdout().flush().unwrap();
    }
}

pub fn process_input(inputs: String) -> Result<String, String> {
    let split = inputs.split_whitespace().collect::<Vec<_>>();
    let first = split.get(0);
    if let Some(first_word) = first {
        if first_word.to_lowercase() == "help" ||
        first_word.to_lowercase() == "h" {
            print!("{}", help_text);
        } else if first_word.to_lowercase() == "quit" ||
        first_word.to_lowercase() == "q" {
            return Err(String::from("Quit"));
        } else if *first_word == "=" || // assign
        *first_word == "+" || // addition
        *first_word == "*" || // geometric product
        *first_word == "/" || // Geometric Division
        *first_word == ">>" || // Left Contraction
        *first_word == "<<" || // Right Contraction
        *first_word == "^" || // Outer Product
        *first_word == "sel_grade" || // grade selection
        *first_word == "inverse" || // Inversion
        *first_word == "involution" || // Inversion
        *first_word == "reverse" || // Inversion
        *first_word == "dual" || // dual
        *first_word == "undual" || // undual
        *first_word == "generate_I" || // Generate Pseudoscalar
        *first_word == "&" || // Minimum
        *first_word == "|" || // Maximum
        *first_word == "==" || // Equality
        *first_word == "<" || // Less Than
        *first_word == ">" || // Greater Than
        *first_word == "<=" || // Less Than Or Equal
        *first_word == ">=" || // Greater Than Or Equal
        *first_word == "!=" || // Not Equal
        *first_word == "mag" || // Magnitude
        *first_word == "~" || // Negative/Negate
        *first_word == "-" || // subtraction
        *first_word == "is_blade?" // Is Blade?
        {

        }
    }

    Ok(String::from("No Comments"))
}

const help_text: &str = 
"System currently uses Postfix Notation for ease of management.
----------
Token Shorthands
Id: A name for an variable, must start with a letter. Cannot share a name with a reserved word.
Number: A number, including basis vectors.
Basis: A specific basis, in the form P(#), N(#), or Z(#), where # is non-negative.
Value: A number or already instantiated Id.
----------
Currently available options: 
Assign Value: Id Value =, Assigns or copies a value to a new varianble with the Id given.
Add: Value Value + value, adds value to value and outputs the result. If no second value given, it defaults to 0.
Subtract: Value Value -, subtracts the second value from the first. If no second value given, it instead negates the single value given.
Product: Value Value *, Geometric Product of the two values. In missing second value, it defaults to 0.
Division: Value Value /, equivalent to Geometric product of v1 v2 inverse * (v1 * v2^-1)
Left Contraction: Value Value >>, removes v1 from v2
Right Contraction: Value Value <<, Removes v2 from v1
Outer Product: Value Value ^, takes the outer product of v1 and v2
Select Grade: Value Integer <>, extracts from value the components with a grade equal to integer.
inverse: Value inverse, returns the inverse of the Value. IE, v1^-1 = v1 rev / v1 normsqrd
involution: Value involute, returns the involution of th evalue, IE, for each k-blade in the multivector, we multiply that blade by (-1)^k.
reverse: Value reverse, returns the reversion of the blades in a multivector. IE, it reverses the order of the bases, then corrects their order, resulting in blades A_k rev = (-1)^((1/2) *k * (k+1)) A_k
dual: Value I Dual, returns the dual of the value with respect to a given pseudoscalar I, such that A dual = A >> I^-1
undual: Value I Dual, returns the undual of the value with repect to a given Pseudoscalar I. Depending on the space, not all double duals equal the original value, as such, the undual guarantees that the original value is returned. Notes: In cases where the double dual doesn't equal the original blade, the 4 dual will. Both of these hold so long as no basis is degenerate (ie, no ONB::Z)
generate I: value generate_I, returns a pseudoscalar which uses all the bases vectors contained in value.
Is Blade: Value is_blade?, returns true (+inf) or false(-inf) if the given value is a blade.
Equality: Value Value ==, returns +inf if values are equal, -inf otherwise.
Not Equal: Value Value !=, returns the Negative of Equality
Less Than: Value Value <, Returns +inf if true, -inf if false and NaN if it the values cannot be defined as such. Note: You can only compare multivectors which share all their components. In that case it compares their magnitudes.
Less than or Equal: Value Value <=, equivalent to Value Value == Value Value < |
Greater than: Value Value >, Negative of Less than or equal to
Greater than or Equal: Value Value >=, Negative of Less than
Max: Value Value |, Returns the maximum of the two values. If one cannot be deterimined, it returns NaN.
Min: Value Value &, Returns the minimum of the two values, if one cannot be determined, it returns NaN.
Magnitude: Value mag, Returns the scalar value of the product between values's inverse and value. Equivalent to the norm Squared.
Negative: Value ~, Returns the negative value of the Value.
---\n";