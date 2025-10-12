use super::*;

pub use app::Result;
pub use app::Error;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Markdown(pub String);

impl From<String> for Markdown {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<::std::path::PathBuf> for Markdown {
    type Error = app::Error;

    fn try_from(value: ::std::path::PathBuf) -> ::std::result::Result<Self, Self::Error> {
        Ok(Self(::std::fs::read_to_string(value)?))
    }
}