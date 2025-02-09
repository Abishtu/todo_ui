use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files as afs;
use lazy_static::lazy_static;
use tera::Tera;
use todo_rust_openapi::apis::configuration::Configuration;
use todo_rust_openapi::apis::default_api::task_get;

lazy_static! {
    pub static ref API_CONFIGURATION: Configuration = {
        let mut config = Configuration::new();
        config.base_path = String::from("http://localhost:3080/api");
        config
    };

    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new("www/**/*.html").unwrap();
        tera
    };
}

#[get("/tasks")]
async fn tasks_endpoint() -> impl Responder {
    let mut context = tera::Context::new();

    let task_list = task_get(
        &API_CONFIGURATION,
        None, None, None, None,
    ).await.unwrap().data;

    context.insert("tasks", &task_list);

    let html = TEMPLATES.render("task_list.html", &context).unwrap();

    HttpResponse::Ok().body(html)
}

#[get("/")]
async fn index() -> impl Responder {
    let mut context = tera::Context::new();

    let task_list = task_get(
        &API_CONFIGURATION,
        None, None, None, None,
    ).await.unwrap().data;

    context.insert("tasks", &task_list);

    let html = TEMPLATES.render("pages_index.html", &context).unwrap();

    HttpResponse::Ok().body(html)
}

fn to_title_case(s: &str) -> String {
    format!("{}{}", s.get(0).to_string().to_uppercase(), s[1:])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at: http://localhost:3180");

    HttpServer::new(move || {
        App::new()
            .service(
                afs::Files::new("/public", "www/public")
                    .use_last_modified(true),
            )
            .service(index)
            .service(tasks_endpoint)
    }).bind(("127.0.0.1", 3180))?.run().await
}
