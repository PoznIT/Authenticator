use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_str(to_hash: &str) -> String {
    Argon2::default()
        .hash_password(to_hash.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string()
}

pub fn verify_hash(to_verify: &str, hash: &str) -> bool {
    Argon2::default()
        .verify_password(
            to_verify.as_bytes(), &PasswordHash::new(hash).unwrap())
        .is_ok()
}
