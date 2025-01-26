use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use common::{PropertyRecord, compute_tax};
use serde_json::json;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::signal;  // We can now import this because "signal" feature is enabled

#[derive(Clone)]
struct AppState {
    db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) Connect to DB
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@db:5432/tax_db".to_string());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 2) Create table if needed
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS property_records (
          id SERIAL PRIMARY KEY,
          property_id VARCHAR NOT NULL,
          owner_name VARCHAR NOT NULL,
          street VARCHAR NOT NULL,
          city VARCHAR NOT NULL,
          state VARCHAR NOT NULL,
          zip VARCHAR NOT NULL,
          assessed_value FLOAT NOT NULL,
          location_code VARCHAR NOT NULL,
          is_overdue BOOLEAN NOT NULL,
          last_payment_unix BIGINT,
          computed_tax FLOAT NOT NULL,
          created_at TIMESTAMP NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    let shared_state = Arc::new(AppState { db_pool: pool });

    // 3) Axum routes
    let app = Router::new()
        .route("/api/compute", post(handle_compute))
        .with_state(shared_state);

    let addr = "0.0.0.0:8080";
    println!("Compute listening on {}", addr);

    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn handle_compute(
    State(app_state): State<Arc<AppState>>,
    Json(record): Json<PropertyRecord>,
) -> Json<serde_json::Value> {
    // 1) compute the tax
    let tax_amount = compute_tax(&record);

    // 2) Insert into DB
    let _ = sqlx::query(
        r#"
        INSERT INTO property_records
        (property_id, owner_name, street, city, state, zip,
         assessed_value, location_code, is_overdue, last_payment_unix, computed_tax, created_at)
        VALUES
        ($1, $2, $3, $4, $5, $6,
         $7, $8, $9, $10, $11, NOW())
        "#,
    )
    .bind(&record.property_id)
    .bind(&record.owner_name)
    .bind(&record.address.street)
    .bind(&record.address.city)
    .bind(&record.address.state)
    .bind(&record.address.zip)
    .bind(record.assessed_value)
    .bind(&record.location_code)
    .bind(record.is_overdue)
    // last_payment_unix is Option<i64>, which is encodable for BIGINT
    .bind(record.last_payment_unix)
    .bind(tax_amount)
    .execute(&app_state.db_pool)
    .await;

    // 3) Return JSON
    Json(json!({
        "property_id": record.property_id,
        "computed_tax": tax_amount,
        "owner": record.owner_name,
        "message": "Recorded property in DB with computed tax"
    }))
}

// Example shutdown using tokio::signal for ctrl+c
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };
    tokio::select! {
        _ = ctrl_c => {},
    }
    println!("Compute shutting down gracefully...");
}
