use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use std::sync::Arc;

use crate::repositories::{CreateTodo, Todo, TodoRepository, UpdateTodo};

pub fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", get(list_todos::<T>).post(create_todo::<T>))
        .route(
            "/todos/:id",
            get(get_todo::<T>).patch(update_todo::<T>).delete(delete_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    "Hello, world!"
}

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo: Todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}

pub async fn list_todos<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todos = repository.all();
    (StatusCode::OK, Json(todos))
}

pub async fn get_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    match repository.find(id) {
        Some(todo) => (StatusCode::OK, Json(todo)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    match repository.update(id, payload) {
        Ok(todo) => (StatusCode::OK, Json(todo)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    match repository.delete(id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::TodoRepositoryForMemory;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, world!");
    }

    #[tokio::test]
    async fn should_create_todo() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder()
            .uri("/todos")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "text": "牛乳を買う" }"#))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::CREATED);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let todo: Todo = serde_json::from_slice(&bytes).expect("valid Todo json");

        assert_eq!(
            todo,
            Todo {
                id: 1,
                text: "牛乳を買う".to_string(),
                completed: false,
            },
        );
    }

    #[tokio::test]
    async fn should_list_all_todos() {
        let repository = TodoRepositoryForMemory::new();
        let todo1 = repository.create(CreateTodo {
            text: "タスク1".to_string(),
        });
        let todo2 = repository.create(CreateTodo {
            text: "タスク2".to_string(),
        });

        let app = create_app(repository);

        let req = Request::builder()
            .uri("/todos")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let todos: Vec<Todo> = serde_json::from_slice(&bytes).expect("valid Todo list json");

        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0], todo1);
        assert_eq!(todos[1], todo2);
    }

    #[tokio::test]
    async fn should_get_todo_by_id() {
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo {
            text: "詳細取得".to_string(),
        });

        let app = create_app(repository);

        let req = Request::builder()
            .uri(format!("/todos/{}", todo.id))
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let got: Todo = serde_json::from_slice(&bytes).expect("valid Todo json");

        assert_eq!(got, todo);
    }

    #[tokio::test]
    async fn should_return_404_when_get_todo_not_found() {
        let repository = TodoRepositoryForMemory::new();
        let app = create_app(repository);

        let req = Request::builder()
            .uri("/todos/999")
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo {
            text: "古いタイトル".to_string(),
        });

        let app = create_app(repository.clone());

        let req = Request::builder()
            .uri(format!("/todos/{}", todo.id))
            .method(Method::PATCH)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                r#"{ "text": "新しいタイトル", "completed": true }"#,
            ))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let updated: Todo = serde_json::from_slice(&bytes).expect("valid Todo json");

        assert_eq!(updated.id, todo.id);
        assert_eq!(updated.text, "新しいタイトル");
        assert!(updated.completed);

        let stored = repository.find(todo.id).expect("todo should exist");
        assert_eq!(stored, updated);
    }

    #[tokio::test]
    async fn should_return_404_when_update_todo_not_found() {
        let repository = TodoRepositoryForMemory::new();
        let app = create_app(repository);

        let req = Request::builder()
            .uri("/todos/999")
            .method(Method::PATCH)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                r#"{ "text": "存在しない", "completed": true }"#,
            ))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let repository = TodoRepositoryForMemory::new();
        let todo_to_delete = repository.create(CreateTodo {
            text: "消すタスク".to_string(),
        });
        let remaining = repository.create(CreateTodo {
            text: "残すタスク".to_string(),
        });

        let app = create_app(repository.clone());

        let req = Request::builder()
            .uri(format!("/todos/{}", todo_to_delete.id))
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        assert!(repository.find(todo_to_delete.id).is_none());
        let all = repository.all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0], remaining);
    }

    #[tokio::test]
    async fn should_return_404_when_delete_todo_not_found() {
        let repository = TodoRepositoryForMemory::new();
        let app = create_app(repository);

        let req = Request::builder()
            .uri("/todos/999")
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
