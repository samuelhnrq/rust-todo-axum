// @generated automatically by Diesel CLI.

diesel::table! {
    todos (todo_id) {
        todo_id -> Int4,
        todo_title -> Text,
        todo_description -> Text,
        todo_done -> Bool,
    }
}
