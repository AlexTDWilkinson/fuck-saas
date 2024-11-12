use crate::user::user::User;
use crate::AppState;
use aes_gcm::aead::Aead;
use aes_gcm::*;
use axum::extract::Query;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::Form;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use time::format_description;

use time::PrimitiveDateTime;
use tokio::time::Duration;

pub static SESSION_KEY: LazyLock<Key<Aes256Gcm>> =
    LazyLock::new(|| *Key::<Aes256Gcm>::from_slice(&[42; 32]));

const SESSION_MAX_AGE_SECONDS: u32 = 3600; // 4 hours

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoginFormError {
    WrongEmail,
    WrongPassword,
    UnknownError,
    NothingEntered,
    AccountDisabled,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignupFormError {
    InvalidEmail,
    InvalidPassword,
    EmailAlreadyExists,
    NothingEntered,
    HumanCheckAnswerIsWrong,
    UnknownError,
}

use bcrypt::{hash, verify, DEFAULT_COST};

use sqlx::SqlitePool;

impl User {
    pub fn is_admin(&self) -> bool {
        self.permissions.contains("admin")
    }

    pub async fn get_by_id(id: i64, pool: &SqlitePool) -> Option<Self> {
        let server_user = sqlx::query_as!(
            User,
            r#"SELECT 
                id,
                username,
                email,
                password_hash,
                created_at,
                permissions,
                set_password_mode,
                set_password_pin,
                set_password_attempts,
                user_disabled,
                user_deleted
            FROM account WHERE id = ?"#,
            id
        )
        .fetch_optional(pool)
        .await;

        if let Err(err) = server_user {
            println!("get by id error {:?}", err);
            return None;
        }

        match server_user {
            Ok(Some(server_user)) => Some(server_user),
            _ => None,
        }
    }

    pub async fn get_by_email(email: String, pool: &SqlitePool) -> Option<Self> {
        // Case insensitive lookup. Not super performant but probably not a problem for years/decades/ever.
        let server_user = sqlx::query_as!(
            User,
            r#"SELECT 
                id,
                username,
                email,
                password_hash,
                created_at,
                permissions,
                set_password_mode,
                set_password_pin,
                set_password_attempts,
                user_disabled,
                user_deleted
            FROM account WHERE LOWER(email) LIKE LOWER(?)"#,
            email
        )
        .fetch_optional(pool)
        .await;

        if let Err(err) = server_user {
            println!("get by email error {:?}", err);
            return None;
        }

        match server_user {
            Ok(Some(server_user)) => Some(server_user),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct FlackSession {
    pub user_id: i64,
    pub valid_until: i64,
}

impl FlackSession {
    pub fn encrypt(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Generate key and cipher as before
        let key = &SESSION_KEY;
        let cipher = Aes256Gcm::new(&key);

        // Generate a unique nonce
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);

        // Data to encrypt
        let data_to_encrypt = format!("{}|||||{}", self.user_id, self.valid_until);

        // Encrypt
        let ciphertext = cipher.encrypt(
            &Nonce::from_slice(&nonce),
            data_to_encrypt.as_bytes().as_ref(),
        );

        let ciphertext = match ciphertext {
            Ok(ciphertext) => ciphertext,
            Err(err) => {
                println!("encrypt error {:?}", err);
                return Err("Could not encrypt".into());
            }
        };

        // Convert nonce and ciphertext to hexadecimal strings
        let nonce_str = bytes_to_hex_string(&nonce);
        // println!("encrypt nonce str {:?}", nonce_str);
        let ciphertext_str = bytes_to_hex_string(&ciphertext);
        // println!("encrypt ciphertext str {:?}", ciphertext_str);

        // Concatenate nonce and ciphertext as a single string
        Ok(format!("{}{}", nonce_str, ciphertext_str))
    }

    pub fn decrypt(cookie: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Generate key and cipher as before
        let key = &SESSION_KEY;
        let cipher = Aes256Gcm::new(&key);

        // Split the nonce and ciphertext from the cookie

        if cookie.len() <= 24 {
            println!(
                "decrypt cookie length error: {:?} is too short",
                cookie.len()
            );
            return Err("Invalid cookie length".into());
        }

        let (nonce_str, ciphertext_str) = cookie.split_at(24); // Each byte results in 2 hex characters; 12-byte nonce becomes a 24-character string
        let nonce = hex_string_to_bytes(nonce_str);

        let nonce = match nonce {
            Ok(nonce) => nonce,
            Err(err) => {
                println!("decrypt nonce error {:?}", err);
                return Err("Could not decrypt".into());
            }
        };

        let ciphertext = hex_string_to_bytes(ciphertext_str);

        let ciphertext = match ciphertext {
            Ok(ciphertext) => ciphertext,
            Err(err) => {
                println!("decrypt ciphertext error {:?}", err);
                return Err("Could not decrypt".into());
            }
        };

        let plaintext = cipher.decrypt(&Nonce::from_slice(&nonce), ciphertext.as_ref());

        let plaintext = match plaintext {
            Ok(plaintext) => plaintext,
            Err(err) => {
                println!("decrypt error {:?}", err);
                return Err("Could not decrypt".into());
            }
        };
        let plaintext_str = std::str::from_utf8(&plaintext);

        let plaintext_str = match plaintext_str {
            Ok(plaintext_str) => plaintext_str,
            Err(err) => {
                println!("decrypt plaintext str error {:?}", err);
                return Err("Could not decrypt".into());
            }
        };

        // Extract fields and return
        let parts: Vec<&str> = plaintext_str.split("|||||").collect();

        if parts.len() != 2 {
            println!("decrypt parts error {:?}", parts);
            return Err("Could not decrypt".into());
        }

        // println!("parts {:?}", parts);

        let user_id = parts[0].parse();

        let user_id: i64 = match user_id {
            Ok(user_id) => user_id,
            Err(err) => {
                println!("decrypt user id error {:?}", err);
                return Err("Could not decrypt".into());
            }
        };

        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]")
                .map_err(|_| "Invalid format string")?;

        let valid_until: i64 = match PrimitiveDateTime::parse(parts[1], &format) {
            Ok(dt) => dt.assume_utc().unix_timestamp(),
            Err(err) => {
                println!("decrypt valid until error {:?}", err);
                return Err("Could not decrypt".into());
            }
        };

        Ok(Self {
            user_id,
            valid_until,
        })
    }
}

pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

pub fn hex_string_to_bytes(s: &str) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::new();

    for i in 0..(s.len() / 2) {
        let res = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(_) => return Err("Invalid hex string"),
        }
    }

    Ok(bytes)
}

