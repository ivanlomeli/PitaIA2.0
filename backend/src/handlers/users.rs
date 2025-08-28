use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::models::{ApiResponse, UserProfile};
use crate::repository::UserRepository;

pub async fn get_user_profile(
    State(user_repo): State<Arc<UserRepository>>,
    Path(username): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    if username.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error("Username requerido"))
        ));
    }

    let user = match user_repo.find_by_username(&username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error("Usuario no encontrado"))
        )),
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error del servidor"))
        ))
    };

    let user_profile: UserProfile = user.into();
    Ok(Json(ApiResponse::success(user_profile, "Perfil obtenido exitosamente")))
}
