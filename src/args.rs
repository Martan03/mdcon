use termint::{
    enums::fg::Fg,
    help,
    widgets::{grad::Grad, span::StrSpanExtension},
};

use crate::err::args_err::ArgsErr;

/// Struct for parsing input arguments
pub struct Args {
    pub md_file: String,
    pub skip_title: bool,
}

impl Args {
    /// Parses given arguments
    pub fn parse(args: std::env::Args) -> Result<Args, ArgsErr> {
        let mut parsed = Self {
            md_file: "README.md".to_string(),
            skip_title: true,
        };

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    Args::help();
                    return Err(ArgsErr::ExitSucc);
                }
                "-f" | "--file" => {
                    parsed.md_file = Args::get_next(&arg, &mut args_iter)?;
                }
                "-a" | "--all" => parsed.skip_title = false,
                a => return Err(ArgsErr::Unexpected(a.to_string())),
            }
        }
        Ok(parsed)
    }

    /// Gets next arguments
    fn get_next<T>(arg: &str, args: &mut T) -> Result<String, ArgsErr>
    where
        T: Iterator<Item = String>,
    {
        let Some(arg) = args.next() else {
            return Err(ArgsErr::MissingOp(arg.to_string()));
        };
        Ok(arg)
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
            "mdcom" => "Generates contents for 'README.md'\n"
            "mdcom" ["options"] =>
                "Works based on given options\n"
            "Options":
            "-a  --all" => "Doesn't skip the title\n"
            "-f  --file" ["file"] => "File to generate contents for\n"
            "-h  --help" => "Prints this help (other options are ignored)"
        );
    }
}
