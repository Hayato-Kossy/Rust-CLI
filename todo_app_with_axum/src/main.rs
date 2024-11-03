use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Task {
    id: i64,
    description: String,
    completed: bool,
}


#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Pool<Sqlite>>>,
}

#[tokio::main]
async fn main() {
    // データベースプールを作成
    let db = SqlitePoolOptions::new()
        .connect("sqlite:database.sqlite")
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    // ルーターの設定
    let app = Router::new()
        .route("/tasks", post(add_task).get(get_tasks))
        .route("/tasks/:id/complete", post(complete_task))
        .with_state(state);

    println!("Server running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 新しいタスクを追加
async fn add_task(
    State(state): State<AppState>,
    Json(payload): Json<TaskRequest>,
) -> impl IntoResponse {
    let db = state.db.lock().await;
    let query = "INSERT INTO tasks (description, completed) VALUES (?, ?)";
    sqlx::query(query)
        .bind(&payload.description)
        .bind(false)
        .execute(&*db)
        .await
        .expect("Failed to insert task");

    (StatusCode::CREATED, "Task added")
}

// すべてのタスクを取得
async fn get_tasks(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db.lock().await;
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(&*db)
        .await
        .expect("Failed to fetch tasks");

    Json(tasks)
}

// タスクを完了済みに更新
async fn complete_task(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    let db = state.db.lock().await;
    let query = "UPDATE tasks SET completed = 1 WHERE id = ?";
    let result = sqlx::query(query).bind(id).execute(&*db).await;

    match result {
        Ok(_) => (StatusCode::OK, "Task marked as completed"),
        Err(_) => (StatusCode::NOT_FOUND, "Task not found"),
    }
}

// タスクを追加するためのリクエスト構造体
#[derive(Deserialize)]
struct TaskRequest {
    description: String,
}
