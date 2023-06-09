mod router;
mod routes;
use router::create_router;

pub async fn run() {
    let app = create_router();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
