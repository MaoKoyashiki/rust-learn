import { type FC, useEffect, useState } from 'react'
import './App.css'
import type { Todo, NewTodoPayload } from './types/todo'
import TodoForm from './components/TodoForm'

const App: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const fetchTodos = async () => {
    setLoading(true)
    setError(null)
    try {
      const res = await fetch('/todos')
      if (!res.ok) {
        throw new Error(`failed to fetch todos: ${res.status}`)
      }
      const data: Todo[] = await res.json()
      setTodos(data)
    } catch (e) {
      console.error(e)
      setError('TODO一覧の取得に失敗しました')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    void fetchTodos()
  }, [])

  const onSubmit = async (payload: NewTodoPayload) => {
    if (!payload.text) return
    setError(null)
    try {
      const res = await fetch('/todos', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(payload),
      })
      if (!res.ok) {
        throw new Error(`failed to create todo: ${res.status}`)
      }
      const created: Todo = await res.json()
      setTodos((prev) => [created, ...prev])
    } catch (e) {
      console.error(e)
      setError('TODOの作成に失敗しました')
    }
  }

  const toggleCompleted = async (todo: Todo) => {
    setError(null)
    try {
      const res = await fetch(`/todos/${todo.id}`, {
        method: 'PATCH',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text: todo.text,
          completed: !todo.completed,
        }),
      })
      if (!res.ok) {
        throw new Error(`failed to update todo: ${res.status}`)
      }
      const updated: Todo = await res.json()
      setTodos((prev) => prev.map((t) => (t.id === updated.id ? updated : t)))
    } catch (e) {
      console.error(e)
      setError('TODOの更新に失敗しました')
    }
  }

  const deleteTodo = async (id: number) => {
    setError(null)
    try {
      const res = await fetch(`/todos/${id}`, {
        method: 'DELETE',
      })
      if (!res.ok && res.status !== 404) {
        throw new Error(`failed to delete todo: ${res.status}`)
      }
      setTodos((prev) => prev.filter((t) => t.id !== id))
    } catch (e) {
      console.error(e)
      setError('TODOの削除に失敗しました')
    }
  }

  return (
    <div className="app-root">
      <header className="app-header">
        <h1>Todo App</h1>
      </header>
      <main className="app-main">
        <section className="todo-panel">
          <TodoForm onSubmit={onSubmit} />

          {loading && <p>読み込み中...</p>}
          {error && <p className="error-text">{error}</p>}

          <ul className="todo-list">
            {todos.map((todo) => (
              <li key={todo.id} className="todo-item">
                <label>
                  <input
                    type="checkbox"
                    checked={todo.completed}
                    onChange={() => {
                      void toggleCompleted(todo)
                    }}
                  />
                  <span className={todo.completed ? 'todo-completed' : ''}>
                    {todo.text}
                  </span>
                </label>
                <button
                  type="button"
                  className="delete-button"
                  onClick={() => {
                    void deleteTodo(todo.id)
                  }}
                >
                  削除
                </button>
              </li>
            ))}
          </ul>
        </section>
      </main>
    </div>
  )
}

export default App
