use bcrypt::{hash, verify};

pub struct Hasher;

impl Hasher {
    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        // Hash the password using bcrypt
        hash(password, bcrypt::DEFAULT_COST)
    }

    pub fn verify_password(password: &str, hash: &str) -> bool {
        // Verify the password against its hash using bcrypt
        match verify(password, hash) {
            Ok(result) => result,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::etc::hasher::Hasher;

    #[test]
    fn it_should_hash_password_and_verify() {
        const PASSWORD: &str = "password";
        let hashed_password = Hasher::hash_password(PASSWORD).unwrap();
        let is_valid = Hasher::verify_password(PASSWORD, &hashed_password);
        assert_eq!(is_valid, true);
    }
}
