use rocket::http::Status;
use rocket::{response::Redirect, State};
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[macro_use] extern crate rocket;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

pub type TodoList = Mutex<Vec<Todo>>;

#[derive(FromForm)]
struct PostData<'r> {
    r#description: &'r str,
}

#[derive(FromForm)]
struct PutData {
    completed: bool,
}

#[get("/")]
fn site_index() -> Redirect {
    Redirect::to(uri!("/todos"))
}

#[get("/")]
fn get_todos_page(items: &State<TodoList>) -> Template {
    let lock = items.lock().expect("lock app state");

    Template::render("index", context! {
        title: "Todos",
        items: lock.to_vec(),
    })
}

#[get("/<id>")]
fn get_todo_item(id: i32, items: &State<TodoList>) -> Option<Template> {
    let lock = items.lock().expect("lock app state");

    lock
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

#[post("/", data = "<data>")]
fn post_todo(data: Form<PostData<'_>>, items: &State<TodoList>) -> Template {
    let mut lock = items.lock().expect("lock app state");

    let id: i32 = 1 + lock
        .iter()
        .fold(0, |acc, item| if item.id > acc { item.id } else { acc });
    let desc = data.description.to_string();
    lock.push(Todo { id, description: desc.clone(), completed: false });

    Template::render("todo-item", context! {
        id: id,
        description: desc.clone(),
        completed: false,
    })
}

#[put("/<id>", data = "<data>")]
fn put_todo(id: i32, data: Form<PutData>, items: &State<TodoList>) -> Option<Template> {
    let mut lock = items.lock().expect("lock app state");

    lock
        .iter_mut()
        .find(|t| t.id == id)
        .map(|t| {
            t.completed = data.completed;

            Template::render("todo-item", context! {
                id: t.id,
                description: t.description.clone(),
                completed: t.completed,
            })
        })
}

#[delete("/<id>")]
fn delete_todo(id: i32, items: &State<TodoList>) -> Status {
    let mut lock = items.lock().expect("lock app state");
    lock
        .iter()
        .position(|t| t.id == id)
        .map(|i| {
            _ = lock.remove(i);

            Status::Ok
        })
        .unwrap_or(Status::NotFound)
}

#[launch]
fn rocket() -> _ {
    let items: TodoList = Mutex::new(vec![Todo {
        id: 1,
        description: "Finish the demo".to_string(),
        completed: false,
    }]);

    rocket::build()
        .manage(items)
        .mount("/", routes![site_index])
        .mount("/todos", routes![
            get_todos_page,
            get_todo_item,
            post_todo,
            put_todo,
            delete_todo,
        ])
        .attach(Template::fairing())
}

