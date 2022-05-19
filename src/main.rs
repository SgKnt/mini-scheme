mod parse;
mod token;
mod data;
mod eval;
mod function;

use std::io::{self, Write};

use eval::eval;
use parse::Parser;
use data::{*, memory::*};

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    print!(">>> ");
    io::stdout().flush()?;
    loop {
        io::stdin().read_line(&mut buf)?;

        let has_read = !at_unterminated_paren_or_string(&buf);
        if has_read {
            break;
        }
        print!("... ");
        io::stdout().flush()?;
    }
    Ok(buf)
}

fn at_unterminated_paren_or_string(buf: &str) -> bool {
    let mut nest = 0;           // the number of layers of nesting ()
    let mut in_str = false;    // between " " ?

    let mut prev_c = b' ';
    for c in buf.as_bytes() {
        if !in_str {
            match c {
                b'(' => nest += 1,
                b')' => nest = std::cmp::max(0, nest - 1),
                b'"' if prev_c != b'\\' => {
                    in_str = true;
                }
                _ => {}
            }
        } else {
            if *c == b'"' && prev_c != b'\\' {
               in_str = false;
            }
        }
        prev_c = *c;
    }
    nest != 0 || in_str
}

fn main() {
    Memory::init(1024);
    let global_env = Environment::new_global(function::make_lib());
    loop {
        let input = read_stdin().unwrap();
        if input.len() == 0 {
            break;
        }
        let lex = Parser::new(input);
        let tokens = lex.build_tokens();
        for token in tokens {
            match token {
                Ok(token) => {
                    let res = eval(token, global_env.clone());
                    match res {
                        Ok(obj) => println!("{}", obj),
                        Err(err) => println!("{:?}", err),
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
        Memory::gc();
    }
}
