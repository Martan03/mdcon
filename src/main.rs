use args::Args;
use gen::Gen;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};

mod args;
mod err;
mod gen;

fn main() {
    let args = match Args::parse(std::env::args()) {
        Ok(args) => args,
        Err(e) => {
            printe(e.to_string());
            return;
        }
    };

    let gen = match Gen::parse(&args, &args.md_file) {
        Ok(gen) => gen,
        Err(e) => {
            printe(e.to_string());
            return;
        }
    };

    println!("{}", gen.gen());
}

fn printe(text: String) {
    if text.is_empty() {
        return;
    }
    eprintln!("{} {text}", "Error:".fg(Fg::Red));
}
