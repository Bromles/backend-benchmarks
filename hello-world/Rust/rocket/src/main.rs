use rocket::{get, routes, Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
