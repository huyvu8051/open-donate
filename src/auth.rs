use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[cfg(feature = "ssr")]
pub mod handlers {
    use axum::{
        extract::Query,
        response::{IntoResponse, Redirect},
    };
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct CallbackParams {
        pub code: String,
        pub state: Option<String>,
    }

    #[derive(Deserialize)]
    struct TokenResponse {
        id_token: String,
    }

    pub async fn login() -> impl IntoResponse {
        let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();
        let redirect_uri = std::env::var("OIDC_REDIRECT_URI").unwrap_or_default();
        let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();

        let encoded_redirect = urlencoding::encode(&redirect_uri);
        let url = format!(
            "{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid+profile+email&state=state123&nonce=nonce123",
            issuer, client_id, encoded_redirect
        );

        Redirect::to(&url)
    }

    pub async fn callback(
        session: tower_sessions::Session,
        Query(params): Query<CallbackParams>,
    ) -> impl IntoResponse {
        let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();
        let client_secret = std::env::var("OIDC_CLIENT_SECRET").unwrap_or_default();
        let redirect_uri = std::env::var("OIDC_REDIRECT_URI").unwrap_or_default();
        let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();

        let token_url = format!("{}/protocol/openid-connect/token", issuer);
        let client = reqwest::Client::new();

        let form_data = [
            ("grant_type", "authorization_code"),
            ("code", &params.code),
            ("redirect_uri", &redirect_uri),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
        ];

        match client.post(&token_url).form(&form_data).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    match resp.json::<TokenResponse>().await {
                        Ok(token_resp) => {
                            // Validate JWT and get User
                            match crate::auth::validation::validate_jwt(&token_resp.id_token).await {
                                Ok(user) => {
                                    if let Err(e) = session.insert("user", user).await {
                                        eprintln!("Failed to save session: {:?}", e);
                                    }
                                    return Redirect::to("/dashboard").into_response();
                                }
                                Err(e) => {
                                    eprintln!("Failed to validate JWT: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse token JSON: {:?}", e);
                        }
                    }
                } else {
                    let text = resp.text().await.unwrap_or_default();
                    eprintln!("Token exchange failed (status {}): {}", status, text);
                }
            }
            Err(e) => {
                eprintln!("Token exchange request failed: {:?}", e);
            }
        }

        Redirect::to("/dashboard").into_response()
    }

    pub async fn logout(session: tower_sessions::Session) -> impl IntoResponse {
        let _ = session.delete().await;
        Redirect::to("/").into_response()
    }
}

#[cfg(feature = "ssr")]
pub mod validation {
    use super::User;
    use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
    use serde::Deserialize;
    use std::sync::OnceLock;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Jwk {
        alg: String,
        kid: String,
        n: String,
        e: String,
    }

    #[derive(Debug, Deserialize)]
    struct Jwks {
        keys: Vec<Jwk>,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Claims {
        sub: String,
        name: Option<String>,
        email: Option<String>,
        iss: String,
        aud: serde_json::Value,
        exp: u64,
    }

    static JWKS_CACHE: OnceLock<Jwks> = OnceLock::new();

    async fn fetch_jwks(issuer: &str) -> Result<&'static Jwks, String> {
        if let Some(jwks) = JWKS_CACHE.get() {
            return Ok(jwks);
        }

        let url = format!("{}/protocol/openid-connect/certs", issuer);
        let resp = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

        let jwks = resp
            .json::<Jwks>()
            .await
            .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

        let _ = JWKS_CACHE.set(jwks);

        JWKS_CACHE
            .get()
            .ok_or_else(|| "Failed to get JWKS from cache".to_string())
    }

    pub async fn validate_jwt(token: &str) -> Result<User, String> {
        let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();
        let issuer = std::env::var("OIDC_ISSUER").unwrap_or_default();

        // 1. Decode header to get `kid`
        let header = decode_header(token).map_err(|e| format!("Invalid JWT header: {}", e))?;
        let kid = header
            .kid
            .ok_or_else(|| "JWT missing kid header".to_string())?;

        // 2. Fetch JWKS
        let jwks = fetch_jwks(&issuer).await?;

        // 3. Find matching JWK
        let key = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| "Matching JWK key not found".to_string())?;

        // 4. Create decoding key
        let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
            .map_err(|e| format!("Failed to create decoding key: {}", e))?;

        // 5. Setup validation (verify client_id audience and issuer)
        let mut validation = Validation::new(header.alg);
        validation.set_audience(&[client_id]);
        validation.set_issuer(&[issuer]);

        // 6. Decode and verify claims
        let token_data = decode::<Claims>(token, &decoding_key, &validation)
            .map_err(|e| format!("Failed to decode/validate JWT: {}", e))?;

        let claims = token_data.claims;

        Ok(User {
            id: claims.sub,
            name: claims.name.unwrap_or_else(|| "User".to_string()),
            email: claims.email.unwrap_or_default(),
        })
    }
}
