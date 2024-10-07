use actix_web::{App, HttpRequest, HttpServer, Responder, web};

// Define an asynchronous function `greet` that takes an `HttpRequest` as an argument
// and returns a type that implements the `Responder` trait.
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World!");
    format!("Hello {}!", &name)
}

// The main function is marked with the `tokio::main` attribute, which indicates that
// this function is the entry point of the application and will run on the Tokio runtime.
// Tokio is an asynchronous runtime for Rust, allowing for concurrent execution of tasks.
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Create a new HTTP server instance
    HttpServer::new(|| {
        // Create a new Actix web application
        App::new()
            // and maps them to the `greet` function.
            .route("/", web::get().to(greet))
            // Define a route for any path that matches "/{name}" where `{name}` is a
            // path parameter. This also responds to GET requests and maps them to the `greet` function.
            .route("/{name}", web::get().to(greet))
    })
        // Bind the server to the local address 127.0.0.1 on port 8000.
        // The `?` operator is used to propagate any errors that may occur during binding.
        .bind("127.0.0.1:8000")?
        // Start the server and await its completion.
        .run()
        .await
}
