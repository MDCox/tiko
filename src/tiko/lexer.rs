use super::token::Token;

pub fn lex(inp: String) -> Vec<Token> {
    let mut s: Scanner = Scanner {
        inp: inp,
        pos: 0,
    };

    let mut tokens: Vec<Token> = Vec::new();
    for tok in s {
        println!("{:?}", tok);
        tokens.push(tok);
    }

    tokens
}


struct Scanner {
    inp:     String,
    pos:      usize,
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.inp.len() - 1 { return None } 

        while isspace(self.inp.char_at(self.pos)) { self.pos += 1 }

        let c = self.inp.char_at(self.pos);

        if isalpha(c) {
            let mut ident: String = String::new();

            let mut curr_char = c;
            while isalnum(curr_char) {
                ident.push(curr_char);
                self.pos += 1;
                curr_char = self.inp.char_at(self.pos);
            }

            if ident == "def" { return Some(Token::Def) }
            if ident == "get" { return Some(Token::Get) }

            return Some(Token::Id(ident))
        }

        if isdigit(c) || c == '.' {
            let mut num: String = String::new();
            
            let mut curr_char = c;
            while isdigit(c) || c == '.' {
                num.push(curr_char);
                self.pos += 1;
                curr_char = self.inp.char_at(self.pos);
            }

            return Some(Token::Num(num.parse().unwrap()))
        }

        if c == '#' {
            while self.inp.char_at(self.pos) != '\n' {
                self.pos += 1
            }
        }

        if self.pos >= self.inp.len() - 1 { None } else { Some(Token::Eof) }
    }
}

fn isspace(c: char) -> bool { c == ' ' || c == '\n' }
fn isalpha(c: char) -> bool {(c >= 'a' && 'z' >= c)||( c >= 'A' && 'Z' >= c ) }
fn isdigit(c: char) -> bool { c >= '0' && '9' >= c }
fn isalnum(c: char) -> bool { isalpha(c) || isdigit(c) }
