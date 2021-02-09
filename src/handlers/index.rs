use std::collections::HashMap;
use log::{info, error};
use actix_web::{
    error::{ErrorInternalServerError as InternalError},
    web::{Data, Query},
    Error, HttpResponse, HttpRequest
};
use chrono::Utc;


pub async fn index(
    req: HttpRequest,
    tmpl: Data<tera::Tera>,
    query: Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    
    let t = Utc::now();

    // Check for valid `lang` query params:
    let lang = match query.get("lang") {
        Some(language) => {
            match &language[..] {
                "pt" => "pt".to_string(),
                "es" => "es".to_string(),
                "en" => "en".to_string(),
                _ => {
                    // Invalid or unsupported language specified.
                    // Fetch preferred language from HTTP headers:
                    let languages_str = 
                        match req.headers().get("Accept-Language") {
                            Some(languages_header) => {
                                match languages_header.to_str().ok() {
                                    Some(languages_str) => languages_str.to_string(),
                                    None => {
                                        // Defaults to english if Accept-Language
                                        // header is empty.
                                        "en".to_string()
                                    }
                                }
                            },
                            None => {
                                // Defaults to english if Accept-Language 
                                // header is inexistent.
                                "en".to_string()
                            }
                        };
                    // Intersect preferred with supported languages
                    let common_languages = accept_language::intersection(
                        &languages_str, vec!["pt", "es"]
                    );
                    // Set lang as the preferred language
                    match common_languages.len() {
                        0 => "en".to_string(),
                        _ => common_languages[0].to_owned()
                    }
                }
            }
        },
        None => {
            // No `lang` specified at the request url params.
            // Fetch preferred language from HTTP headers:
            let languages_str = 
                match req.headers().get("Accept-Language") {
                    Some(languages_header) => {
                        match languages_header.to_str().ok() {
                            Some(languages_str) => languages_str.to_string(),
                            None => {
                                // Defaults to english if Accept-Language
                                // header is empty.
                                "en".to_string()
                            }
                        }
                    },
                    None => {
                        // Defaults to english if Accept-Language 
                        // header is inexistent.
                        "en".to_string()
                    }
                };
            // Intersect preferred with supported languages
            let common_languages 
                = accept_language::intersection(
                    &languages_str, vec!["pt", "es"]
                );
            // Set lang as the preferred language
            match common_languages.len() {
                0 => "en".to_string(),
                _ => common_languages[0].to_owned()
            }
        }
    };
    
    // Set context for rendering the HTML template
    // based on the selected language
    let mut ctx = tera::Context::new();
    ctx.insert("lang", &lang);

    /*match &lang[..] {
        "pt" => {
            ctx.insert("title", "Maratona Behind the Code IBM 2020");
        },
        "es" => {
            ctx.insert("title", "Maratón Behind the Code IBM 2020");
        },
        _ => {
            ctx.insert("title", "IBM Behind the Code Marathon 2020.");
        }
    };*/

    let html_body = 
        match tmpl.render("index.html", &ctx) {
            Ok(rendered_body) => rendered_body,
            Err(_) => return {
                error!("Templating error at index.rs line 111.");
                Err(InternalError("Internal Server Error"))
            }
        };

    let dt = (chrono::Utc::now() - t).num_milliseconds();
    info!("Index HTML page dispatched in {}ms", &dt);

    return Ok(
        HttpResponse::Ok()
            .content_type("text/html")
            .body(html_body)
    )

}
