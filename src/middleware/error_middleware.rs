use log::{info, error, warn};
use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{
    ErrorHandlerResponse, ErrorHandlers
};
use actix_web::{
    web::Data, Result
};
use tera::Tera;


pub fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new()
        .handler(StatusCode::NOT_FOUND, not_found)
        .handler(StatusCode::BAD_REQUEST, bad_request)
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page Not Found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Error handler for a 400 Bad request error.
fn bad_request<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Bad Request");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Error handler for a 500 Internal server error.
fn internal_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Ops! Something is wrong, please try again later.");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        warn!("Failed renderization of error.html template, using plain text fallback...");
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let tera = request.app_data::<Data<Tera>>().map(|t| t.get_ref());

    match tera {
        Some(tera) => {

            let mut context = tera::Context::new();

            context.insert("lang", "en");
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());

            let body = tera.render("error.html", &context);

            match body {
                Ok(body) => {
                    info!("Error template rendered correctly.");
                    Response::build(res.status())
                        .content_type("text/html")
                        .body(body)
                },
                Err(exception) => {
                    error!("Tera templating exception: {}", &exception);
                    fallback(error)
                }
            }
        }
        None => {
            error!("Empty Tera template");
            fallback(error)
        }
    }
}
