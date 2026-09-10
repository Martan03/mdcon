use std::process::ExitCode;

use args::Args;
use generate::Gen;
use pareg::Pareg;
use termal::{eprintcln, printcln};

use crate::error::Error;

mod args;
mod error;
mod generate;

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
    let generator = Gen::parse(&content);
    let toc = generator.gen_toc(args.max_ident);

    if args.dump {
        print!("{toc}");
        return Ok(());
    }

    let text = generator.insert_toc(&content, &toc);
    if eq_semantic(&content, &text) {
        printcln!(
            "{'b}Info:{'_} TOC in {} is already up to date.",
            args.md_file.to_string_lossy()
        );
        return Ok(());
    }

    if args.check {
        return Err(
            "TOC check failed (out of date). Run without --check to update."
                .into(),
        );
    }

    std::fs::write(&args.md_file, text)?;
    printcln!(
        "{'g}Success:{'_} Updated TOC in {}.",
        args.md_file.to_string_lossy()
    );
    Ok(())
}

fn eq_semantic(orig: &str, new: &str) -> bool {
    let orig_lines = orig.lines().map(str::trim).filter(|l| !l.is_empty());
    let new_lines = new.lines().map(str::trim).filter(|l| !l.is_empty());
    orig_lines.eq(new_lines)
}
