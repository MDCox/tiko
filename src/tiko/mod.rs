pub mod lexer;
mod parser;

mod ast;
mod token;

pub fn eval(inp: String) {
    lexer::lex(inp);
}
