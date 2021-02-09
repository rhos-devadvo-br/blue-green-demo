use actix_web::{
    middleware::{Logger, Compress, DefaultHeaders},
    App, web, HttpServer
};
use tera::Tera;
use actix_files as fs;


mod handlers;
mod middleware;


static LAYOUT: &'static str = include_str!("../templates/layout.html");
static INDEX: &'static str = include_str!("../templates/index.html");
static ERROR: &'static str = include_str!("../templates/error.html");


fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(web::resource("/").route(web::get().to(handlers::index)))
            .wrap(middleware::error_handlers())
    );
}


// Actix-Web main run time loop
#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::builder()
        .format_timestamp(None)
        .init();

    // Load Tera templates
    let mut tera = Tera::default();
    match tera.add_raw_templates(
        vec![
            ("layout.html", LAYOUT),
            ("index.html", INDEX),
            ("error.html", ERROR)
        ]
    ) {
        Ok(result) => result,
        Err(exception) => {
            panic!("{:?}", exception)
        }
    };

    // Start HttpServer
    HttpServer::new(move || {

        // Start Actix-Web Application
        App::new()

            // Setup default middleware logger first
            .wrap(Logger::default())

            // Setup global maximum payload size
            .data(web::JsonConfig::default().limit(4097))
            .data(web::FormConfig::default().limit(4097))

            // Wrap the templates in App state
            .data(tera.clone())

            // Wrap default HTTP client
            .wrap(
                DefaultHeaders::new()
                    .header("X_DNS_PREFETCH_CONTROL", "off")
                    .header("X_XSS_PROTECTION", "1; mode=block")
                    .header("X_CONTENT_TYPE_OPTIONS", "nosniff")
            )

            // Setup compress request body middleware
            .wrap(Compress::default())

            // Register services and routes
            .configure(app_config)

    })
    .bind("0.0.0.0:5000")?
    .workers(4)
    .run()
    .await

}
