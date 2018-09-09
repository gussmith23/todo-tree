#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket as rkt;

#[get("/")]
fn index() -> &'static str {
    "Nothing here."
}

fn main() {
    // Serve index() at http://localhost:8000/
    rkt::ignite().mount("/", routes![index]).launch();
}
