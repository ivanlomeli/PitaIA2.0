use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use validator::Validate;
use std::sync::Arc;

use crate::models::{ApiResponse, AuthResponse, CreateUser, LoginUser, UserProfile};
use crate::auth::{JwtService, verify_password};
use crate::repository::UserRepository;

pub async fn register(
    State(user_repo): State<Arc<UserRepository>>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(&format!("Datos inválidos: {:?}", validation_errors)))
        ));
    }

    // Verificar si el usuario ya existe
    if let Ok(Some(_)) = user_repo.find_by_username(&payload.username).await {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiResponse::error("El nombre de usuario ya está en uso"))
        ));
    }

    if let Ok(Some(_)) = user_repo.find_by_email(&payload.email).await {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiResponse::error("El email ya está registrado"))
        ));
    }

    // Crear usuario
    let user = match user_repo.create_user(&payload).await {
        Ok(user) => user,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error al crear la cuenta"))
        ))
    };

    // Generar token
    let jwt_service = JwtService::new();
    let token = match jwt_service.generate_token(user.id, &user.username) {
        Ok(token) => token,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error generando token"))
        ))
    };

    let auth_response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(auth_response, "Usuario registrado exitosamente"))
    ))
}

pub async fn login(
    State(user_repo): State<Arc<UserRepository>>,
    Json(payload): Json<LoginUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    // Buscar usuario
    let user = match user_repo.find_by_username(&payload.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error("Credenciales inválidas"))
        )),
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error del servidor"))
        ))
    };

    // Verificar contraseña
    if !verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error("Credenciales inválidas"))
        ));
    }

    // Generar token
    let jwt_service = JwtService::new();
    let token = match jwt_service.generate_token(user.id, &user.username) {
        Ok(token) => token,
        Err(_) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Error generando token"))
        ))
    };

    let auth_response = AuthResponse {
        token,
        user: user.into(),
    };

    Ok(Json(ApiResponse::success(auth_response, "Login exitoso")))
}
