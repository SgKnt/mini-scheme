use std::io;

fn main() {
    let mut buf = String::new();
    let mut v: Vec<char> = Vec::new();
    io::stdin().read_line(&mut buf).expect("error");
    for c in buf.chars() {
        v.push(c);
    }
    println!("{:?}", v);
}
