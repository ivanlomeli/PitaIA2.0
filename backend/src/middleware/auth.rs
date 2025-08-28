use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode, HeaderMap},
    response::{Json, IntoResponse},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::JwtService;
use crate::repository::UserRepository;
use crate::models::ApiResponse;

pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;
        let token = extract_token_from_header(headers)
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::error("Token de autorización requerido"))
                )
            })?;

        let jwt_service = JwtService::new();
        let claims = jwt_service.verify_token(&token)
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::error("Token inválido"))
                )
            })?;

        let user_id = claims.sub.parse::<Uuid>()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse::error("Token malformado"))
                )
            })?;

        Ok(AuthUser {
            id: user_id,
            username: claims.username,
        })
    }
}

fn extract_token_from_header(headers: &HeaderMap) -> Option<String> {
    let header = headers.get("Authorization")?;
    let auth_header = header.to_str().ok()?;
    
    if auth_header.starts_with("Bearer ") {
        Some(auth_header[7..].to_string())
    } else {
        None
    }
}
