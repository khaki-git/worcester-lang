use crate::structures;

pub struct Lexer {
    current_char: String,
    text_body: String,
    index: i64, // i64 instead of usize, because remember, we don't care about the users, we care about ourselves.
    tokens: Vec<structures::Token>
}
impl Lexer {
    pub fn new(body: String) -> Self {
        Lexer {
            current_char: String::new(),
            text_body: body,
            index: -1, // because we want to start from 0.
            tokens: vec![],
        }
    }

    fn advance(&mut self) {
        // advance to the next line
        self.index += 1;

        if self.index >= self.text_body.len() as i64 {
            self.current_char = String::new() // Leave it empty, so we know that it's done
        } else {
            self.current_char = self.text_body.clone().chars().nth(self.index as usize).unwrap().to_string();
        }
    }

    pub fn parse(&mut self) -> Vec<structures::Token> {
        // TODO: implement this.
        vec![] // Give an empty Vec to avoid compiler errors
    }
}