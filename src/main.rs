use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
use handlebars::Handlebars;
use std::collections::BTreeMap;

fn root(_req: HttpRequest) -> HttpResponse {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./src/templates")
        .unwrap();

    let mut data = BTreeMap::new();
    data.insert("paragraph_text", "This page was rendered using Actix-Web as the web server and handlebars as the templating language!");

    let body = handlebars.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
}