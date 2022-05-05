mod parse;
mod token;
mod object;
//mod val;
mod env;
mod eval;
mod function;

use std::io::{self, Write};
use std::rc::Rc;

use env::Environment;
use eval::eval;
use parse::Parser;

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
    let global_env = Rc::new(Environment::new_global());
    loop {
        let input = read_stdin().unwrap();
        if input.len() == 0 {
            break;
        }
        let lex = Parser::new(input);
        let tokens = lex.build_tokens();
        for token in &tokens {
            match token {
                Ok(token) => {
                    let res = eval(token, &global_env);
                    match res {
                        Ok(obj) => println!("{}", obj.borrow()),
                        Err(err) => println!("{:?}", err),
                    }
                }
                Err(err) => println!("{:?}", err),
            }
        }
    }
}
