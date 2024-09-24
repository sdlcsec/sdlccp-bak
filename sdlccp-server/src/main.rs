use std::net::{Ipv4Addr, SocketAddr};

use axum::Router;
use sdlc_cp_api::services::controlplane;
use tokio::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let openapi = controlplane::ControlPlaneAPIDoc::openapi();
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        //.merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        //.merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // Alternative to above
        // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        //.merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/api/v1alpha1/namespaces", controlplane::namespace_router());

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, app.into_make_service()).await
}