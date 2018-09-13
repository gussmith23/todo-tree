use std::vec;

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct TodoEntry {
    pub done: bool,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Hash, Serialize)]
pub struct TodoList {
    pub title: String,
    pub entries: vec::Vec<TodoEntry>,
}
