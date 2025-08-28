use axum::{
    routing::{get, post},
    Router,
    response::Json,
    middleware,
    extract::Request,
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber;
use std::net::SocketAddr;
use std::sync::Arc;

mod auth;
mod database;
mod handlers;
mod middleware;
mod models;
mod repository;

use handlers::{auth as auth_handlers, posts as post_handlers, users as user_handlers};
use repository::{UserRepository, PostRepository};
use models::ApiResponse;

#[tokio::main]
async fn main() {
    // Configurar logging
    tracing_subscriber::fmt::init();
    
    // Cargar variables de entorno
    dotenvy::dotenv().ok();
    
    println!("🍇 Iniciando Pitaia API 2.0.0...");
    
    // Conectar a la base de datos
    let pool = match database::create_pool().await {
        Ok(pool) => {
            println!("✅ Conectado a PostgreSQL");
            pool
        },
        Err(e) => {
            eprintln!("❌ Error conectando a la base de datos: {}", e);
            std::process::exit(1);
        }
    };
    
    // Crear repositorios
    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostRepository::new(pool.clone()));
    
    // Crear router principal
    let app = Router::new()
        // Rutas públicas
        .route("/", get(health_check))
        .route("/health", get(health_check))
        
        // Rutas de autenticación (públicas)
        .route("/api/auth/register", post(auth_handlers::register))
        .route("/api/auth/login", post(auth_handlers::login))
        
        // Rutas de posts
        .route("/api/posts", get(post_handlers::get_feed))
        .route("/api/posts", post(post_handlers::create_post))
        .route("/api/posts/:id/like", post(post_handlers::toggle_like))
        
        // Rutas de usuarios
        .route("/api/users/:username", get(user_handlers::get_user_profile))
        
        // Estados compartidos
        .with_state(user_repo)
        .with_state(post_repo)
        
        // Middleware global
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
        );

    // Configurar dirección del servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Servidor corriendo en http://localhost:8080");
    println!("📚 API Endpoints disponibles:");
    println!("   GET  /health");
    println!("   POST /api/auth/register");
    println!("   POST /api/auth/login");
    println!("   GET  /api/posts");
    println!("   POST /api/posts (requiere auth)");
    println!("   POST /api/posts/:id/like (requiere auth)");
    println!("   GET  /api/users/:username");
    
    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    Json(ApiResponse::success(
        serde_json::json!({
            "service": "Pitaia API",
            "version": "2.0.0",
            "status": "healthy",
            "timestamp": chrono::Utc::now(),
            "features": [
                "authentication",
                "posts",
                "likes",
                "user_profiles",
                "database"
            ]
        }),
        "Pitaia API completamente funcional"
    ))
}
