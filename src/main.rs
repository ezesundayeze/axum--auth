use axum;
use tokio::net::TcpListener;

mod auth;
mod routes;
mod services;

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Unable to conne to connect to the server");

    println!("Listening on {}", listener.local_addr().unwrap() );

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
