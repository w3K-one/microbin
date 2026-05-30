use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::pasta::Pasta;
use crate::util::misc::remove_expired;
use crate::AppState;

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    pastas: &'a Vec<Pasta>,
    args: &'a Args,
    is_admin: bool,
}

#[get("/list")]
pub async fn list(data: web::Data<AppState>, session: Session) -> HttpResponse {
    if ARGS.no_listing {
        return HttpResponse::Found()
            .append_header(("Location", format!("{}/", ARGS.public_path_as_str())))
            .finish();
    }

    let is_admin = session.get::<bool>("admin").unwrap_or(None) == Some(true);

    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    // sort pastas in reverse-chronological order of creation time
    pastas.sort_by(|a, b| b.created.cmp(&a.created));

    HttpResponse::Ok().content_type("text/html").body(
        ListTemplate {
            pastas: &pastas,
            args: &ARGS,
            is_admin,
        }
        .render()
        .unwrap(),
    )
}
