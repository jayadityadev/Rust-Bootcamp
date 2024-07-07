use actix_cors::Cors;
use actix_web::{web::Data, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct MyDocument {
    name: String,
    email: String,
    employment_type: String,
    company: String,
    team: String,
    profile_image_url: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./index.html"))
}

async fn contact() -> impl Responder {
    "Contact us at contact@example.com"
}

async fn get_document(client: web::Data<Arc<Client>>, name: web::Path<String>) -> impl Responder {
    let database = client.database("test_db");
    let collection = database.collection::<MyDocument>("testCollection");
    let name_str = name.into_inner().replace("+", "%20");
    // println!("Name: {}", name_str);

    match collection.find_one(doc! { "name": &name_str }, None).await {
        Ok(Some(document)) => HttpResponse::Ok().json(document),
        Ok(None) => HttpResponse::NotFound().body("No document found"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn query_redirect(req: HttpRequest) -> impl Responder {
    let name = req.query_string().to_string();
    // let truncated_name = &name[5..];
    let truncated_name = &name[5..].replace("+", "%20");
    println!("Name: {}", truncated_name);
    HttpResponse::Found()
        .header("Location", format!("/app/{}", truncated_name))
        .finish()
}

#[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Initialize MongoDB client
//     let client_options = ClientOptions::parse("mongodb://localhost:27017")
//         .await
//         .unwrap();
//     let client = Client::with_options(client_options).unwrap();
//     let client_data = web::Data::new(Arc::new(client));

//     HttpServer::new(move || {
//         let cors = Cors::default()
//             .allowed_origin("http://127.0.0.1:3000")
//             .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
//             .allowed_methods(vec!["GET", "POST"])
//             .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
//             .allowed_header(http::header::CONTENT_TYPE)
//             .max_age(3600);
//         App::new().app_data(client_data.clone()).service(
//             web::scope("/app")
//                 .route("", web::get().to(index))
//                 .route("/contact", web::get().to(contact))
//                 .route("/query", web::get().to(query_redirect))
//                 .route("/{name}", web::get().to(get_document)),
//         )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Initialize MongoDB Client
    let client_options = ClientOptions::parse("mongodb://localhost:27017/").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let client = Arc::new(client); // Wrap the client with Arc for thread-safe sharing

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(Data::new(client.clone())) // Pass the wrapped client to your app
            .service(
                web::scope("/app")
                    .route("", web::get().to(index))
                    .route("/contact", web::get().to(contact))
                    .route("/query", web::get().to(query_redirect))
                    .route("/{name}", web::get().to(get_document)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}