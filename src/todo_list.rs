use std::vec;

#[derive(Clone, Debug, Hash)]
pub struct TodoEntry {
    pub done: bool,
    pub text: String,
}

#[derive(Clone, Debug, Hash)]
pub struct TodoList {
    pub title: String,
    pub entries: vec::Vec<TodoEntry>,
}
