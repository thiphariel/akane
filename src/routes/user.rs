use crate::models::user::NewUser;
use crate::repository::database::Database;
use crate::repository::user::Users;
use crate::views;
use actix_identity::Identity;
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;

#[derive(Deserialize)]
struct Request {
    username: String,
    password: String,
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/user").service(login).service(register));
}

#[post("/login")]
async fn login(
    request: HttpRequest,
    data: web::Form<Request>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let user = db.get_user(&data.username);

    if let Some(user) = user {
        if verify(&data.password, &user.password_hash).unwrap() {
            Identity::login(&request.extensions(), String::from(&user.username))?;

            // Return the protected content directly
            let content = views::user::protected_content(&user.username);
            return Ok(HttpResponse::Ok().body(content.into_string()));
        }
    }

    Ok(HttpResponse::Unauthorized().body("Invalid username or password"))
}

#[post("/register")]
async fn register(request: web::Form<Request>, db: web::Data<Database>) -> Result<HttpResponse> {
    let password_hash = hash(&request.password, DEFAULT_COST).unwrap();
    let user = NewUser {
        username: request.username.clone(),
        password_hash,
    };

    db.create_user(user).unwrap();

    // Redirect to home page on successful registration
    Ok(HttpResponse::Found()
        .append_header(("LOCATION", "/projects"))
        .finish())
}
