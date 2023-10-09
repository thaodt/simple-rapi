// use std::sync::Arc;
use axum::{Extension, Router};
use item::{
    item_service_server::{self, ItemService},
    CreateItemRequest, DeleteItemRequest, GetItemRequest, UpdateItemRequest,
};
// use sqlx::PgPool;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

#[path = "out/item.rs"]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod item;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::create().await;
    let addr: SocketAddr = "0.0.0.0:50051".parse()?;

    let grpc_service = item_service_server::ItemServiceServer::new(app_state);

    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "Hello, world!" }))
        .layer(Extension(grpc_service));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    println!("Server listening on {}", addr);
    Ok(())
}

struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl AppState {
    async fn create() -> Self {
        // maybe i dont need config crate for this, change the way to get db info by docker secrets
        // let cfg_builder = ConfigBuilder::from_env().await.unwrap();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("Failed to create a database connection pool");
        Self { pool: db_pool }
    }
}

#[tonic::async_trait]
impl ItemService for AppState {
    async fn create_item(
        &self,
        request: Request<CreateItemRequest>,
    ) -> Result<Response<item::Item>, Status> {
        let req = request.into_inner();
        let result = sqlx::query!(
            r#"
            INSERT INTO items (name, description)
            VALUES ($1, $2)
            RETURNING id, name, description
            "#,
            req.name,
            req.description
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => {
                let item = item::Item {
                    id: record.id,
                    name: record.name,
                    description: record.description,
                };
                Ok(Response::new(item))
            }
            Err(e) => {
                eprintln!("Error creating item: {}", e);
                Err(Status::internal("Error creating item"))
            }
        }
    }

    async fn get_item(
        &self,
        request: Request<GetItemRequest>,
    ) -> Result<Response<item::Item>, Status> {
        let req = request.into_inner();
        let result = sqlx::query!(
            r#"
            SELECT id, name, description
            FROM items
            WHERE id = $1
            "#,
            req.id
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => {
                let item = item::Item {
                    id: record.id,
                    name: record.name,
                    description: record.description,
                };
                Ok(Response::new(item))
            }
            Err(e) => {
                eprintln!("Error getting item: {}", e);
                Err(Status::internal("Error getting item"))
            }
        }
    }

    async fn update_item(
        &self,
        request: Request<UpdateItemRequest>,
    ) -> Result<Response<item::Item>, Status> {
        let req = request.into_inner();
        let result = sqlx::query!(
            r#"
            UPDATE items
            SET name = $1, description = $2
            WHERE id = $3
            RETURNING id, name, description
            "#,
            req.name,
            req.description,
            req.id
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => {
                let item = item::Item {
                    id: record.id,
                    name: record.name,
                    description: record.description,
                };
                Ok(Response::new(item))
            }
            Err(e) => {
                eprintln!("Error updating item: {}", e);
                Err(Status::internal("Error updating item"))
            }
        }
    }

    async fn delete_item(
        &self,
        request: Request<DeleteItemRequest>,
    ) -> Result<Response<item::Empty>, Status> {
        let req = request.into_inner();
        let result = sqlx::query!(
            r#"
            DELETE FROM items
            WHERE id = $1
            "#,
            req.id
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(Response::new(item::Empty {})),
            Err(e) => {
                eprintln!("Error deleting item: {}", e);
                Err(Status::internal("Error deleting item"))
            }
        }
    }
}
