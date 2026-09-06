use crate::args::{Args, ARGS};
use actix_web::{get, web, HttpResponse};
use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "guide.html")]
struct Guide<'a> {
    args: &'a Args,
}

fn escape_topic(topic: &str) -> String {
    html_escape::encode_text(topic).to_string()
}

#[get("/guide")]
pub async fn guide(query: web::Query<HashMap<String, String>>) -> HttpResponse {
    let topic = escape_topic(query.get("topic").map(String::as_str).unwrap_or_default());

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            "{}<p class=\"guide-topic\">Showing topic: {}</p>",
            Guide { args: &ARGS }.render().unwrap(),
            topic
        ))
}

#[cfg(test)]
mod tests {
    use super::escape_topic;

    #[test]
    fn guide_topic_is_escaped_for_html() {
        assert_eq!(
            escape_topic("<script>alert(1)</script>"),
            "&lt;script&gt;alert(1)&lt;/script&gt;"
        );
    }
}
