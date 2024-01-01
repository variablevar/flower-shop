use regex::Regex;
use serde::{Deserialize, Deserializer};

pub struct Validations;

impl Validations {
    pub fn validate_email<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let email: String = Deserialize::deserialize(deserializer)?;

        // Regular expression pattern for basic email validation
        let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
        if re.is_match(&email) {
            Ok(email)
        } else {
            Err(serde::de::Error::custom("Invalid email format"))
        }
    }

    pub fn validate_mobile_number<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mobile_number: String = Deserialize::deserialize(deserializer)?;

        // Perform mobile number validation using a regex pattern
        let re = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
        if re.is_match(&mobile_number) {
            Ok(mobile_number)
        } else {
            Err(serde::de::Error::custom("Invalid mobile number format"))
        }
    }
    pub fn validate_zip_code<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let zip_code: i64 = Deserialize::deserialize(deserializer)?;

        let re = Regex::new(r"^\d{6}$").unwrap(); // Regex pattern for exactly 6 digits
        if re.is_match(&zip_code.to_string()) {
            Ok(zip_code)
        } else {
            Err(serde::de::Error::custom("Invalid zip code format"))
        }
    }
}
