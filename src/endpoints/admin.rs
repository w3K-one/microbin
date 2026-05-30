use crate::args::{Args, ARGS};
use crate::pasta::Pasta;
use crate::util::misc::{prune_all_expired, remove_expired};
use crate::util::version::{Version, CURRENT_VERSION};
use crate::AppState;
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse};
use askama::Template;
use futures::TryStreamExt;

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate<'a> {
    pastas: &'a Vec<Pasta>,
    args: &'a Args,
    status: &'a String,
    version_string: &'a String,
    message: &'a String,
    update: &'a Option<Version>,
}

fn is_admin(session: &Session) -> bool {
    session.get::<bool>("admin").unwrap_or(None) == Some(true)
}

#[get("/@")]
pub async fn get_admin(
    session: Session,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if !is_admin(&session) {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/@/auth"))
            .finish());
    }

    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    pastas.sort_by(|a, b| b.created.cmp(&a.created));

    let status = if *ARGS.auth_admin_username == "admin" && *ARGS.auth_admin_password == "m1cr0b1n" {
        "WARNING"
    } else {
        "OK"
    };

    let message = if *ARGS.auth_admin_username == "admin" && *ARGS.auth_admin_password == "m1cr0b1n" {
        "Warning: You are using the default admin credentials. This is a security risk."
    } else if ARGS.public_path.is_none() {
        "Warning: No public URL set. QR code and URL copying are disabled."
    } else {
        ""
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(
        AdminTemplate {
            pastas: &pastas,
            args: &ARGS,
            status: &String::from(status),
            version_string: &format!("{}", CURRENT_VERSION.long_title),
            message: &String::from(message),
            update: &None,
        }
        .render()
        .unwrap(),
    ))
}

#[post("/@")]
pub async fn post_admin(
    mut payload: Multipart,
    session: Session,
) -> Result<HttpResponse, Error> {
    let mut username = String::new();
    let mut password = String::new();

    while let Some(mut field) = payload.try_next().await? {
        if field.name() == Some("username") {
            while let Some(chunk) = field.try_next().await? {
                username.push_str(std::str::from_utf8(&chunk).unwrap_or(""));
            }
        } else if field.name() == Some("password") {
            while let Some(chunk) = field.try_next().await? {
                password.push_str(std::str::from_utf8(&chunk).unwrap_or(""));
            }
        }
    }

    if username != *ARGS.auth_admin_username || password != *ARGS.auth_admin_password {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/@/auth/incorrect"))
            .finish());
    }

    let _ = session.insert("admin", true);

    Ok(HttpResponse::Found()
        .append_header(("Location", "/@"))
        .finish())
}

#[post("/@/prune")]
pub async fn post_admin_prune(
    session: Session,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    if !is_admin(&session) {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/@/auth"))
            .finish());
    }

    let mut pastas = data.pastas.lock().unwrap();
    prune_all_expired(&mut pastas);

    Ok(HttpResponse::Found()
        .append_header(("Location", "/@"))
        .finish())
}

#[post("/@/logout")]
pub async fn post_admin_logout(session: Session) -> Result<HttpResponse, Error> {
    session.purge();
    Ok(HttpResponse::Found()
        .append_header(("Location", "/@/auth"))
        .finish())
}
