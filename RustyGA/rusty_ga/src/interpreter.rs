use std::collections::{HashMap, VecDeque};

use crate::{basis::ONBasis, multivector::Multivector};

/// # Translator
/// 
/// Takes in strings, translates, and stores any values and results from the CLI.
/// 
/// May eventually be upgraded to a full interpreter.
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
        for token in line.split_ascii_whitespace() {
            // TODO Pick Up Here
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
    IsBlade,
    EndStatement,
    Basis,
    Component,
    Multivector,
    Id
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
            Token::IsBlade => "is_blade?",
            Token::Id => "[a-zA-Z][a-zA-Z0-9]*",
            // Solo Basis
            Token::Basis => "[PNZ]\\(\\d*\\)",
            // #(Basis)*
            Token::Component => "\\d*\\.?\\d*([PNZ]\\(\\d*\\))*",
            // Component(+component)+
            Token::Multivector => 
            r"\d\*\.?\d\*([PNZ]\\(\d+\\))\*([+-]\d\*\.?\d\*([PNZ]\\(\d+\\))\*)\*",
            Token::EndStatement => ";",
        }
    }
}

const REGEX_BASIS: &str = r"[PNZ]\\(\d\*\\)";
const REGEX_COMPONENT: &str = r"[+-]?\d\*\\.?\d\*([PNZ]\\(\d\*\\))*";
const REGEX_MV: &str = r"[+-]?\d\*\\.?\d\*([PNZ]\\(\d\*\\))*([+-]\d\*\\.?\d\*([PNZ]\\(\d\*\\))*)*";