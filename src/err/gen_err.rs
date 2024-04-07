use std::fmt::Display;

/// Generating error enum
pub enum GenErr {
    FileAccess(String),
}

impl Display for GenErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::FileAccess(name) => format!("cannot access file: '{name}'"),
        };
        write!(f, "{}", res)
    }
}