//  create fake use crate::auth::auth::{login, logout, signup};

pub async fn login() {
    println!("login");
}

pub async fn logout() {
    println!("logout");
}

// pub async fn get_user(
//     headers: &HeaderMap,
//     response: &mut axum::http::response::Builder,
//     pool: &sqlx::Pool<Sqlite>,
// ) -> Option<User> {
//     let get_session_cookie_value = get_session_cookie_value(headers).await;

//     // If there is any error in getting the user, we want to delete the session cookie on the user's side to keep things clean/sync'd.
//     let session_cookie_delete =
//         "flack_session=delete; SameSite=Strict; Path=/; Max-Age=0".to_string();

//     let session_cookie_value = match get_session_cookie_value {
//         Some(s) => s,
//         None => {
//             // println!("no session cookie value found");
//             return None;
//         }
//     };

//     let decrypt_session = decrypt_session(&session_cookie_value).await;

//     let session = match decrypt_session {
//         Some(s) => s,
//         None => {
//             response
//                 .headers_mut()
//                 .unwrap()
//                 .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());
//             // println!("no session found");
//             return None;
//         }
//     };

//     let validate_session = validate_session(response, session, pool).await;

//     let session = match validate_session {
//         Some(s) => s,
//         None => {
//             response
//                 .headers_mut()
//                 .unwrap()
//                 .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());
//             // println!("session expired");
//             return None;
//         }
//     };

