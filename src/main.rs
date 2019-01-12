#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::request::Form;

#[derive(FromForm)]
struct Article {
    id: usize,
    title: String,
    body: String,
}

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

#[get("/<id>")]
fn get_article (id: Result<usize, &RawStr>) -> String {
    match id {
        Ok(uid) => format!("It's a 'real' id, let's do something with it. (Id = {})", uid),
        Err(crap) => format!("Either it's a isize or it's a crap (Crap = {})", crap)
    }
}

#[post("/new", data = "<article>")]
fn create_article (article: Form<Article>) -> String {
    format!("Article is: id:{}, title:{}, body:{}", article.id, article.title, article.body)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, post_index, hello_dynamic])
    .mount("/article", routes![get_article, create_article])
    .launch();
}
