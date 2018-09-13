#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod list;

use rocket as rkt;
use std::sync::Mutex;

struct ServerState {
    hit_count: Mutex<usize>
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

fn main() {
    // Manage state and serve index() at http://localhost:8000/
    rkt::ignite()
        .manage(ServerState { hit_count: Mutex::new(0) })
        .mount("/", routes![index]).launch();
}
