use std::env::VarError;

pub trait StringExtensions {
    fn to_boolean(self) -> bool;
}

impl StringExtensions for String {
    fn to_boolean(self) -> bool {
        self.eq_ignore_ascii_case("true")
    }
}

impl StringExtensions for Result<String, VarError> {
    fn to_boolean(self) -> bool {
        self.map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or_else(|_| false)
    }
}
