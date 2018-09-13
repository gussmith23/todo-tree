use todo_list::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TodoListId(u64);

pub trait TodoListStore {
    fn create(&mut self, todo_list: &TodoList) -> Result<TodoListId, ()>;
    fn read(&self, id: TodoListId) -> Result<TodoList, ()>;
    fn update(&mut self, id: TodoListId, todo_list: &TodoList) -> Result<(), ()>;
    fn delete(&mut self, id: TodoListId) -> Result<(), ()>;
}
