use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod engine;
use engine::Engine;

#[derive(Deserialize)]
struct AnalyzeRequest {
    fen: String,
    depth: Option<u8>,
}

#[derive(Serialize)]
struct AnalyzeResponse {
    best_move: Option<String>,
    error: Option<String>,
}

use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Chess AI Server at http://127.0.0.1:8081");

    HttpServer::new(|| {
        let cors = Cors::permissive(); // For development, allow all

        App::new()
            .wrap(cors)
            .route("/analyze", web::post().to(analyze))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

async fn analyze(req: web::Json<AnalyzeRequest>) -> impl Responder {
    let depth = req.depth.unwrap_or(3);
    match Engine::get_best_move(&req.fen, depth) {
        Some(m) => HttpResponse::Ok().json(AnalyzeResponse {
            best_move: Some(m),
            error: None,
        }),
        None => HttpResponse::Ok().json(AnalyzeResponse {
            best_move: None,
            error: Some("Could not find best move or invalid FEN".to_string()),
        }),
    }
}
