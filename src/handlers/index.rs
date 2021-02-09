use log::{info, error};
use actix_web::{
    error::{ErrorInternalServerError as InternalError},
    web::Data,
    Error, HttpResponse
};
use chrono::Utc;


pub async fn index(
    tmpl: Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    
    let t = Utc::now();
    
    let color: String = 
        std::env::var("COLOR")
            .expect("COLOR environment variable not set!");
    
    // Set context for rendering the HTML template
    let mut ctx = tera::Context::new();
    ctx.insert("color", &color);

    let html_body = 
        match tmpl.render("index.html", &ctx) {
            Ok(rendered_body) => rendered_body,
            Err(_) => return {
                error!("Templating error at index.rs.");
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
