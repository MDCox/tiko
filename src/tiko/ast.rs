#[derive(Show)]
// All possible Expressions in Tiko.
enum Expr<'a> {
    // Num -- numeric litarals (eg: 1, 2.1)
    //        args: value
    Num(usize),

    // Var -- variable reference
    //        args: variable Name
    Var(&'a str),
    
    // Bin -- binary operator. args: operator, left expr, right expr
    Bin(char, Box<Expr<'a>>, Box<Expr<'a>>),

    // Inv -- function invocation/call.
    //        args: callee, args
    Inv(&'a str, Vec<Expr<'a>>)
}

// Representation of the "prototype" of a function.
// Has it's name and arguments.
struct Prototype {
    name: String,
    args: Vec<String>,
}

// The actual definition of a function.
struct Function<'a> {
    proto: Prototype,
    body:  Expr<'a>,
}
