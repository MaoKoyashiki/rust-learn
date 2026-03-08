-- Add migration script here
CREATE TABLE todos (
    -- id: 自動採番される主キー
    id SERIAL PRIMARY KEY,
    
    -- text: Todoの内容。1文字以上100文字以内という制約をDB側でも保証
    text VARCHAR(100) NOT NULL CHECK (length(text) >= 1),
    
    -- completed: 完了ステータス。デフォルトは未完了(false)
    completed BOOLEAN NOT NULL DEFAULT false
);