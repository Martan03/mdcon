use std::process::ExitCode;

use args::Args;
use gen::Gen;
use pareg::Pareg;
use termal::eprintcln;

use crate::error::Error;

mod args;
mod error;
mod gen;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintcln!("{'r}Error:{'_} {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(Pareg::args())?;
    if args.should_exit {
        return Ok(());
    }

    let content = std::fs::read_to_string(&args.md_file)?;
    let gen = Gen::parse(&content);
    let toc = gen.gen_toc(args.max_ident);

    if args.dump {
        print!("{toc}");
    } else {
        let text = gen.insert_toc(&content, &toc);
        std::fs::write(&args.md_file, text)?;
    }
    Ok(())
}
