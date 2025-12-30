use crate::util::animalnumbers::to_animal_names;
use crate::util::hashids::to_hashids;
use crate::util::misc::remove_expired;
use crate::{AppState, ARGS};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct UrlCheckResponse {
    available: bool,
}

#[get("/api/check-url/{custom_url}")]
pub async fn check_url_availability(
    data: web::Data<AppState>,
    custom_url: web::Path<String>,
) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();
    let custom_url = custom_url.into_inner();

    // Remove expired pastas first
    remove_expired(&mut pastas);

    // Check if custom URL already exists
    for pasta in pastas.iter() {
        // Check against other custom URLs
        if let Some(ref existing_custom_url) = pasta.custom_url {
            if existing_custom_url == &custom_url {
                return HttpResponse::Ok().json(UrlCheckResponse { available: false });
            }
        }

        // Check against generated slugs
        let existing_slug = if ARGS.hash_ids {
            to_hashids(pasta.id)
        } else {
            to_animal_names(pasta.id)
        };

        if existing_slug == custom_url {
            return HttpResponse::Ok().json(UrlCheckResponse { available: false });
        }
    }

    // URL is available
    HttpResponse::Ok().json(UrlCheckResponse { available: true })
}
