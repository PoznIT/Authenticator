use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Clone)]
pub struct CryptoService {
    argon: Argon2<'static>,
}

impl CryptoService {
    pub fn new() -> Self {
        CryptoService {
            argon: Argon2::default(),
        }
    }
    
    pub fn hash_str(&self, to_hash: &str) -> String {
        self.argon
            .hash_password(to_hash.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap()
            .to_string()
    }

    pub fn verify_hash(&self, to_verify: &str, hash: &str) -> bool {
        self.argon
            .verify_password(
                to_verify.as_bytes(), &PasswordHash::new(hash).unwrap())
            .is_ok()
    }
}


