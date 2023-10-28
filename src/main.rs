use rocket::{response::Redirect, State};
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};

#[macro_use] extern crate rocket;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

pub type TodoList = Vec<Todo>;

#[get("/")]
fn site_index() -> Redirect {
    Redirect::to(uri!("/todos"))
}

#[get("/")]
fn get_todos_page(items: &State<TodoList>) -> Template {
    Template::render("index", context! {
        title: "Todos",
        items: items.to_vec(),
    })
}

#[get("/<id>")]
fn get_todo_item(id: i32, items: &State<TodoList>) -> Option<Template> {
    items
        .iter()
        .find(|&ti| ti.id == id)
        .map(|ti| {
            Template::render("todo-item", context! {
                id: ti.id,
                description: ti.description.clone(),
                completed: ti.completed,
            })
        })
}

#[launch]
fn rocket() -> _ {
    let items: TodoList = vec![Todo {
        id: 1,
        description: "Finish the demo".to_string(),
        completed: false,
    }];

    rocket::build()
        .manage(items)
        .mount("/", routes![site_index])
        .mount("/todos", routes![
            get_todos_page,
            get_todo_item,
        ])
        .attach(Template::fairing())
}

