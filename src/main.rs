use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;

#[get("/")]
fn site_index() -> Redirect {
    Redirect::to(uri!("/todos"))
}

#[get("/")]
fn get_todos_page() -> Template {
    Template::render("index", context! {
        title: "Todo App",
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![site_index])
        .mount("/todos", routes![get_todos_page])
        .attach(Template::fairing())
}

