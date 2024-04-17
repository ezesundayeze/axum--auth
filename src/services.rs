use axum::{Extension, Json, response::IntoResponse};
use serde::{Serialize, Deserialize};

use crate::auth::CurrentUser;

#[derive(Serialize, Deserialize)]
struct UserResponse {
    email: String, 
    first_name: String,
    last_name: String
}

pub async fn hello(Extension(currentUser): Extension<CurrentUser>) -> impl IntoResponse {
    Json(UserResponse {
        email: currentUser.email,
        first_name: currentUser.first_name,
        last_name: currentUser.last_name
    })
}
