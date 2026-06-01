use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[cfg(feature = "ssr")]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[server(LoginWithEmail, "/api")]
pub async fn login_with_email(email: String, password: String) -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;

    let pool = extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|_| ServerFnError::new("Database connection error"))?;

    let record = sqlx::query("SELECT id, name, email, password_hash FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&*pool)
        .await
        .map_err(|_| ServerFnError::new("Database query failed"))?;

    if let Some(record) = record {
        use sqlx::Row;
        let password_hash_str: String = record.get("password_hash");
        let parsed_hash = PasswordHash::new(&password_hash_str)
            .map_err(|_| ServerFnError::new("Password hash error"))?;
            
        if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
            let user = User {
                id: record.get("id"),
                name: record.get("name"),
                email: record.get("email"),
            };

            let session = extract::<Session>().await
                .map_err(|_| ServerFnError::new("Failed to get session"))?;

            session.insert("user", user).await.map_err(|_| ServerFnError::new("Session insert failed"))?;

            Ok(())
        } else {
            Err(ServerFnError::new("Invalid email or password."))
        }
    } else {
        Err(ServerFnError::new("Invalid email or password."))
    }
}

#[server(RegisterWithEmail, "/api")]
pub async fn register_with_email(email: String, password: String, password_confirm: String) -> Result<(), ServerFnError> {
    if password != password_confirm {
        return Err(ServerFnError::new("Passwords do not match."));
    }
    
    if password.len() < 6 {
        return Err(ServerFnError::new("Password is too weak (minimum 6 characters)."));
    }

    use leptos_axum::extract;
    use tower_sessions::Session;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|_| ServerFnError::new("Failed to hash password"))?
        .to_string();

    let pool = extract::<axum::Extension<sqlx::PgPool>>().await
        .map_err(|_| ServerFnError::new("Database connection error"))?;

    let user_id = uuid::Uuid::new_v4().to_string();
    let name = email.split('@').next().unwrap_or("User").to_string();

    let result = sqlx::query(
        "INSERT INTO users (id, name, email, password_hash) VALUES ($1, $2, $3, $4)"
    )
    .bind(&user_id)
    .bind(&name)
    .bind(&email)
    .bind(&password_hash)
    .execute(&*pool)
    .await;

    match result {
        Ok(_) => {
            let user = User {
                id: user_id,
                name,
                email,
            };

            let session = extract::<Session>().await
                .map_err(|_| ServerFnError::new("Failed to get session"))?;

            session.insert("user", user).await.map_err(|_| ServerFnError::new("Session insert failed"))?;

            Ok(())
        },
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code().as_deref() == Some("23505") { // unique constraint violation
                    return Err(ServerFnError::new("This email is already registered."));
                }
            }
            leptos::logging::error!("Register DB error: {}", e);
            Err(ServerFnError::new("Registration failed due to database error."))
        }
    }
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;

    let session = extract::<Session>().await
        .map_err(|_| ServerFnError::new("Failed to get session"))?;
    
    session.delete().await.map_err(|_| ServerFnError::new("Failed to delete session"))?;
    
    Ok(())
}


