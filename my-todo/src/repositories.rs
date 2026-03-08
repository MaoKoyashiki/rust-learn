use validator::Validate; //
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        let mut store = self.store.write().expect("Todo store lock poisoned");
        let next_id = store.keys().max().copied().unwrap_or(0) + 1;
        let todo = Todo::new(next_id, payload.text);
        store.insert(todo.id, todo.clone());
        todo
    }

    fn find(&self, id: i32) -> Option<Todo> {
        let store = self.store.read().expect("Todo store lock poisoned");
        store.get(&id).cloned()
    }

    fn all(&self) -> Vec<Todo> {
        let store = self.store.read().expect("Todo store lock poisoned");
        let mut todos: Vec<Todo> = store.values().cloned().collect();
        todos.sort_by_key(|t| t.id);
        todos
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
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

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.store.write().expect("Todo store lock poisoned");
        match store.remove(&id) {
            Some(_) => Ok(()),
            None => Err(RepositoryError::NotFound(id).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_todo_should_store_and_return_todo() {
        let repo = TodoRepositoryForMemory::new();

        let todo = repo.create(CreateTodo {
            text: "テストを書く".to_string(),
        });

        assert_eq!(todo.id, 1);
        assert_eq!(todo.text, "テストを書く");
        assert!(!todo.completed);

        let all = repo.all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0], todo);
    }

    #[test]
    fn find_should_return_corresponding_todo_or_none() {
        let repo = TodoRepositoryForMemory::new();

        let todo1 = repo.create(CreateTodo {
            text: "牛乳を買う".to_string(),
        });
        let _todo2 = repo.create(CreateTodo {
            text: "パンを買う".to_string(),
        });

        let found = repo.find(todo1.id);
        assert_eq!(found, Some(todo1));

        let not_found = repo.find(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn all_should_return_all_todos_sorted_by_id() {
        let repo = TodoRepositoryForMemory::new();

        let todo1 = repo.create(CreateTodo {
            text: "タスク1".to_string(),
        });
        let todo2 = repo.create(CreateTodo {
            text: "タスク2".to_string(),
        });
        let todo3 = repo.create(CreateTodo {
            text: "タスク3".to_string(),
        });

        let all = repo.all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0], todo1);
        assert_eq!(all[1], todo2);
        assert_eq!(all[2], todo3);
    }

    #[test]
    fn update_should_change_fields_when_todo_exists() {
        let repo = TodoRepositoryForMemory::new();

        let todo = repo.create(CreateTodo {
            text: "古いタスク".to_string(),
        });

        let updated = repo
            .update(
                todo.id,
                UpdateTodo {
                    text: Some("新しいタスク".to_string()),
                    completed: Some(true),
                },
            )
            .expect("update should succeed");

        assert_eq!(updated.id, todo.id);
        assert_eq!(updated.text, "新しいタスク");
        assert!(updated.completed);

        let stored = repo.find(todo.id).expect("todo should exist");
        assert_eq!(stored, updated);
    }

    #[test]
    fn update_should_return_error_when_todo_not_found() {
        let repo = TodoRepositoryForMemory::new();

        let result = repo.update(
            999,
            UpdateTodo {
                text: Some("存在しない".to_string()),
                completed: Some(true),
            },
        );

        assert!(result.is_err());
        let msg = format!("{}", result.err().unwrap());
        assert!(msg.contains("NotFound, id is 999"));
    }

    #[test]
    fn delete_should_remove_todo_when_exists() {
        let repo = TodoRepositoryForMemory::new();

        let todo = repo.create(CreateTodo {
            text: "消すタスク".to_string(),
        });
        let _another = repo.create(CreateTodo {
            text: "残すタスク".to_string(),
        });

        let result = repo.delete(todo.id);
        assert!(result.is_ok());

        assert!(repo.find(todo.id).is_none());
        let all = repo.all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].text, "残すタスク".to_string());
    }

    #[test]
    fn delete_should_return_error_when_todo_not_found() {
        let repo = TodoRepositoryForMemory::new();

        let result = repo.delete(999);
        assert!(result.is_err());
        let msg = format!("{}", result.err().unwrap());
        assert!(msg.contains("NotFound, id is 999"));
    }
}
