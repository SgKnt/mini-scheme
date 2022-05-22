use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "Mini Scheme",
    version = "1.0.0",
    author = "Kento Sogo",
    about = "Interpreter of Scheme subset"
)]
pub struct Opt {
    #[clap(short, long, name="FILE")]
    pub files: Option<Vec<String>>
}
