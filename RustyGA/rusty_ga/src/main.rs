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
            print!("System currently uses Pre-fix Notation for ease of management.
            Currently available options: 
            Assign Value: = Name Value/Name, Assigns or copies a value to a new varianble with the name [name].
            ---\n");
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
        *first_word == "generate I" || // Generate Pseudoscalar
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
        *first_word == "-" // subtraction
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
Add: + Value Value, adds value to value and outputs the result. If no second value given, it defaults to 0.
Subtract: - Value Value, subtracts the second value from the first. If no second value given, it instead negates the single value given.
Product: * Value Value, Geometric Product of the two values. In missing second value, it defaults to 0.
Division: / Value Value, equivalent to Geometric product of * v1 inverse v2
Left Contraction: >> Value Value
Right Contraction: << Value Value
Outer Product: ^ Value Value
Select Grade sel_grade
inverse inverse
involution involution
reverse reverse
dual dual
undual undual
generate I generate I
---\n";