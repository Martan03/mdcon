use args::Args;

mod args;

fn main() {
    let args = Args::parse(std::env::args());
}
