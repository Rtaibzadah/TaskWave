use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // craete unique id's for to-do items
use std::sync::Mutex;
use chrono::{Utc, DateTime}; // to get the current time
// importing modules
#[derive(Serialize, Deserialize, Clone)]
//methdo to utilise the following
struct TodoItem {
    id : Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]

struct CreateItem {
    title: String,
    completed: bool,
}

#[derive(Deserialize)]

struct UpdateItem {
    title: Option<String>,
    completed: Option<bool>,
}

struct AppState {
    todo_list: Mutex<Vec<TodoItem>>,
}

//function -> get the full list for the user
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

//create a new to-do item
async fn add_todo (
                   item: web::Json<CreateItem>,
                   data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    let new_todo = TodoItem {
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };
    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

//update a to-do item
async fn update_todo(
    path:web::Path<Uuid>,
    item: web::Json<UpdateItem>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap(); //lock the mutex to get the list

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == *path) {
        if let Some(title) = &item.title {
            todo.title = title.clone();
        }

        if let Some(completed) = item.completed {
            todo.completed = completed;
        }

        HttpResponse::Ok().json(&*todos)
    }
    else {
        HttpResponse::NotFound().body(" Todo not found ")
    }
}

//delete a to-do item
async fn delete_todo(
                    path: web::Path<Uuid>,
                    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    if todos.iter().any(|todo| todo.id == *path) {
        todos.retain(|todo| todo.id != *path);
        HttpResponse::Ok().json(&*todos)
    }
    else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

//main function
#[actix_web::main]

async fn main() -> std::io::Result<()>{
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600);


        //start the app
        App::new()
        .app_data(app_state.clone())
        .wrap(cors).route("/todos", web::get().to(get_todos))
        .route("/todos", web::post().to(add_todo))
        .route("/todos/{id}", web::put().to(update_todo))
        .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080") ? .run().await
}