use actix_multipart::Multipart;
use actix_web::dev::ServiceRequest;
use actix_web::web::Bytes;
use actix_web::{error, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use futures::TryStreamExt;

use crate::args::ARGS;

pub async fn auth_validator(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    match (
        ARGS.auth_basic_username.as_ref(),
        ARGS.auth_basic_password.as_ref(),
        creds.password(),
    ) {
        (Some(conf_user), Some(conf_pwd), Some(cred_pwd))
            if creds.user_id() == **conf_user && **conf_pwd == cred_pwd =>
        {
            Ok(req)
        }
        _ => Err((error::ErrorBadRequest("Invalid login details."), req)),
    }
}

pub async fn password_from_multipart(mut payload: Multipart) -> Result<String, Error> {
    let mut password = String::new();

    while let Some(mut field) = payload.try_next().await? {
        if field.name() == Some("password") {
            let password_bytes = field.bytes(1024).await.unwrap_or(Ok(Bytes::new()))?;
            password = String::from_utf8_lossy(&password_bytes).to_string();
        }
    }
    Ok(password)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(password_hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn password_hash_only_accepts_the_original_password() {
        let hash = hash_password("correct password").unwrap();

        assert!(verify_password("correct password", &hash));
        assert!(!verify_password("arbitrary password", &hash));
    }
}
