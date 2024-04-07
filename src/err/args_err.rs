use std::fmt::Display;

/// Args error enum
pub enum ArgsErr {
    MissingOp(String),
    Unexpected(String),
    ExitSucc,
}

impl Display for ArgsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::MissingOp(s) => format!("'{s}' expects argument"),
            Self::Unexpected(s) => format!("unexpected argument: '{s}'"),
            Self::ExitSucc => String::new(),
        };
        write!(f, "{}", res)
    }
}
