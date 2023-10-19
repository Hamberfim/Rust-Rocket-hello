#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
	"<h1>Hello, World!, Hello Rust!</h1>"
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}
