use std::fs::File;
use std::path::PathBuf;

use crate::args::ARGS;
use crate::util::auth;
use crate::util::misc::{decrypt_file, find_pasta_by_slug, remove_expired};
use crate::AppState;
use actix_multipart::Multipart;
use actix_web::http::header;
use actix_web::{get, post, web, Error, HttpResponse};

const HTML_BACK_BUTTON: &str = r#"<style>@media print{#_mbback{display:none!important}}#_mbback{position:fixed;bottom:28px;right:28px;width:52px;height:52px;background:rgba(26,26,46,.9);backdrop-filter:blur(6px);-webkit-backdrop-filter:blur(6px);border-radius:50%;display:flex;align-items:center;justify-content:center;text-decoration:none;box-shadow:0 3px 14px rgba(0,0,0,.35);z-index:2147483647;transition:background .18s,transform .12s}#_mbback:hover{background:rgba(10,10,30,.97);transform:scale(1.09)}#_mbback svg{width:26px;height:26px;fill:#fff}</style><a id="_mbback" href="/" title="Back to MicroBin" aria-label="Back to MicroBin"><svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/></svg></a>"#;

fn inject_back_button(html: &str) -> String {
    let lower = html.to_lowercase();
    if let Some(pos) = lower.rfind("</body>") {
        format!("{}{}{}", &html[..pos], HTML_BACK_BUTTON, &html[pos..])
    } else {
        format!("{}{}", html, HTML_BACK_BUTTON)
    }
}

#[post("/secure_file/{id}")]
pub async fn post_secure_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let slug = id.into_inner();

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    let password = auth::password_from_multipart(payload).await?;

    // find the pasta by slug (custom URL or generated ID)
    if let Some(index) = find_pasta_by_slug(&pastas, &slug) {
        if let Some(ref pasta_file) = pastas[index].file {
            let file = File::open(format!(
                "{}/attachments/{}/data.enc",
                ARGS.data_dir,
                pastas[index].id_as_animals()
            ))?;

            // Not compatible with NamedFile from actix_files (it needs a File
            // to work therefore secure files do not support streaming
            let decrypted_data: Vec<u8> = decrypt_file(&password, &file)?;

            // HTML files: serve inline with non-printable navigation button injected.
            // CSP sandbox prevents script execution on the app origin.
            if pasta_file.is_html() {
                match String::from_utf8(decrypted_data) {
                    Ok(html_str) => {
                        return Ok(HttpResponse::Ok()
                            .content_type("text/html; charset=utf-8")
                            .insert_header(("Content-Security-Policy", "sandbox; default-src 'none'; style-src 'unsafe-inline'"))
                            .insert_header(("X-Content-Type-Options", "nosniff"))
                            .body(inject_back_button(&html_str)));
                    }
                    Err(e) => {
                        // Not valid UTF-8 — fall through to attachment download
                        let decrypted_data = e.into_bytes();
                        return Ok(HttpResponse::Ok()
                            .content_type("application/octet-stream")
                            .append_header((
                                "Content-Disposition",
                                format!("attachment; filename=\"{}\"", pasta_file.name()),
                            ))
                            .body(decrypted_data));
                    }
                }
            }

            // Set the content type based on the file extension
            let content_type = mime_guess::from_path(&pasta_file.name)
                .first_or_octet_stream()
                .to_string();

            // Create a response with the decrypted data
            let response = HttpResponse::Ok()
                .content_type(content_type)
                .append_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", pasta_file.name()),
                ))
                // TODO: make streaming <21-10-24, dvdsk>
                .body(decrypted_data);
            return Ok(response);
        }
    }
    Ok(HttpResponse::NotFound().finish())
}

#[get("/file/{id}")]
pub async fn get_file(
    request: actix_web::HttpRequest,
    id: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let slug = id.into_inner();

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the pasta by slug (custom URL or generated ID)
    if let Some(index) = find_pasta_by_slug(&pastas, &slug) {
        if let Some(ref pasta_file) = pastas[index].file {
            if pastas[index].encrypt_server {
                return Ok(HttpResponse::Found()
                    .append_header((
                        "Location",
                        format!("/auth_file/{}", slug),
                    ))
                    .finish());
            }

            // Construct the path to the file
            let file_path = format!(
                "{}/attachments/{}/{}",
                ARGS.data_dir,
                pastas[index].id_as_animals(),
                pasta_file.name()
            );
            let file_path = PathBuf::from(file_path);

            // HTML files: serve inline with non-printable navigation button injected.
            // CSP sandbox prevents script execution on the app origin.
            if pasta_file.is_html() {
                let html_content = std::fs::read_to_string(&file_path)?;
                return Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .insert_header(("Content-Security-Policy", "sandbox; default-src 'none'; style-src 'unsafe-inline'"))
                    .insert_header(("X-Content-Type-Options", "nosniff"))
                    .body(inject_back_button(&html_content)));
            }

            // All other files: stream as download attachment
            let file_reponse = actix_files::NamedFile::open(file_path)?;
            let file_reponse = file_reponse.set_content_disposition(header::ContentDisposition {
                disposition: header::DispositionType::Attachment,
                parameters: vec![header::DispositionParam::Filename(
                    pasta_file.name().to_string(),
                )],
            });
            // This takes care of streaming/seeking using the Range
            // header in the request.
            return Ok(file_reponse.into_response(&request));
        }
    }

    Ok(HttpResponse::NotFound().finish())
}