//     let fetch_db_session = fetch_db_session(pool, &session).await;

//     let db_session = match fetch_db_session {
//         Some(s) => s,
//         None => {
//             response
//                 .headers_mut()
//                 .unwrap()
//                 .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());
//             // println!("no db session found");
//             return None;
//         }
//     };

//     let update_session = update_session(response, db_session, pool).await;

//     let session = match update_session {
//         Some(s) => s,
//         None => {
//             response
//                 .headers_mut()
//                 .unwrap()
//                 .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());
//             // println!("no session found");
//             return None;
//         }
//     };

//     let server_user = match User::get_by_id(session.user_id, pool).await {
//         Some(user) => user,
//         _ => {
//             response
//                 .headers_mut()
//                 .unwrap()
//                 .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());
//             return None;
//         }
//     };

//     Some(server_user)
// }

// async fn get_session_cookie_value_from_headers(headers: &HeaderMap) -> Option<String> {
//     let cookie_header = match headers.get("cookie") {
//         Some(c) => c,
//         None => {
//             // println!("no cookie header found");
//             return None;
//         }
//     };

//     let cookie_str = match cookie_header.to_str() {
//         Ok(s) => s,
//         Err(err) => {
//             println!("cookie str error {:?}", err);
//             return None;
//         }
//     };

//     let cookies: Vec<&str> = cookie_str.split(';').collect();
//     let flack_cookie = cookies.iter().find(|&&c| c.contains("flack_session"))?;

//     let cookie_parts: Vec<&str> = flack_cookie.split('=').collect();

//     let cookie_session = match cookie_parts.get(1) {
//         Some(&v) => v,
//         None => {
//             // println!("no cookie session found");
//             return None;
//         }
//     };

//     // println!("cookie session {:?}", cookie_session);

//     Some(cookie_session.to_string())
// }

// async fn get_session_cookie_value(headers: &HeaderMap) -> Option<String> {
//     let cookie_header = match headers.get("cookie") {
//         Some(c) => c,
//         None => {
//             // println!("no cookie header found");
//             return None;
//         }
//     };

//     let cookie_str = match cookie_header.to_str() {
//         Ok(s) => s,
//         Err(err) => {
//             println!("cookie str error {:?}", err);
//             return None;
//         }
//     };

//     let cookies: Vec<&str> = cookie_str.split(';').collect();
//     let flack_cookie = cookies.iter().find(|&&c| c.contains("flack_session"))?;

//     let cookie_parts: Vec<&str> = flack_cookie.split('=').collect();

//     let cookie_session = match cookie_parts.get(1) {
//         Some(&v) => v,
//         None => {
//             // println!("no cookie session found");
//             return None;
//         }
//     };

//     // println!("cookie session {:?}", cookie_session);

//     Some(cookie_session.to_string())
// }

// async fn decrypt_session(session_cookie_value: &str) -> Option<FlackSession> {
//     let flack_session = match FlackSession::decrypt(session_cookie_value.to_string()) {
//         Ok(s) => s,
//         Err(err) => {
//             return None;
//         }
//     };

//     Some(flack_session)
// }

// async fn validate_session(
//     response: &mut axum::http::response::Builder,
//     session: FlackSession,
//     pool: &sqlx::Pool<Sqlite>,
// ) -> Option<FlackSession> {
//     use axum::http::header;

//     let session_is_invalid = session.valid_until < chrono::Utc::now().naive_utc();

//     if session_is_invalid {
//         let session_cookie_delete =
//             "flack_session=delete; SameSite=Strict; Path=/; Max-Age=0".to_string();

//         response
//             .headers_mut()
//             .unwrap()
//             .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());

//         let _ = sqlx::query("DELETE FROM SESSION WHERE user_id = $1")
//             .bind(session.user_id.clone())
//             .execute(pool)
//             .await;
//         return None;
//     } else {
//         return Some(session);
//     }
// }

