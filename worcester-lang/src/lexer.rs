struct Lexer {
    current_char: String,
    text_body: String,
    index: i64, // i64 instead of usize, because remember, we don't care about the users, we care about ourselves.
    tokens: Vec<Token>
}
impl Lexer {
    fn new(body: String) -> Self {
        Lexer {
            current_char: String::new(),
            text_body: body,
            index: -1 // because we want to start from 0.
        }
    }
}