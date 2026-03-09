import { type FC, useState } from 'react'
import type { NewTodoPayload } from '../types/todo'

type Props = {
  onSubmit: (newTodo: NewTodoPayload) => void | Promise<void>
}

const TodoForm: FC<Props> = ({ onSubmit }) => {
  const [editText, setEditText] = useState('')

  const addTodoHandler = async () => {
    if (!editText) return

    await onSubmit({
      text: editText,
    })
    setEditText('')
  }

  return (
    <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
      <input
        style={{ flex: 1, padding: 8 }}
        placeholder="新しいTODOを入力"
        value={editText}
        onChange={(e) => setEditText(e.target.value)}
      />
      <button type="button" onClick={addTodoHandler}>
        追加
      </button>
    </div>
  )
}

export default TodoForm

