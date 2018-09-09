#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Nothing here."
}

fn main() {
    // Serve index() at http://localhost:8000/
    rocket::ignite().mount("/", routes![index]).launch();
}
