use actix_web::{web, HttpResponse, Responder};
use crate::use_case::user::list_users::{ListUsers, ListUsersUseCase};
use crate::use_case::user::register_user::{RegisterUser, RegisterUserUseCase};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterForm {
    name: String,
    email: String,
}

pub async fn list_users_handler() -> impl Responder {
    let list_users = ListUsers;
    match list_users.execute().await {
        Ok(users) => {
            let mut html = String::from("<h1>User List</h1><ul>");
            for user in users {
                html += &format!("<li>{} ({})</li>", user.name, user.email);
            }
            html += "</ul><a href=\"/register\">Register New User</a>";
            HttpResponse::Ok().content_type("text/html").body(html)
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub async fn register_user_form() -> impl Responder {
    let html = r#"
        <h1>Register User</h1>
        <form action="/register" method="post">
            <label>Name:</label><br>
            <input type="text" name="name" required><br>
            <label>Email:</label><br>
            <input type="email" name="email" required><br><br>
            <input type="submit" value="Register">
        </form>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn register_user_handler(form: web::Form<RegisterForm>) -> impl Responder {
    let register = RegisterUser {
        name: form.name.clone(),
        email: form.email.clone(),
    };
    match register.execute().await {
        Ok(_) => HttpResponse::SeeOther().header("Location", "/users").finish(),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
