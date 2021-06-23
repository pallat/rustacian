#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u32) -> String {
    format!("Hello {} age {}",name,age)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/hello", routes![hello])
}