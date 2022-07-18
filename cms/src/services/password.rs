use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn verify_password(password: &str, phc: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(phc)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
