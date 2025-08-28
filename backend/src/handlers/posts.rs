use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;
use std::sync::Arc;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{ApiResponse, CreatePost, PostWithUser};
use crate::repository::PostRepository;
use crate::middleware::AuthUser;

#[derive(Deserialize)]
pub struct FeedQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create_post(
    State(post_repo): State<Arc<PostRepository>>,
    auth_user: AuthUser,
    Json(payload): Json<CreatePost>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(&format!("Contenido inválido: {:?}", validation_errors)))
        ));
    }

    let post = match post_repo.create_post(auth_user.id, &payload).await {
        Ok(post) => post,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error al crear el post"))
        ))
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(post, "Post creado exitosamente"))
    ))
}

pub async fn get_feed(
    State(post_repo): State<Arc<PostRepository>>,
    auth_user: Option<AuthUser>,
    Query(params): Query<FeedQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let limit = params.limit.unwrap_or(20).min(50);
    let offset = params.offset.unwrap_or(0);
    let user_id = auth_user.map(|u| u.id);

    let posts = match post_repo.get_feed(user_id, limit, offset).await {
        Ok(posts) => posts,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error al obtener el feed"))
        ))
    };

    Ok(Json(ApiResponse::success(posts, "Feed obtenido exitosamente")))
}

pub async fn toggle_like(
    State(post_repo): State<Arc<PostRepository>>,
    Path(post_id): Path<Uuid>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let is_liked = match post_repo.toggle_like(auth_user.id, post_id).await {
        Ok(liked) => liked,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error al procesar like"))
        ))
    };

    let message = if is_liked { "Like agregado" } else { "Like removido" };
    Ok(Json(ApiResponse::success(is_liked, message)))
}
