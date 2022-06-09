use clap::Parser;
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    let stdout = stdout();
    let mut message = String::from("Hello ");
    message.push_str(&args.name);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
