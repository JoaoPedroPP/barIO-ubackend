use actix_cors::Cors;
use actix_web::{
    App,
    HttpServer,
    web,
    http
};

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            // .allow_any_origin()
            // .send_wildcard()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(
                web::scope("/api")
                    .route("/upload", web::post().to(services::upload_cos))
            )
    })
    .bind("0.0.0.0:8080")?
    .workers(5)
    .run()
    .await
}