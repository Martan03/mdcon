use std::path::PathBuf;

use pareg::Pareg;
use termal::printcln;

use crate::error::Error;

/// Struct for parsing input arguments
pub struct Args {
    pub md_file: PathBuf,
    pub max_ident: usize,
    pub dump: bool,
    pub check: bool,
    pub should_exit: bool,
}

impl Args {
    pub const VERSION_NUMBER: &str = {
        let v = option_env!("CARGO_PKG_VERSION");
        if let Some(v) = v {
            v
        } else {
            "unknown"
        }
    };

    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Self::default();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-f" | "--file" => parsed.md_file = args.next_arg()?,
                "-m" | "--max-ident" => parsed.max_ident = args.next_arg()?,
                "-d" | "--dump" => parsed.dump = true,
                "-c" | "--check" => parsed.check = true,
                "-h" | "--help" | "help" => {
                    Self::help();
                    parsed.should_exit = true;
                    break;
                }
                "-v" | "--version" => {
                    Self::version();
                    parsed.should_exit = true;
                    break;
                }
                _ => return args.err_unknown_argument().err()?,
            }
        }
        Ok(parsed)
    }

    /// Prints the help.
    pub fn help() {
        printcln!(
            "Welcome to {'g}mdcon{'_} by {}{'_}
{'bl}Version {}{'_}

{'g}Usage{'_}:
  {'c}mdcon{'_}
    Generates contents for 'README.md'.

  {'c}mdcon{'_} [{'y}flags{'_}]
    Behaves according to the flags.

{'g}Flags{'_}:
  {'y}-d  --dump{'_}
    Dump table of contents to the terminal.

  {'y}-c  --check{'_}
    Checks whether TOC is up to date.

  {'y}-f  --file{'_}
    File to generate contents for.

  {'y}-h  --help{'_}
    Displays this help.

  {'y}-v  --version{'_}
    Displays the version number of {'c}mdcon{'_}.",
            termal::gradient("Martan03", (0, 220, 255), (175, 80, 255)),
            Self::VERSION_NUMBER,
        );
    }

    /// Prints the app version
    pub fn version() {
        println!("mdcon {}", Self::VERSION_NUMBER)
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            md_file: PathBuf::from("README.md"),
            max_ident: 6,
            dump: false,
            check: false,
            should_exit: false,
        }
    }
}
