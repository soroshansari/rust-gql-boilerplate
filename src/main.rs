use std::io;

use actix_cors::Cors;
use actix_web::{middleware, web, web::Data, App, HttpServer};
use dotenv::dotenv;

use ::lib::config::get as config_get;
use ::lib::graphql::schema::create_schema;
use ::lib::handlers::graphql::{graphql, playground};
use ::lib::models::key::Key;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load .env variables
    dotenv().ok();

    let env: String = config_get("env");
    let host: String = config_get("host");
    let port: String = config_get("port");
    let key = config_get("api_key");
    let key = Key::new(key);

    // configure logging
    std::env::set_var(
        "RUST_LOG",
        if env == "development" {
            "actix_web=debug"
        } else {
            "actix_web=info"
        },
    );
    env_logger::init();

    // create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    println!(
        "Starting GraphQL server at http://{}:{}/playground",
        host, port
    );

    // start http server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(key.clone()))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3050")
                    .allowed_methods(vec!["GET", "POST"]),
            ) // allow all cross origin requests
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
