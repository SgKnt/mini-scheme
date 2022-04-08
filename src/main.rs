pub mod lex;

use std::io;

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    loop {
        io::stdin().read_line(&mut buf)?;

        let has_read = is_outside_paren_or_string(&buf);
        if has_read {
            break;
        }
    }
    Ok(buf.trim().to_string())
}

fn is_outside_paren_or_string(buf: &str) -> bool {
    let mut prev_c = b' ';
    let mut nest = 0;           // the number of layers of nesting
    let mut in_str = false;    // between " " ?
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
    nest == 0 && !in_str
}

fn main() {
    let input = read_stdin().unwrap();
    println!("{}", input);
}
