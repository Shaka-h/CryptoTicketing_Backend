pub mod date_format {
    use serde::{self, Deserialize, Serializer};
    use chrono::NaiveDateTime;
    use chrono::format::ParseError;

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S"; // Expected format: "2024-12-25 19:30:00"

    // Serialize the NaiveDateTime into the expected format
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = date.format(FORMAT).to_string();
        serializer.serialize_str(&formatted)
    }

    // Deserialize the string into NaiveDateTime
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date_str = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&date_str, FORMAT).map_err(serde::de::Error::custom)
    }
}
