#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/")]
fn post_index() -> &'static str {
    "Hello, Post!"
}

#[get("/hello/<name>")]
fn hello_dynamic(name: &RawStr) -> String {
    format!("Hello, {}", name.as_str())
}

#[get("/new/<id>")]
fn new_article (id: Result<usize, &RawStr>) -> String {
    match id {
        Ok(uid) => format!("It's a 'real' id, let's do something with it. (Id = {})", uid),
        Err(crap) => format!("Either it's a isize or it's a crap (Crap = {})", crap)
    }
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, post_index, hello_dynamic])
    .mount("/article", routes![new_article])
    .launch();
}
