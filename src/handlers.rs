use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use uuid::Uuid;
use validator::Validate;

use crate::errors::AppError;
use crate::{
    models::{Friend, NewFriend},
    repository::FriendRepository,
};

pub async fn list_friends(State(repository): State<FriendRepository>) -> Json<Vec<Friend>> {
    return Json(repository.list());
}

pub async fn create_friend(
    State(repository): State<FriendRepository>,
    Json(friend_params): Json<NewFriend>,
) -> Result<Json<Friend>, AppError> {
    friend_params.validate()?;
    let new_friend = repository.add(friend_params);
    return Ok(Json(new_friend));
}

pub async fn get_friend(
    State(repository): State<FriendRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<Friend>, AppError> {
    return repository
        .get(&id)
        .map(Json)
        .ok_or_else(|| AppError::not_found(format!("friend with ID {id} not found")));
}

pub async fn delete_friend(
    State(repository): State<FriendRepository>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    repository.delete(&id);
    return StatusCode::NO_CONTENT;
}

pub async fn update_friend(
    State(repository): State<FriendRepository>,
    Path(id): Path<Uuid>,
    Json(friend_params): Json<NewFriend>,
) -> Result<Json<Friend>, AppError> {
    friend_params.validate()?;

    return repository
        .update(&id, friend_params)
        .map(Json)
        .ok_or_else(|| AppError::not_found(format!("friend with ID {id} was not found")));
}
