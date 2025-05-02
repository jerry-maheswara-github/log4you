use uuid::Uuid;

/// Trait for conversion between UUID and log_id format (32 char without '-')
/// This version of the trait can be used as a dyn trait.
pub trait LogIdFormat {
    fn to_log_id(&self) -> String;
    fn from_log_id(log_id: &str) -> Option<Uuid>;
}

/// Implements `LogIdFormat` for `Uuid`, providing methods to convert
/// to and from a hyphen-less log ID string.
impl LogIdFormat for Uuid {
    /// Converts the UUID to a 32-character, hyphen-less string.
    fn to_log_id(&self) -> String {
        self.to_string().replace("-", "")
    }

    /// Parses a 32-character hyphen-less log ID string back into a `Uuid`.
    /// Returns `None` if the string is not valid or incorrectly formatted.
    fn from_log_id(log_id: &str) -> Option<Uuid> {
        if log_id.len() != 32 {
            return None;
        }

        let formatted = format!(
            "{}-{}-{}-{}-{}",
            &log_id[0..8],
            &log_id[8..12],
            &log_id[12..16],
            &log_id[16..20],
            &log_id[20..32],
        );

        Uuid::parse_str(&formatted).ok()
    }
}
