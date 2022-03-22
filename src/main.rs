#![forbid(unsafe_code)]
use axum::{
    http::{
        // StatusCode,
        Method,
        Request,
        Response,
        uri::Uri,
        HeaderValue
    },
    // body::{Body},
    response::{IntoResponse, Redirect},
    routing::{any},
    Router,
    extract::{Extension, Path, Query},
};
use tower_http::cors::{CorsLayer, Origin};
use hyper::{client::{Client, HttpConnector}, Body};
use hyper_tls::HttpsConnector;
use serde_json::{Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // let clinet = Client::new(); // only support http
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/redirect/*__origin__", any(redirect_handler))
        .route("/proxy/*__origin__", any(proxy_handler))
        .layer(Extension(client))
        .layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            CorsLayer::new()
                .allow_origin(Origin::exact("*".parse().unwrap()))
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn redirect_handler(Path(origin): Path<String>, Query(_params): Query<Value>, req: Request<Body>) -> impl IntoResponse {
    // println!("get_handler called {:?}, req: {:?}", params, req);
    let origin = origin.strip_prefix("/").unwrap();
    let target_uri = get_full_url(&origin, &req);
    Redirect::temporary(target_uri)
    // (StatusCode::OK, origin)
}

// https://github.com/tokio-rs/axum/blob/main/examples/reverse-proxy/src/main.rs
async fn proxy_handler(Path(origin): Path<String>, Extension(client): Extension<Client<HttpsConnector<HttpConnector>, Body>>, mut req: Request<Body>) -> Response<Body> {
    println!("get_handler called, req: {:?}", req);
    let origin = origin.strip_prefix("/").unwrap();
    let target_uri = get_full_url(&origin, &req);
    // println!("uri: {:?}", target_uri);
    *req.uri_mut() = target_uri.clone();
    // 自定义追加header
    req.headers_mut().insert("HOST", HeaderValue::from_str(target_uri.host().unwrap()).unwrap());
    client.request(req).await.unwrap()
}

fn get_full_url(origin: &str, req: &Request<Body>) -> Uri {
    let cur_uri = req.uri();
    let mut full_url = origin.to_owned();
    if let Some(raw_query) = cur_uri.query() {
        full_url.push_str("?");
        full_url.push_str(raw_query);
    }
    full_url.parse().unwrap()
}