// async fn fetch_db_session(
//     pool: &sqlx::Pool<Sqlite>,
//     session: &FlackSession,
// ) -> Option<FlackSession> {
//     let db_session = match sqlx::query_as!(
//         FlackSession,
//         "SELECT * FROM SESSION WHERE user_id = ? AND valid_until = ? LIMIT 1",
//         session.user_id.clone(),
//         session.valid_until.clone()
//     )
//     .fetch_optional(pool)
//     .await
//     {
//         Ok(s) => s,
//         Err(err) => {
//             println!("db session error {:?}", err);
//             return None;
//         }
//     };

//     let db_session = match db_session {
//         Some(s) => s,
//         None => {
//             println!("no db session found");
//             return None;
//         }
//     };

//     Some(db_session)
// }

// async fn update_session(
//     response: &mut axum::http::response::Builder,
//     session: FlackSession,
//     pool: &sqlx::Pool<Sqlite>,
// ) -> Option<FlackSession> {
//     use axum::http::header;
//     // should add 15 mins to valid_until, then update both the db and Cookie

//     let mut updated_session = session.clone();

//     updated_session.valid_until = (chrono::Utc::now().naive_utc()
//         + chrono::Duration::seconds(SESSION_MAX_AGE_SECONDS.into()))
//     .trunc_subsecs(0);

//     // delete all before inserting new

//     let db_delete = sqlx::query("DELETE FROM SESSION WHERE user_id = $1")
//         .bind(updated_session.user_id.clone())
//         .execute(pool)
//         .await;

//     let _ = match db_delete {
//         Ok(s) => s,
//         Err(err) => {
//             println!("db delete error {:?}", err);
//             return None;
//         }
//     };

//     let db_insert =
//         sqlx::query("INSERT INTO SESSION (user_id, valid_until) VALUES ($1, $2) RETURNING *")
//             .bind(updated_session.user_id.clone())
//             .bind(updated_session.valid_until.clone())
//             .execute(pool)
//             .await;

//     let _ = match db_insert {
//         Ok(s) => s,
//         Err(err) => {
//             println!("db insert error {:?}", err);
//             return None;
//         }
//     };

//     let session_encrypted = updated_session.encrypt();

//     let session_encrypted = match session_encrypted {
//         Ok(session_encrypted) => session_encrypted,
//         Err(err) => {
//             println!("session encryption error {:?}", err);
//             return None;
//         }
//     };

//     let session_cookie = format!(
//         "flack_session={}; SameSite=Strict; Path=/; Max-Age={}",
//         session_encrypted, SESSION_MAX_AGE_SECONDS
//     );

//     response
//         .headers_mut()
//         .unwrap()
//         .insert(header::SET_COOKIE, session_cookie.parse().unwrap());

//     Some(updated_session)
// }

// async fn delete_all_session_data(
//     headers: &HeaderMap,
//     response: &mut axum::http::response::Builder,
//     pool: &sqlx::Pool<Sqlite>,
// ) -> Option<FlackSession> {
//     use axum::http::header;

//     // No matter what happens, we want to delete the session cookie on the user's side, so we do that first.
//     let session_cookie_delete =
//         "flack_session=delete; SameSite=Strict; Path=/; Max-Age=0".to_string();
//     response
//         .headers_mut()
//         .unwrap()
//         .insert(header::SET_COOKIE, session_cookie_delete.parse().unwrap());

//     let get_session_cookie_value = get_session_cookie_value(headers).await;

//     let session_cookie_value = match get_session_cookie_value {
//         Some(s) => s,
//         None => {
//             // println!("no session cookie value found");

//             return None;
//         }
//     };

//     let decrypt_session = decrypt_session(&session_cookie_value).await;

//     let session = match decrypt_session {
//         Some(s) => s,
//         None => {
//             // println!("no session found");

//             return None;
//         }
//     };

//     let _ = sqlx::query("DELETE FROM SESSION WHERE user_id = $1")
//         .bind(session.user_id.clone())
//         .execute(pool)
//         .await;

