use actix_web::web::Query;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Deserialize)]
struct AddParams {
    numbers: String,
}

#[derive(Serialize)]
struct AddResponse {
    numbers: Vec<f64>,
    sum: f64,
    message: String,
}
struct AppState {
    app_name: String,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!")
}

#[get("/cal-sum")]
async fn echo(Query(params): Query<AddParams>) -> impl Responder {
    // Split the 'numbers' string by commas, parse each piece into a f64, and collect into a Vec<f64>
    let numbers: Vec<f64> = params
        .numbers
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();

    // Sum all the numbers
    let sum: f64 = numbers.iter().sum();

    // Build the response structure
    let response = AddResponse {
        numbers: numbers.clone(),
        sum,
        message: format!("The sum of {:?} is {}", numbers, sum),
    };

    // A structured JSON should be returned
    HttpResponse::Ok().json(response)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Robert App"),
            }))
            .service(hello)
            .service(echo)
            .service(
                web::scope("/app1")
                    // .guard(guard::Host("users.rust-lang.org"))
                    .route("/hey", web::get().to(manual_hello)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
