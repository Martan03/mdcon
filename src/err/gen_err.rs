use std::fmt::Display;

/// Generating error enum
pub enum GenErr {
    FileAccess(String),
    FileWrite(String),
}

impl Display for GenErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::FileAccess(name) => format!("cannot access file: '{name}'"),
            Self::FileWrite(name) => format!("cannot write to file: '{name}'"),
        };
        write!(f, "{}", res)
    }
}
