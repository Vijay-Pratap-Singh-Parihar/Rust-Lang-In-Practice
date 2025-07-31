use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

// This is a handler function.
// The `#[get("/")]` attribute is a macro that creates a route for a GET request to the root path ("/").
#[get("/")]
async fn hello() -> impl Responder {
    // This function returns something that implements the `Responder` trait.
    // In this case, it's an `HttpResponse` with a "200 OK" status and a body of "Hello, World!".
    HttpResponse::Ok().body("Hello, World!")
}

// Another handler for a different route.
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    // This handler uses an extractor (`web::Path`) to get the `name` from the URL.
    // For example, if you go to `/hello/Alice`, the `name` will be "Alice".
    format!("Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // `HttpServer::new` creates a new HTTP server.
    // The closure passed to `new` is an "application factory".
    // Actix Web will run this closure for each worker thread.
    HttpServer::new(|| {
        // `App::new()` creates a new application.
        // We then register our handlers with the `service` method.
        App::new()
            .service(hello)
            .service(greet)
    })
    // `bind` tells the server to listen on this IP address and port.
    .bind(("127.0.0.1", 8080))?
    // `run` starts the server.
    .run()
    .await
}
