use argon2::{Argon2, PasswordHash, PasswordVerifier};
use tracing::info;

pub fn verify_password(password: &str, phc: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(phc)?;

    info!("Input    '{password}'");
    info!("Database '{phc}'");

    Ok(Argon2::default()
        .verify_password(password.trim().as_bytes(), &parsed_hash)
        .is_ok())
}
