use crate::views::page::page;
use actix_web::{get, web};
use maud::{html, Markup};

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/user").service(login).service(register));
}

pub fn protected_content(username: &str) -> Markup {
    page(
        "Protected area!",
        html! {
            div {
                h2 { "Welcome, " (username) "!" }
                p { "This is protected content." }
                form method="post" action="/api/user/logout" {
                    input type="submit" value="Logout" {}
                }
            }
        },
    )
}

#[get("/login")]
async fn login() -> Markup {
    page(
        "Login",
        html! {
            form method="post" action="/api/user/login" {
            div {
                label for="username" { "Username: " }
                input type="text" name="username" id="username" {}
            }
            div {
                label for="password" { "Password: " }
                input type="password" name="password" id="password" {}
            }
            div {
                input type="submit" value="Login" {}
            }
        }
        },
    )
}

#[get("/register")]
async fn register() -> Markup {
    page(
        "Register",
        html! {
            form method="post" action="/api/user/register" {
                div {
                    label for="username" { "Username: " }
                    input type="text" name="username" id="username" {}
                }
                div {
                    label for="password" { "Password: " }
                    input type="password" name="password" id="password" {}
                }
                div {
                    input type="submit" value="Register" {}
                }
            }
        },
    )
}
