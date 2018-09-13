#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod todo_list;
mod todo_list_store;

use rocket as rkt;
use std::sync::Mutex;
use todo_list::TodoList;
use todo_list_store::*;

struct ServerState {
    hit_count: Mutex<usize>,
    todo_list_store: Mutex<InMemoryStore>,
}

#[get("/")]
fn index(state: rkt::State<ServerState>) -> String {
    let old_count = {
        let mut hit_count = state.hit_count.lock().unwrap();
        *hit_count += 1;
        *hit_count - 1
    };

    format!("Server hit {} times since start.", old_count)
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
            hit_count: Mutex::new(0),
            todo_list_store: Mutex::new(InMemoryStore::new()),
        }).mount("/", routes![index, create])
        .launch();
}
