use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{auth, services};

pub async fn app() -> Router {
    Router::new()
        .route("/signin", post(auth::sign_in))
        .route(
            "/protected/",
            get(services::hello).layer(middleware::from_fn(auth::authorize)),
        )
        // .with_state()
}
