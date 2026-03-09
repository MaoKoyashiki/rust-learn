import { type FC, useState } from 'react'
import type { NewTodoPayload } from '../types/todo'
import { Box, Button, TextField, Paper } from '@mui/material'

type Props = {
  // 非同期のonSubmitも許容してバックエンド連携に対応
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
    <Paper elevation={2}>
      <Box sx={{ p: 2 }}>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
          <TextField
            label="new todo text"
            variant="filled"
            value={editText}
            onChange={(e) => setEditText(e.target.value)}
            fullWidth
          />
          <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
            <Button onClick={addTodoHandler} variant="contained">
              add todo
            </Button>
          </Box>
        </Box>
      </Box>
    </Paper>
  )
}

export default TodoForm