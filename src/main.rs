use anyhow::Result;
use axum::{
    routing::{post,get},
    Router,
    Extension,
};
use axum_server::Server;

use opsx::{http, json::PayloadJson};
use std::{net::SocketAddr, sync::Arc};



use std::fs::File;
use std::io::Read;
use std::path::Path;


#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let path = Path::new("./cfg.json");
    let mut file = File::open(&path).expect("Unable to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let cfg: Vec<PayloadJson> = serde_json::from_str(&contents).expect("Unable to parse JSON");

    let app_cfg = Arc::new(cfg);

    // build our application with a route
    let app = Router::new()
        .route("/", get(http::index))
        .route("/dev", post(http::root))
        .layer(Extension(app_cfg)); 

    let addr = SocketAddr::from(([0, 0, 0, 0], 3177));

    tracing::debug!("listening on {}", addr);

    let refx = Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


    println!("refx: {:?}", refx);
    Ok(())
}
