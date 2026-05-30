use std::fs::File;
use std::path::PathBuf;

use comrak;

use crate::args::ARGS;
use crate::util::auth;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::remove_expired;
use crate::util::{animalnumbers::to_u64, misc::decrypt_file};
use crate::AppState;
use actix_multipart::Multipart;
use actix_web::http::header;
use actix_web::{get, post, web, Error, HttpResponse};

use std::collections::HashMap;

const HTML_BACK_BUTTON: &str = r#"<style>@media print{#_mbback{display:none!important}}#_mbback{position:fixed;bottom:28px;right:28px;width:52px;height:52px;background:rgba(26,26,46,.9);backdrop-filter:blur(6px);-webkit-backdrop-filter:blur(6px);border-radius:50%;display:flex;align-items:center;justify-content:center;text-decoration:none;box-shadow:0 3px 14px rgba(0,0,0,.35);z-index:2147483647;transition:background .18s,transform .12s}#_mbback:hover{background:rgba(10,10,30,.97);transform:scale(1.09)}#_mbback svg{width:26px;height:26px;fill:#fff}</style><a id="_mbback" href="/" title="Back to MicroBin" aria-label="Back to MicroBin"><svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/></svg></a>"#;

const MARKDOWN_SHELL: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>
:root{color-scheme:light dark}
body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,sans-serif;line-height:1.7;color:#1a1a2e;background:#fff;max-width:780px;margin:2.5rem auto;padding:0 1.5rem 5rem}
@media(prefers-color-scheme:dark){body{background:#1a1a2e;color:#e8e8f0}a{color:#58a6ff}h1,h2{border-color:#30363d}code,pre{background:#161b22}th,td{border-color:#30363d}th{background:#0d1117}blockquote{border-color:#444;color:#8b949e}}
h1,h2,h3,h4,h5,h6{line-height:1.25;margin-top:2rem;margin-bottom:.5rem}
h1{font-size:2em;border-bottom:2px solid #eee;padding-bottom:.3em}
h2{font-size:1.5em;border-bottom:1px solid #eee;padding-bottom:.2em}
a{color:#0969da;text-decoration:none}a:hover{text-decoration:underline}
code{background:#f6f8fa;border-radius:4px;padding:.15em .4em;font-size:.875em;font-family:'SFMono-Regular',Consolas,'Liberation Mono',Menlo,monospace}
pre{background:#f6f8fa;border-radius:8px;padding:1.2rem;overflow-x:auto;line-height:1.5}
pre code{background:none;padding:0;font-size:.9em}
blockquote{border-left:4px solid #d0d7de;margin:0;padding:.5em 1em;color:#57606a}
table{border-collapse:collapse;width:100%;margin:1rem 0}
th,td{border:1px solid #d0d7de;padding:.5em 1em;text-align:left}
th{background:#f6f8fa;font-weight:600}
img{max-width:100%;border-radius:4px}
hr{border:none;border-top:2px solid #eee;margin:2rem 0}
ul,ol{padding-left:1.5em}
input[type=checkbox]{margin-right:.4em;vertical-align:middle}
p{margin:.75em 0}
</style>
</head>
<body>
MARKDOWN_CONTENT_PLACEHOLDER
</body>
</html>"#;

fn inject_back_button(html: &str) -> String {
    let lower = html.to_lowercase();
    if let Some(pos) = lower.rfind("</body>") {
        format!("{}{}{}", &html[..pos], HTML_BACK_BUTTON, &html[pos..])
    } else {
        format!("{}{}", html, HTML_BACK_BUTTON)
    }
}

fn render_markdown(md: &str) -> String {
    let mut opts = comrak::Options::default();
    opts.extension.table = true;
    opts.extension.strikethrough = true;
    opts.extension.tasklist = true;
    opts.extension.autolink = true;
    let body = comrak::markdown_to_html(md, &opts);
    MARKDOWN_SHELL.replace("MARKDOWN_CONTENT_PLACEHOLDER", &inject_back_button(&body))
}

#[post("/secure_file/{id}")]
pub async fn post_secure_file(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: Multipart,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id.into_inner()).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            index = i;
            found = true;
            break;
        }
    }

    let password = auth::password_from_multipart(payload).await?;

    if found {
        let mut target_filename = None;
        if let Some(fname) = query.get("fname") {
             // sanitize fname? It should match one of the attachments or file.
             // Security check: ensure fname is in the list of files for this pasta
             if let Some(file) = &pastas[index].file {
                 if file.name() == *fname {
                     target_filename = Some(file.name());
                 }
             }
             if target_filename.is_none() {
                 if let Some(attachments) = &pastas[index].attachments {
                     for att in attachments {
                         if att.name() == *fname {
                             target_filename = Some(att.name());
                             break;
                         }
                     }
                 }
             }
        }

        // Fallback to primary file if no fname or not found (and fname wasn't provided)
        if target_filename.is_none() && query.get("fname").is_none() {
             if let Some(file) = &pastas[index].file {
                 target_filename = Some(file.name());
             }
        }

        if let Some(filename) = target_filename {
            // Try new naming scheme {filename}.enc first, then fallback to data.enc (legacy/primary)
            let mut enc_path = format!(
                "{}/attachments/{}/{}.enc",
                ARGS.data_dir,
                pastas[index].id_as_animals(),
                filename
            );
            
            if !std::path::Path::new(&enc_path).exists() {
                 // Fallback for legacy primary file
                 enc_path = format!(
                    "{}/attachments/{}/data.enc",
                    ARGS.data_dir,
                    pastas[index].id_as_animals()
                );
            }

            if let Ok(file) = File::open(&enc_path) {
                // Not compatible with NamedFile from actix_files (it needs a File
                // to work therefore secure files do not support streaming
                let decrypted_data: Vec<u8> = decrypt_file(&password, &file)?;

                // Set the content type based on the file extension
                let content_type = mime_guess::from_path(&filename)
                    .first_or_octet_stream()
                    .to_string();

                // Create a response with the decrypted data
                let response = HttpResponse::Ok()
                    .content_type(content_type)
                    .append_header((
                        "Content-Disposition",
                        format!("attachment; filename=\"{}\"", filename),
                    ))
                    // TODO: make streaming <21-10-24, dvdsk>
                    .body(decrypted_data);
                return Ok(response);
            }
        }
    }
    Ok(HttpResponse::NotFound().finish())
}

#[get("/file/{id}")]
pub async fn get_file(
    request: actix_web::HttpRequest,
    id: web::Path<String>,
    data: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    // get access to the pasta collection
    let mut pastas = data.pastas.lock().unwrap();

    let id_intern = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id.into_inner()).unwrap_or(0)
    };

    // remove expired pastas (including this one if needed)
    remove_expired(&mut pastas);

    // find the index of the pasta in the collection based on u64 id
    let mut index: usize = 0;
    let mut found: bool = false;
    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id_intern {
            index = i;
            found = true;
            break;
        }
    }

    if found {
        // Determine which file to serve
        let mut target_file = None;
        if let Some(fname) = query.get("fname") {
            if let Some(file) = &pastas[index].file {
                if file.name() == *fname {
                    target_file = Some(file);
                }
            }
            if target_file.is_none() {
                if let Some(attachments) = &pastas[index].attachments {
                    for att in attachments {
                        if att.name() == *fname {
                            target_file = Some(att);
                            break;
                        }
                    }
                }
            }
        } else {
            // Default to primary file
             if let Some(file) = &pastas[index].file {
                target_file = Some(file);
            }
        }

        if let Some(pasta_file) = target_file {
            if pastas[index].encrypt_server {
                return Ok(HttpResponse::Found()
                    .append_header((
                        "Location",
                        format!("{}/auth_file/{}", ARGS.public_path_as_str(), pastas[index].id_as_animals()),
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

            // HTML and Markdown: read and serve inline with CSP sandbox
            if pasta_file.is_html() || pasta_file.is_markdown() {
                let text = std::fs::read_to_string(&file_path)
                    .map_err(actix_web::error::ErrorInternalServerError)?;
                let body = if pasta_file.is_markdown() {
                    render_markdown(&text)
                } else {
                    inject_back_button(&text)
                };
                return Ok(HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .insert_header(("Content-Security-Policy", "sandbox allow-scripts; default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'"))
                    .insert_header(("X-Content-Type-Options", "nosniff"))
                    .body(body));
            }

            let file_reponse = actix_files::NamedFile::open(file_path)?;

            let preview_requested = query.get("preview").map(|s| s == "true").unwrap_or(false);
            let mime = mime_guess::from_path(pasta_file.name()).first_or_octet_stream();
            let mime_str = mime.essence_str();
            let safe_for_inline = mime_str.starts_with("image/")
                || mime_str.starts_with("video/")
                || mime_str.starts_with("audio/")
                || mime_str == "application/pdf";

            let disposition = if preview_requested && safe_for_inline {
                header::DispositionType::Inline
            } else {
                header::DispositionType::Attachment
            };

            let file_reponse = file_reponse.set_content_disposition(header::ContentDisposition {
                disposition,
                parameters: vec![header::DispositionParam::Filename(
                    pasta_file.name().to_string(),
                )],
            });

            let mut response = file_reponse.into_response(&request);
            let headers = response.headers_mut();
            headers.insert(
                header::HeaderName::from_static("x-content-type-options"),
                header::HeaderValue::from_static("nosniff"),
            );
            if preview_requested && safe_for_inline {
                headers.insert(
                    header::HeaderName::from_static("content-security-policy"),
                    header::HeaderValue::from_static("sandbox; default-src 'none'"),
                );
            }
            return Ok(response);
        }
    }

    Ok(HttpResponse::NotFound().finish())
}
