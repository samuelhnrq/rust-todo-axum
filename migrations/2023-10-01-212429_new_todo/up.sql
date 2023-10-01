CREATE TABLE IF NOT EXISTS todos(
  todo_id SERIAL PRIMARY KEY,
  todo_title TEXT NOT NULL,
  todo_description TEXT NOT NULL,
  todo_done BOOLEAN NOT NULL DEFAULT FALSE
);
