use args::Args;
use gen::Gen;

mod args;
mod gen;

fn main() -> Result<(), String> {
    let args = Args::parse(std::env::args()).map_err(|_| "Ups")?;

    let gen = Gen::parse(&args.md_file).map_err(|_| "Ups")?;
    println!("{}", gen.gen());
    Ok(())
}
