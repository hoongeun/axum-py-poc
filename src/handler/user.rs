use crate::model::user::{CreateUser, User};
use axum::{http::StatusCode, Json};

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
