#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod todo_list;
mod todo_list_store;

use rocket as rkt;
use std::sync::Mutex;
use todo_list::TodoList;
use todo_list_store::*;

struct ServerState {
    todo_list_store: Mutex<InMemoryStore>,
}

#[post("/create", format = "media/text", data = "<title>")]
fn create(state: rkt::State<ServerState>, title: String) -> String {
    let todo_list = TodoList {
        title: title.to_string(),
        entries: Default::default(),
    };
    let mut list_store = state.todo_list_store.lock().unwrap();
    let id = list_store.create(&todo_list).unwrap();
    format!("Createe Todo List with id {}.", id.0)
}

fn main() {
    // Manage state and serve index() at http://localhost:8000/
    rkt::ignite()
        .manage(ServerState {
            todo_list_store: Mutex::new(InMemoryStore::new()),
        }).mount("/", routes![create])
        .launch();
}
