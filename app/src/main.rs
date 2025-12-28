use askama::Template;
use warp::Filter;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| {
        let template = IndexTemplate {
            title: "cwrdd".to_string(),
            message: "Hello from cwrdd!".to_string(),
        };
        warp::reply::html(template.render().unwrap())
    });

    let greeting = warp::path!("api" / "greeting").map(|| {
        warp::reply::html("<p>👋 Hello from the server! This was fetched with htmx.</p>")
    });

    let routes = index.or(greeting);

    println!("🚀 Server running at https://0.0.0.0:8443");
    warp::serve(routes)
        .tls()
        .cert_path("/app/certs/cert.pem")
        .key_path("/app/certs/key.pem")
        .run(([0, 0, 0, 0], 8443))
        .await;
}
