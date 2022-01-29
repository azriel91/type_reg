use std::fmt;

/// `&'static str` newtype whose `Debug` / `Display` impl do not output double
/// quotes.
#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TypeNameLit(pub(crate) &'static str);

impl fmt::Debug for TypeNameLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for TypeNameLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
