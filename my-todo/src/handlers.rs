use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, RequestParts},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    BoxError, Json, Router,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

use crate::repositories::{CreateTodo, Todo, TodoRepository, UpdateTodo};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate, //
    B: http_body::Body + Send,      //
    B::Data: Send,
    B::Error: Into<axum::BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // JSONとしてのパースを試行
        let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
            let message = format!("Json parse error: [{}]", rejection); //
            (StatusCode::BAD_REQUEST, message)
        })?;

        // データのバリデーション実行
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", "); //
            (StatusCode::BAD_REQUEST, message)
        })?;

        Ok(ValidatedJson(value))
    }
}

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
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = repository.create(payload).await;
    let todo: Todo = match result {
        Ok(t) => t,
        Err(_) => return Err(StatusCode::NOT_FOUND)
    };
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn list_todos<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = repository.all().await;
    let todos = match result {
        Ok(t) => t,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok((StatusCode::OK, Json(todos)))
}

pub async fn get_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let result = repository.find(id).await;
    let todo: Todo = match result {
        Ok(maybe_todo) => match maybe_todo {
            Some(t) => t,
            None => return Err(StatusCode::NOT_FOUND),
        },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    let result = repository.delete(id).await;
    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(e) if e.to_string().to_lowercase().contains("notfound") => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
    async fn should_return_400_when_create_todo_with_empty_text() {
        let repository = TodoRepositoryForMemory::new();
        let req = Request::builder()
            .uri("/todos")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "text": "" }"#))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn should_return_400_when_create_todo_with_text_over_100_chars() {
        let repository = TodoRepositoryForMemory::new();
        let long_text = "a".repeat(101);
        let req = Request::builder()
            .uri("/todos")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(format!(r#"{{ "text": "{}" }}"#, long_text)))
            .unwrap();
        let res = create_app(repository).oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn should_return_400_when_update_todo_with_empty_text() {
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo {
            text: "元のテキスト".to_string(),
        }).await.expect("Failed to create todo");

        let app = create_app(repository);

        let req = Request::builder()
            .uri(format!("/todos/{}", todo.id))
            .method(Method::PATCH)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "text": "" }"#))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn should_return_400_when_update_todo_with_text_over_100_chars() {
        let repository = TodoRepositoryForMemory::new();
        let todo = repository.create(CreateTodo {
            text: "元のテキスト".to_string(),
        }).await.expect("Failed to create todo");
        let long_text = "a".repeat(101);

        let app = create_app(repository);

        let req = Request::builder()
            .uri(format!("/todos/{}", todo.id))
            .method(Method::PATCH)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(format!(r#"{{ "text": "{}" }}"#, long_text)))
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn should_list_all_todos() {
        let repository = TodoRepositoryForMemory::new();
        let todo1 = repository.create(CreateTodo {
            text: "タスク1".to_string(),
        }).await.expect("Failed to create todo");
        let todo2 = repository.create(CreateTodo {
            text: "タスク2".to_string(),
        }).await.expect("Failed to create todo");

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
        }).await.expect("Failed to create todo");

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
        }).await.expect("Failed to create todo");

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

        let stored = repository.find(todo.id).await.expect("todo should exist");
        assert_eq!(stored, Some(updated));
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
        }).await.expect("Failed to create todo");
        let remaining = repository.create(CreateTodo {
            text: "残すタスク".to_string(),
        }).await.expect("Failed to create todo");

        let app = create_app(repository.clone());

        let req = Request::builder()
            .uri(format!("/todos/{}", todo_to_delete.id))
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        let deleted_todo = repository.find(todo_to_delete.id).await.expect("DB error");
        assert!(deleted_todo.is_none());
        let all = repository.all().await.expect("Failed to get all");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0], remaining);
    }

    #[tokio::test]
    async fn sould_hreturn_404_when_delete_todo_not_found() {
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
