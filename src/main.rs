#![allow(unstable)]

extern crate getopts;

use std::io;
use std::os;

mod tiko;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = os::args();
    let ref program = args[0];

    let opts = [
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("v", "version", "output version information and exit"),
    ];

    // Matched options and arguments
    let matches = match getopts::getopts(args.tail(), &opts) {
        Ok(m)   => m,
        Err(e)  => {
            println!("{:?}", e);
            os::set_exit_status(1);
            return;
        }
    };

    if matches.opt_present("help") {
        println!("tiko {:?} - the concise and precise language", VERSION);
        println!("Usage:");
        println!("  {} OPTION", program);
        print!("{}", &*getopts::usage("", &opts));
        return;
    }

    if matches.opt_present("version") {
        println!("tiko version: {:?}", VERSION);
        return
    }
   
    // Non option arguments.
    // In this case a filename.
    if matches.free.is_empty() { 
        repl()
    } else {
        let filename = matches.free[0].clone();
        intp(filename)
    }
}

// Runs an interactive REPL
fn repl() {
    loop {
        print!("> ");
        let inp = io::stdin()
                    .read_line()
                    .ok()
                    .expect("Failed to read line");
        tiko::lexer::lex(inp);
    }
}

// Runs loaded code from a file
fn intp(filename: String) {
    // load file
    let path = Path::new(filename);
    let file = io::File::open(&path)
                 .read_to_string()
                 .unwrap();
        
    tiko::lexer::lex(file);
}
