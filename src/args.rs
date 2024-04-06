use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

/// Args error enum
pub enum ArgsErr {
    Err,
}

/// Struct for parsing input arguments
pub struct Args {
    md_file: String,
}

impl Args {
    /// Parses given arguments
    pub fn parse(args: std::env::Args) -> Result<Args, ArgsErr> {
        let mut parsed = Self {
            md_file: "".to_string(),
        };

        let mut args_iter = args.into_iter();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-h" | "--help" => Args::help(),
                md => parsed.md_file = md.to_string(),
            }
        }
        Ok(parsed)
    }

    /// Prints help
    fn help() {
        println!(
            "Welcome in {} by {}\n",
            "mdcom".fg(Fg::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "mdcom" ["md file"] ["flags"] =>
                "Generates contents for the given file\n"
            "Flags":
            "-h  --help" => "Prints this help (other options are ignored)"
        );
    }
}
