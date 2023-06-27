use std::fmt;

/// `&'static str` newtype whose `Debug` / `Display` impl do not output double
/// quotes.
#[derive(PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TypeNameLit(pub &'static str);

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

#[cfg(test)]
mod tests {
    use super::TypeNameLit;

    #[test]
    fn debug() {
        let type_name_lit = TypeNameLit("A");
        assert_eq!("\"A\"", format!("{type_name_lit:?}"));
    }

    #[test]
    fn display() {
        let type_name_lit = TypeNameLit("A");
        assert_eq!("A", format!("{type_name_lit}"));
    }
}
