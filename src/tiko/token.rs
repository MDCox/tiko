#[derive(Show)]
pub enum Token {
    Eof,
    Def,
    Get,
    Id(String),
    Num(usize),
}
