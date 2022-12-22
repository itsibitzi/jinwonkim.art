use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::http::header::{AUTHORIZATION, WWW_AUTHENTICATE};
use axum::http::{HeaderMap, StatusCode};

use super::database::Database;
use super::password::verify_password;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AuthBasic(pub (String, String));

#[async_trait]
impl<B> FromRequest<B> for AuthBasic
where
    B: Send,
{
    type Rejection = (StatusCode, HeaderMap, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> std::result::Result<Self, Self::Rejection> {
        let ok_headers = HeaderMap::new();
        let mut rejection_headers = HeaderMap::new();
        rejection_headers.append(WWW_AUTHENTICATE, "Basic".parse().unwrap());

        // Get authorisation header
        let authorisation = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or((
                StatusCode::UNAUTHORIZED,
                rejection_headers,
                "`Authorization` header is missing",
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    ok_headers.clone(),
                    "`Authorization` header contains invalid characters",
                )
            })?;

        // Check that its a well-formed basic auth then decode and return
        let split = authorisation.split_once(' ');
        match split {
            Some((name, contents)) if name == "Basic" => {
                const ERR: (StatusCode, &'static str) = (
                    StatusCode::BAD_REQUEST,
                    "`Authorization` header's basic authentication was improperly encoded",
                );

                // Decode from base64 into a string
                let decoded = base64::decode(contents).map_err(|_| ERR).unwrap();
                let decoded = String::from_utf8(decoded).map_err(|_| ERR).unwrap();

                if let Some((id, password)) = decoded.split_once(':') {
                    Ok(AuthBasic((id.to_string(), password.to_string())))
                } else {
                    Err((StatusCode::BAD_REQUEST, ok_headers, "Password not presetn"))
                }
            }
            _ => Err((
                StatusCode::BAD_REQUEST,
                ok_headers,
                "`Authorization` header must be for basic authentication",
            )),
        }
    }
}

pub async fn check_password_for_user(username: &str, password: &str, db: &Database) -> bool {
    if let Ok(Some(user)) = db.get_user(username).await {
        match verify_password(password, &user.password_hash) {
            Ok(true) => true,
            _ => false,
        }
    } else {
        false
    }
}
