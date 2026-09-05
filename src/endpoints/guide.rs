use crate::args::{Args, ARGS};
use actix_web::{get, web, HttpResponse};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "guide.html")]
struct Guide<'a> {
    args: &'a Args,
}

#[get("/guide")]
pub async fn guide(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let topic = query.get("topic").cloned().unwrap_or_default();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "{}<p class=\"guide-topic\">Showing topic: {}</p>",
            Guide { args: &ARGS }.render().unwrap(),
            topic
        ))
}
