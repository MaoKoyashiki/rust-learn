use axum::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use sqlx::{FromRow, PgPool};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected Error: [{0}]")]
    Unexpected(String),
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self, id: i32) -> anyhow::Result<Todo>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    pub text: Option<String>,
    pub completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForMemory {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
        let mut store = self.store.write().expect("Todo store lock poisoned");
        let next_id = store.keys().max().copied().unwrap_or(0) + 1;
        let todo = Todo::new(next_id, payload.text);
        store.insert(todo.id, todo.clone());
        Ok(todo)
    }

    async fn find(&self, id: i32) -> anyhow::Result<Todo> {
        let store = self.store.read().expect("Todo store lock poisoned");
        let todo = store.get(&id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("Todo with id {} not found", id))?;
        
        Ok(todo)
    }

    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let store = self.store.read().expect("Todo store lock poisoned");
        let mut todos: Vec<Todo> = store.values().cloned().collect();
        todos.sort_by_key(|t| t.id);
        Ok(todos)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let mut store = self.store.write().expect("Todo store lock poisoned");
        let todo = store
            .get_mut(&id)
            .ok_or_else(|| RepositoryError::NotFound(id))?;

        if let Some(text) = payload.text {
            todo.text = text;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }

        Ok(todo.clone())
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.store.write().expect("Todo store lock poisoned");
        match store.remove(&id) {
            Some(_) => Ok(()),
            None => Err(RepositoryError::NotFound(id).into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TodoRepositoryForDb {
    pool: PgPool,
}

impl TodoRepositoryForDb {
    pub fn new(pool: PgPool) -> Self {
        TodoRepositoryForDb {
            pool
        }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryForDb {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo> {
        println!("aaa");
        let todo = sqlx::query_as::<_, Todo>(
            r#"
insert into todos (text, completed)
values ($1, false)
returning *
            "#,
            )
            .bind(payload.text.clone())
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)
    }

    async fn find(&self, id: i32) -> anyhow::Result<Todo> {
        let todo = sqlx::query_as::<_, Todo>(
            r#"
select * from todos where id=$1
            "#,
            )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
                _ => RepositoryError::Unexpected(e.to_string()),
            })?;

            Ok(todo)
    }

    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
select * from todos
            "#,
            )
            .fetch_all(&self.pool)
            .await?;

            Ok(todos)
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        let old_todo = self.find(id).await?;
        let todo = sqlx::query_as::<_, Todo>(
            r#"
update todos set text=$1, completed=$2
where id=$3
returning *
            "#,
            )
            .bind(payload.text.unwrap_or(old_todo.text))
            .bind(payload.completed.unwrap_or(old_todo.completed))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(todo)

    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
delete from todos where id=$1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => 
                RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;
    
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_todo_should_store_and_return_todo() {
        let repo = TodoRepositoryForMemory::new();

        let todo = repo.create(CreateTodo {
            text: "テストを書く".to_string(),
        }).await.expect("Failed to create todo");

        assert_eq!(todo.id, 1);
        assert_eq!(todo.text, "テストを書く");
        assert!(!todo.completed);

        let all = repo.all().await.expect("Failed to get all");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0], todo);
    }

    #[tokio::test]
    async fn find_should_return_corresponding_todo_or_none() {
        let repo = TodoRepositoryForMemory::new();

        let todo1 = repo.create(CreateTodo {
            text: "牛乳を買う".to_string(),
        }).await.expect("Failed to create todo");
        let _todo2 = repo.create(CreateTodo {
            text: "パンを買う".to_string(),
        }).await.expect("Failed to create todo");

        let found = repo.find(todo1.id).await.expect("Failed to get");
        assert_eq!(found, todo1);

        let res = repo.find(999).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn all_should_return_all_todos_sorted_by_id() {
        let repo = TodoRepositoryForMemory::new();

        let todo1 = repo.create(CreateTodo {
            text: "タスク1".to_string(),
        }).await.expect("Failed to create todo");
        let todo2 = repo.create(CreateTodo {
            text: "タスク2".to_string(),
        }).await.expect("Failed to create todo");
        let todo3 = repo.create(CreateTodo {
            text: "タスク3".to_string(),
        }).await.expect("Failed to create todo");

        let all = repo.all().await.expect("Failed to get");
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], todo1);
        assert_eq!(all[1], todo2);
        assert_eq!(all[2], todo3);
    }

    #[tokio::test]
    async fn update_should_change_fields_when_todo_exists() {
        let repo = TodoRepositoryForMemory::new();

        let todo = repo.create(CreateTodo {
            text: "古いタスク".to_string(),
        }).await.expect("Failed to create todo");

        let updated = repo
            .update(
                todo.id,
                UpdateTodo {
                    text: Some("新しいタスク".to_string()),
                    completed: Some(true),
                },
            )
            .await.expect("update should succeed");

        assert_eq!(updated.id, todo.id);
        assert_eq!(updated.text, "新しいタスク");
        assert!(updated.completed);

        let stored = repo.find(todo.id).await.expect("todo should exist");
        assert_eq!(stored, updated);
    }

    #[tokio::test]
    async fn update_should_return_error_when_todo_not_found() {
        let repo = TodoRepositoryForMemory::new();

        let result = repo.update(
            999,
            UpdateTodo {
                text: Some("存在しない".to_string()),
                completed: Some(true),
            },
        ).await;

        assert!(result.is_err());
        let msg = format!("{}", result.err().unwrap());
        assert!(msg.contains("NotFound, id is 999"));
    }

    #[tokio::test]
    async fn delete_should_remove_todo_when_exists() {
        let repo = TodoRepositoryForMemory::new();

        let todo = repo.create(CreateTodo {
            text: "消すタスク".to_string(),
        }).await.expect("Failed to create todo");
        let _another = repo.create(CreateTodo {
            text: "残すタスク".to_string(),
        }).await;

        let result = repo.delete(todo.id).await;
        let all = repo.all().await.expect("Failed to get");
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].text, "残すタスク".to_string());
    }

    #[tokio::test]
    async fn delete_should_return_error_when_todo_not_found() {
        let repo = TodoRepositoryForMemory::new();

        let result = repo.delete(999).await;
        assert!(result.is_err());
        let msg = format!("{}", result.err().unwrap());
        assert!(msg.contains("NotFound, id is 999"));
    }
}
