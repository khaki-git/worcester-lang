use crate::structures::{TokenTypes, Token, ascii_letters};

pub struct Lexer {
    current_char: String,
    text_body: String,
    index: i64, // i64 instead of usize, because remember, we don't care about the users, we care about ourselves.
    pub tokens: Vec<Token>
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

    pub fn parse(&mut self) { // TODO: Finish
        self.advance();
        while self.current_char != String::new() {
            if self.current_char == "\"".to_string() {
                // TODO: Strings are not recognize, should be fixed
                let string_contents = self.make_string();

                self.tokens.push(
                    Token {
                        token_types: TokenTypes::STR,
                        body: string_contents,
                    }
                );
            } else if ascii_letters().contains(self.current_char.as_str()) {
                // make identifier
                // TODO: Sometimes identifiers exclude the first letter for some reason.
                let identifier = self.make_identifier();

                self.tokens.push(
                    Token {
                        token_types: TokenTypes::KEY,
                        body: identifier,
                    }
                );
            }

            self.advance();
        }
    }

    fn make_string(&mut self) -> String {
        let mut str_content = String::new();

        while
            self.current_char != String::new() &&
                self.current_char != "\"".to_string() { // indentation hell
            self.make_string().push_str(self.current_char.as_str());
            self.advance();
        }

        self.advance();

        str_content
    }

    fn make_identifier(&mut self) -> String {
        let mut identifier = String::new();

         while ascii_letters().contains(self.current_char.as_str()) {
             identifier.push_str(self.current_char.as_str());
             self.advance();
         }

        self.advance();

         identifier
    }
}

pub fn repr_tokens(tokens: Vec<Token>) {
    for token in tokens {
        println!("{:?}", token);
    }
}