use std::{collections::{HashMap, VecDeque}, fmt::format};

use crate::{basis::ONBasis, component::Component, multivector::Multivector};

/// # Translator
/// 
/// Takes in strings, translates, and stores any values and results from the CLI.
/// 
/// May eventually be upgraded to a full interpreter.
/// 
/// The system works by a Mixed RPN.
/// 
/// Put spaces between different major operations and items, leave spaces out for 
/// those things which are part of the same thing.
/// 
/// Multivectors with multiple components should be done
pub struct Interpreter {
    /// The Bases which have been used so far.
    pub bases: HashMap<String, ONBasis>,
    /// The existing variables which have been assigned.
    pub vars: HashMap<String, Multivector>,
    // todo: Add spot for sets and strings here probably.
    /// The working stack for any operations, cleared after every line.
    pub stack: Vec<Multivector>,
    /// The previous lines processed, capped to 1,000 lines.
    pub priors: VecDeque<String>,
}

impl Interpreter {
    pub fn parse(&mut self, text: &String) -> Result<String, String> {
        for line in text.split([';', '\n']) {
            match self.parse_line(&String::from(line)) {
                Ok(res) => println!("{}", res),
                Err(exception) => {
                    if exception == "q" { // line response is actually crash/stop worthy
                        return Err(exception);
                    } else { // normal error, send message so user can correct.
                        return Ok(exception);
                    }
                },
            }
        }
        Ok(String::from("All Good"))
    }

    pub fn parse_line(&mut self, line: &String) -> Result<String, String> {
        //let mut tokens = vec![];
        for token in line.split_ascii_whitespace() {
            
            
            // Try to parse as multivec first
            
        }
        todo!()
    }
}

/// # Tokens
/// 
/// Our record of all the predefined names for the interpreter
pub enum Token {
    Assn,
    Add,
    Sub,
    Prod,
    Div,
    LCont,
    RCont,
    Outer,
    Invert,
    Involute,
    SelGrade,
    Reverse,
    Dual,
    Undual,
    GenI,
    Min,
    Max,
    Equals,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Mag,
    Neg,
    //IsBlade,
    EndStatement,
    Basis(ONBasis),
    Scalar(f64),
    Component(Component),
    Multivector(Multivector),
    Id(String)
}

impl Token {
    pub const fn val(&self) -> &'static str {
        match *self {
            Token::Assn => "=",
            Token::Add => "+",
            Token::Sub => "-",
            Token::Prod => "*",
            Token::Div => "/",
            Token::LCont => ">>",
            Token::RCont => "<<",
            Token::Outer => "^",
            Token::Invert => "invert",
            Token::Involute => "involution",
            Token::SelGrade => "sel_grade",
            Token::Reverse => "reverse",
            Token::Dual => "dual",
            Token::Undual => "undual",
            Token::GenI => "generate_I",
            Token::Min => "&",
            Token::Max => "|",
            Token::Equals => "==",
            Token::NotEqual => "!=",
            Token::LessThan => "<",
            Token::LessThanOrEqual => "<=",
            Token::GreaterThan => ">",
            Token::GreaterThanOrEqual => ">=",
            Token::Mag => "mag",
            Token::Neg => "~",
            //Token::IsBlade => "is_blade?",
            Token::Id(_) => "[a-zA-Z][a-zA-Z0-9]*",
            // Solo Basis
            Token::Basis(_) => "[PNZ]\\(\\d*\\)",
            Token::Scalar(_) => "[+-]?([0-9]*[.])?[0-9]+",
            // #(Basis)*
            Token::Component(_) => "\\d*\\.?\\d*([PNZ]\\(\\d*\\))*",
            // Component(+component)+
            Token::Multivector(_) => 
            r"\d\*\.?\d\*([PNZ]\\(\d+\\))\*([+-]\d\*\.?\d\*([PNZ]\\(\d+\\))\*)\*",
            Token::EndStatement => ";",
        }
    }

    /// # Translate String
    /// 
    /// Turns text into a token. If it cannot translate into a token, it returns 
    /// an error.
    pub fn translate_string(text: &String) -> Result<Token, String>
    {
        if text == "=" { // assign
            return Ok(Token::Assn);
        } else if text == "+" { // addition
            return Ok(Token::Add);
        } else if text == "*" { // geometric product / multiplication
            return Ok(Token::Prod);
        } else if text == "/" { // Geometric Division
            return Ok(Token::Div);
        } else if text == ">>" { // Left Contraction
            return Ok(Token::LCont);
        } else if text == "<<" { // Right Contraction
            return Ok(Token::RCont);
        } else if text == "^" { // Outer Product
            return Ok(Token::Outer);
        }  else if text == "inverse" { // Inversion
            return Ok(Token::Invert);
        } else if text == "involution" { // Inversion
            return Ok(Token::Involute);
        } else if text == "reverse" { // Inversion
            return Ok(Token::Reverse);
        } else if text == "dual" { // dual
            return Ok(Token::Dual);
        } else if text == "undual" { // undual
            return Ok(Token::Undual);
        } else if text == "generate_I" { // Generate Pseudoscalar
            return Ok(Token::GenI);
        } else if text == "&" { // Minimum
            return Ok(Token::Min);
        } else if text == "|" { // Maximum
            return Ok(Token::Max);
        } else if text == "==" { // Equality
            return Ok(Token::Equals);
        } else if text == "<" { // Less Than
            return Ok(Token::LessThan);
        } else if text == ">" { // Greater Than
            return Ok(Token::GreaterThan);
        } else if text == "<=" { // Less Than Or Equal
            return Ok(Token::LessThanOrEqual);
        } else if text == ">=" { // Greater Than Or Equal
            return Ok(Token::GreaterThanOrEqual);
        } else if text == "!=" { // Not Equal
            return Ok(Token::NotEqual);
        } else if text == "mag" { // Magnitude
            return Ok(Token::Mag);
        } else if text == "~" { // Negative/Negate
            return Ok(Token::Neg);
        } else if text == "-" { // subtraction
            return Ok(Token::Sub);
        }

        // text == "sel_grade" { // grade selection
        //     return Ok(Token::SelGrade);
        // }

        Err(String::from(format!("Text '{}' is not a recognized token.", text)))
    }
}

const REGEX_BASIS: &str = r"[PNZ]\\(\d\*\\)";
const REGEX_COMPONENT: &str = r"[+-]?\d\*\\.?\d\*([PNZ]\\(\d\*\\))*";
const REGEX_MV: &str = r"[+-]?\d\*\\.?\d\*([PNZ]\\(\d\*\\))*([+-]\d\*\\.?\d\*([PNZ]\\(\d\*\\))*)*";