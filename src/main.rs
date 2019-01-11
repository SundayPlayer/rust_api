#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/")]
fn post_index() -> &'static str {
    "Hello, Post!"
}

#[get("/hello/<name>")]
fn hello_dynamic(name: String) -> String {
    format!("Hello, {}", name)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, post_index, hello_dynamic])
    .launch();
}
