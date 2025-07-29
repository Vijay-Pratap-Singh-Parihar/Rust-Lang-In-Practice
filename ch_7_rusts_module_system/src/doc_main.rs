// src/main.rs (Your binary that starts the server/app)

// Important: here we use the name of the package.
// If your Cargo.toml has `name = "my_backend_app"`, then use `use my_backend_app::user_service;`
// For simplicity, let's assume your package name is `restaurant` as in the example.
use ch_7_p2_restaurant::user_service; // Import the user_service module from your library crate

fn main() {
    println!("Starting backend application...");

    // Call functions from your library
    user_service::create_user(String::from("John Doe data"));
    let user = user_service::get_user(123);
    println!("Fetched user: {}", user);

    // In a real backend, you'd integrate with a web framework here
    // like Axum, Actix-Web, Rocket, etc.
    // For example (conceptual, not runnable without a web framework):
    // let app = web::App::new().service(web::resource("/users").to(handle_create_user));
    // HttpServer::new(|| app).bind("127.0.0.1:8080").run().unwrap();
}
