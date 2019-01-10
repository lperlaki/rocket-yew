#![feature(proc_macro_hygiene, decl_macro, uniform_paths)]

use common::Message;

use rocket::{catch, catchers, get, routes};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> Json<Message> {
    Json(Message {
        contents: String::from("Test"),
    })
}
#[get("/hello/<name>")]
fn hello(name: String) -> Json<Message> {
    Json(Message { contents: name })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/deploy")),
        )
        .mount("/api", routes![index, hello])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
