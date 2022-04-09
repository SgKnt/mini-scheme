use regex::Regex;

fn main() {
    let re = Regex::new(r"^\.[\(\)'\s]?$").unwrap();
    assert!(re.is_match(" . ".split_at(1).1));
}
