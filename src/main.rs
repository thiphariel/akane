use crate::repository::database::Database;
use actix_files::{Files, NamedFile};
use actix_identity::IdentityMiddleware;
use actix_session::config::{PersistentSession, TtlExtensionPolicy};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use env_logger::Env;
use lightningcss::bundler::{Bundler, FileProvider};
use lightningcss::stylesheet::{ParserOptions, PrinterOptions};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod models;
mod repository;
mod routes;
mod views;

type DieselDatabase = diesel::sqlite::Sqlite;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
const SECS_IN_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize)]
pub struct Response {
    status: String,
    message: String,
}

fn run_migrations(connection: &mut impl MigrationHarness<DieselDatabase>) {
    if let Err(err) = connection.run_pending_migrations(MIGRATIONS) {
        eprintln!("Failed to run migrations: {:?}", err);
    }
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    }))
}

async fn favicon() -> Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(Response {
        status: "ok".to_string(),
        message: "Server is running".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new();
    run_migrations(&mut database.pool.get().unwrap());

    let data = web::Data::new(database);
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Secure key for signing/encrypting the session cookies
    let key = Key::generate();

    let fs = FileProvider::new();
    let mut bundler = Bundler::new(&fs, None, ParserOptions::default());
    let bundle = bundler.bundle(Path::new("src/styles/app.css")).unwrap();
    let mut css = File::create("static/bundle.css").expect("creation failed");
    css.write(
        bundle
            .to_css(PrinterOptions::default())
            .unwrap()
            .code
            .as_bytes(),
    )
    .expect("write failed");

    HttpServer::new(move || {
        App::new()
            .service(
                Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .route("/favicon.ico", web::get().to(favicon))
            .app_data(data.clone())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(), // Cookie session store
                    key.clone(),                   // Signing key
                )
                .cookie_secure(true) // Ensures the session cookie is only sent over HTTPS
                .cookie_name(String::from("auth-session")) // Name of the session cookie
                .cookie_same_site(actix_web::cookie::SameSite::Strict) // SameSite policy
                .cookie_http_only(true) // Prevents JavaScript access to the cookie
                .cookie_path(String::from("/")) // Cookie path
                .session_lifecycle(
                    PersistentSession::default()
                        .session_ttl(Duration::seconds(SECS_IN_WEEK))
                        .session_ttl_extension_policy(TtlExtensionPolicy::OnEveryRequest),
                )
                .build(),
            )
            .wrap(IdentityMiddleware::default())
            .configure(routes::user::init_routes)
            .configure(routes::projects::init_routes)
            .configure(views::user::init_routes)
            .configure(views::projects::init_routes)
            .service(health)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
