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

    let gen = Gen::parse(&args.md_file)?;
    gen.gen(&args)?;
    Ok(())
}
