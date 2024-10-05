// Maybe a little bit misleading because we also have enums, but it'll work.

#[derive(PartialEq, Debug)]
pub enum TokenTypes {
    MUL, // Multiplication
    DIV, // Division
    ADD, // Addition
    SUB, // Subtraction
    STR, // String
    INT, // integer
    FLT, // float
    EQL, // equals value
    KEY, // key value
    NIL, // nil value
    CBT, // closed brackets (})
    OBT, // open brackets ({)
    OSB, // open square brackets ([)
    CSB, // closed square brackets (])
    OP, // open parenthesis (())
    CLP, // closed parenthesis (), added L for legal reasons
    OPS, // opposite (!)
    CMA, // comma (,)
}

#[derive(PartialEq)]
pub enum VariableTypes {
    INT,
    FLT,
    STR,
    BLN, // boolean
    NIL, // nil value
    INF, // infinite value

}

#[derive(Debug)]
pub struct Token {
    pub(crate) token_types: TokenTypes,
    pub(crate) body: String
}

// TODO: Implement variables during compile time
pub struct Variable {
    strictly_typed: bool, // If true, then the variable must be of a certain type.
    name: String,
    value: String
}

pub fn ascii_letters() -> String {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_".to_string()
}