//     Some(session)
// }

#[derive(Deserialize)]
pub struct SignupQuery {
    redirect: Option<String>,
}

#[derive(Deserialize)]
pub struct SignupForm {
    email: String,
    password: String,
    human_check_value: String,
}

#[axum::debug_handler]
pub async fn signup(
    State(state): State<AppState>,
    Query(query): Query<SignupQuery>,
    Form(form): Form<SignupForm>,
) -> impl IntoResponse {
    let redirect = query.redirect.unwrap_or_else(|| "/dashboard".to_string());

    // Check if email and password are provided
    if form.email.is_empty() || form.password.is_empty() {
        return Redirect::to(&format!(
            "/signup?error=missing_fields&redirect={}",
            redirect
        ))
        .into_response();
    }

    // Validate email
    if !is_valid_email(&form.email) {
        return Redirect::to(&format!(
            "/signup?error=invalid_email&redirect={}",
            redirect
        ))
        .into_response();
    }

    // Validate password
    if !is_valid_password(&form.password) {
        return Redirect::to(&format!(
            "/signup?error=invalid_password&redirect={}",
            redirect
        ))
        .into_response();
    }

    // Check human verification
    if form.human_check_value != "1" {
        return Redirect::to(&format!(
            "/signup?error=human_check_failed&redirect={}",
            redirect
        ))
        .into_response();
    }

    // Check if email already exists
    if User::get_by_email(form.email.clone(), &state.pool)
        .await
        .is_some()
    {
        return Redirect::to(&format!("/signup?error=email_exists&redirect={}", redirect))
            .into_response();
    }

    // Hash password
    let password_hashed = match hash(&form.password, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred").into_response(),
    };

    // Insert new user into database
    let user = match sqlx::query_as!(
        User,
        r#"INSERT INTO account (email, password_hash) VALUES ($1, $2) RETURNING *"#,
        form.email,
        password_hashed
    )
    .fetch_one(&state.pool)
    .await
    {
        Ok(user) => user,
        Err(err) => {
            println!("signup error {:?}", err); // server side only log
            return (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred").into_response();
        }
    };

    // Create session (similar to login function)
    let session = FlackSession {
        user_id: user.id,
        valid_until: (time::OffsetDateTime::now_utc()
            + tokio::time::Duration::from_secs(SESSION_MAX_AGE_SECONDS.into()))
        .unix_timestamp(),
    };
    // Encrypt session
    let encrypted_session = match session.encrypt() {
        Ok(encrypted) => encrypted,
        Err(err) => {
            println!("Failed to encrypt session: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred").into_response();
        }
    };

    // Insert session into database
    if let Err(err) = sqlx::query("INSERT INTO SESSION (user_id, valid_until) VALUES ($1, $2)")
        .bind(session.user_id)
        .bind(session.valid_until)
        .execute(&state.pool)
        .await
    {
        println!("Failed to insert session into database: {:?}", err);
        return (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred").into_response();
    }

    // Create cookie
    let cookie = format!(
        "flack_session={}; SameSite=Strict; Path=/; Max-Age={}",
        encrypted_session, SESSION_MAX_AGE_SECONDS
    );

    // Set cookie and redirect
    (
        StatusCode::SEE_OTHER,
        [
            (axum::http::header::SET_COOKIE, cookie),
            (axum::http::header::LOCATION, redirect.parse().unwrap()),
        ],
        "",
    )
        .into_response()
}

fn is_valid_email(s: &str) -> bool {
    let split: Vec<&str> = s.split('@').collect();
    if split.len() != 2 {
        return false;
    }
    if split[0].is_empty() || split[1].is_empty() {
        return false;
    }
    if !split[1].contains('.') {
        return false;
    }
    let domain_split: Vec<&str> = split[1].split('.').collect();
    if domain_split.len() < 2 {
        return false;
    }
    if domain_split.iter().any(|s| s.is_empty()) {
        return false;
    }
    true
}

fn is_valid_password(s: &str) -> bool {
    s.len() >= 6
}

// #[axum::debug_handler]
// pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
//     let mut response = Response::builder();
//     let referer = headers
//         .get(header::REFERER)
//         .and_then(|h| h.to_str().ok())
//         .unwrap_or("/dashboard")
//         .to_string()
//         .split('?')
//         .next()
//         .unwrap_or("/dashboard")
//         .to_string();

//     let _ = delete_all_session_data(&headers, &mut response, &state.pool).await;

//     response
//         .status(StatusCode::SEE_OTHER)
//         .header(header::LOCATION, referer)
//         .body("".to_string())
//         .expect("Failed to perform logout")
// }

// #[derive(Deserialize)]
// pub struct LoginQuery {
//     redirect: Option<String>,
// }

// #[derive(Deserialize)]
// pub struct LoginForm {
//     email: String,
//     password: String,
// }

// #[axum::debug_handler]
// pub async fn login(
//     State(state): State<AppState>,
//     Query(query): Query<LoginQuery>,
//     Form(form): Form<LoginForm>,
// ) -> impl IntoResponse {
//     let redirect = query.redirect.unwrap_or_else(|| "/dashboard".to_string());

//     // Check if email and password are provided
//     if form.email.is_empty() || form.password.is_empty() {
//         return Redirect::to(&format!(
//             "/login?error=missing_fields&redirect={}",
//             redirect
//         ))
//         .into_response();
//     }

//     // Attempt to get user by email
//     let user = match User::get_by_email(form.email, &state.pool).await {
//         Some(user) => user,
//         None => {
//             return Redirect::to(&format!("/login?error=invalid_email&redirect={}", redirect))
//                 .into_response()
//         }
//     };

//     // Verify password
//     if !verify(&form.password, &user.password_hash).unwrap_or(false) {
//         return Redirect::to(&format!(
//             "/login?error=invalid_password&redirect={}",
//             redirect
//         ))
//         .into_response();
//     }

//     // Check if account is disabled
//     if user.user_disabled {
//         return Redirect::to(&format!(
//             "/login?error=account_disabled&redirect={}",
//             redirect
//         ))
//         .into_response();
//     }

//     // Clear expired sessions (1 in 10 chance)
//     if rand::thread_rng().gen_range(0..10) == 0 {
//         let _ = sqlx::query("DELETE FROM SESSION WHERE valid_until < NOW()")
//             .execute(&state.pool)
//             .await;
//     }

//     // Create session
//     let session = FlackSession {
//         user_id: user.id,
//         valid_until: (chrono::Utc::now().naive_utc()
//             + chrono::Duration::seconds(SESSION_MAX_AGE_SECONDS.into()))
//         .trunc_subsecs(0),
//     };

//     // Encrypt session
//     let encrypted_session = match session.encrypt() {
//         Ok(encrypted) => encrypted,
//         Err(err) => {
//             println!("Failed to encrypt session: {:?}", err);
//             return (StatusCode::INTERNAL_SERVER_ERROR, "An error occured").into_response();
//         }
//     };

//     // Insert session into database
//     if let Err(err) = sqlx::query("INSERT INTO SESSION (user_id, valid_until) VALUES ($1, $2)")
//         .bind(session.user_id)
//         .bind(session.valid_until)
//         .execute(&state.pool)
//         .await
//     {
//         println!("Failed to insert session into database: {:?}", err);
//         return (StatusCode::INTERNAL_SERVER_ERROR, "An error occured").into_response();
//     }

//     // Create cookie
//     let cookie = format!(
//         "flack_session={}; SameSite=Strict; Path=/; Max-Age={}",
//         encrypted_session, SESSION_MAX_AGE_SECONDS
//     );

//     // Set cookie and redirect
//     (
//         StatusCode::SEE_OTHER,
//         [
//             (axum::http::header::SET_COOKIE, cookie),
//             (axum::http::header::LOCATION, redirect.parse().unwrap()),
//         ],
//         "",
//     )
//         .into_response()
// }
