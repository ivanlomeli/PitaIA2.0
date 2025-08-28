use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Post, PostWithUser, CreatePost};

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_post(&self, user_id: Uuid, data: &CreatePost) -> Result<Post> {
        let mut tx = self.pool.begin().await?;
        
        // Crear el post
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (user_id, content, image_url)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            user_id,
            data.content,
            data.image_url
        )
        .fetch_one(&mut *tx)
        .await?;

        // Incrementar contador de posts del usuario
        sqlx::query!(
            "UPDATE users SET posts_count = posts_count + 1 WHERE id = $1",
            user_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(post)
    }

    pub async fn get_feed(&self, user_id: Option<Uuid>, limit: i64, offset: i64) -> Result<Vec<PostWithUser>> {
        let posts = if let Some(user_id) = user_id {
            sqlx::query_as!(
                PostWithUser,
                r#"
                SELECT 
                    p.id,
                    p.user_id,
                    u.username,
                    u.display_name,
                    u.avatar_url,
                    p.content,
                    p.image_url,
                    p.likes_count,
                    p.comments_count,
                    p.created_at,
                    CASE WHEN l.user_id IS NOT NULL THEN true ELSE false END as "is_liked: bool"
                FROM posts p
                JOIN users u ON p.user_id = u.id
                LEFT JOIN likes l ON p.id = l.post_id AND l.user_id = $1
                ORDER BY p.created_at DESC
                LIMIT $2 OFFSET $3
                "#,
                user_id,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                PostWithUser,
                r#"
                SELECT 
                    p.id,
                    p.user_id,
                    u.username,
                    u.display_name,
                    u.avatar_url,
                    p.content,
                    p.image_url,
                    p.likes_count,
                    p.comments_count,
                    p.created_at,
                    NULL as "is_liked: Option<bool>"
                FROM posts p
                JOIN users u ON p.user_id = u.id
                ORDER BY p.created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(posts)
    }

    pub async fn toggle_like(&self, user_id: Uuid, post_id: Uuid) -> Result<bool> {
        let mut tx = self.pool.begin().await?;
        
        // Verificar si ya existe el like
        let existing_like = sqlx::query!(
            "SELECT id FROM likes WHERE user_id = $1 AND post_id = $2",
            user_id,
            post_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        let is_liked = if existing_like.is_some() {
            // Quitar like
            sqlx::query!(
                "DELETE FROM likes WHERE user_id = $1 AND post_id = $2",
                user_id,
                post_id
            )
            .execute(&mut *tx)
            .await?;

            // Decrementar contador
            sqlx::query!(
                "UPDATE posts SET likes_count = likes_count - 1 WHERE id = $1",
                post_id
            )
            .execute(&mut *tx)
            .await?;

            false
        } else {
            // Agregar like
            sqlx::query!(
                "INSERT INTO likes (user_id, post_id) VALUES ($1, $2)",
                user_id,
                post_id
            )
            .execute(&mut *tx)
            .await?;

            // Incrementar contador
            sqlx::query!(
                "UPDATE posts SET likes_count = likes_count + 1 WHERE id = $1",
                post_id
            )
            .execute(&mut *tx)
            .await?;

            true
        };

        tx.commit().await?;
        Ok(is_liked)
    }
}
