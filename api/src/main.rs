use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod handler;

/// `app.design` is intentionally stateless — it only ships static UI showcase
/// pages for `@liberte/svelte-components`. The API surface is kept as a stub
/// (one health endpoint) for symmetry with `app.smoke` and as a future
/// extension hook; business logic does not belong here.
#[derive(OpenApi)]
#[openapi(
    paths(handler::health::health),
    components(schemas(handler::health::Health))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(4330);

    let app = Router::new()
        .merge(handler::health::routes())
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.expect("bind failed");
    eprintln!("app.design.api listening on {addr}");
    axum::serve(listener, app).await.expect("serve error");
}
