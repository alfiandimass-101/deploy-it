use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crabchess::prelude::*;
use crabchess::squares::{File, Rank, Square};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MoveRequest {
    history: Vec<String>,        // SAN history
    user_move: Option<UserMove>, // Optional, if just fetching state
}

#[derive(Deserialize)]
struct UserMove {
    from: String,              // e.g. "e2"
    to: String,                // e.g. "e4"
    promotion: Option<String>, // e.g. "q"
}

#[derive(Serialize)]
struct StateResponse {
    fen: String,
    history: Vec<String>,
    valid_moves: Vec<String>, // "e2e4" format
    status: String,
    turn: String,
    error: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Chess Web Server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/api/move", web::post().to(handle_move))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn handle_move(req: web::Json<MoveRequest>) -> impl Responder {
    let mut position = ChessPosition::new();

    // Replay history
    let move_strs: Vec<&str> = req.history.iter().map(|s| s.as_str()).collect();
    if let Err(_) = position.apply_sans(move_strs.into_iter()) {
        return HttpResponse::BadRequest().json(StateResponse {
            fen: "".to_string(),
            history: req.history.clone(),
            valid_moves: vec![],
            status: "Error".to_string(),
            turn: "".to_string(),
            error: Some("Invalid history".to_string()),
        });
    }

    let mut new_history = req.history.clone();

    // Apply user move if present
    if let Some(um) = &req.user_move {
        // Find the move in legal moves
        let legal_moves = generate_legal_moves(&position);
        let mut found_move = None;

        // We need to parse from/to strings to Squares
        // Assuming "e2" -> File::E, Rank::Two
        // We need a helper for this.

        for m in legal_moves {
            // Check if m matches um.from and um.to
            // We need to get start/end squares from Move.
            // Move enum variants have fields.
            // We can implement a helper to get start/end.
            if move_matches(&m, &um.from, &um.to) {
                found_move = Some(m);
                break;
            }
        }

        if let Some(m) = found_move {
            // Apply move
            // We need the SAN representation to add to history.
            // crabchess might not give SAN easily from Move without context?
            // `position.apply_move(m)` updates position.
            // `position.last_move_san()`? (Not in snippet)
            // But `position.sans()` returns all SANs.
            // So if we apply move, then call `position.sans()`, we get the new list!

            if position.apply_move(m).is_ok() {
                // Get updated history
                if let Ok(sans) = position.sans() {
                    new_history = sans.iter().map(|s| s.to_string()).collect();
                }
            } else {
                return HttpResponse::BadRequest().json(StateResponse {
                    fen: position.fen(),
                    history: req.history.clone(),
                    valid_moves: vec![],
                    status: "Error".to_string(),
                    turn: "".to_string(),
                    error: Some("Failed to apply move".to_string()),
                });
            }
        } else {
            return HttpResponse::BadRequest().json(StateResponse {
                fen: position.fen(),
                history: req.history.clone(),
                valid_moves: vec![],
                status: "Error".to_string(),
                turn: "".to_string(),
                error: Some("Illegal move".to_string()),
            });
        }
    }

    // Generate valid moves for the *new* position
    let legal_moves = generate_legal_moves(&position);
    let valid_move_strings = legal_moves.iter().map(|m| format_move(m)).collect();

    let fen = position.fen();
    let turn_str = fen.split_whitespace().nth(1).unwrap_or("w");
    let turn = if turn_str == "w" { "White" } else { "Black" };

    HttpResponse::Ok().json(StateResponse {
        fen,
        history: new_history,
        valid_moves: valid_move_strings,
        status: format!("{:?}", position.status()),
        turn: turn.to_string(),
        error: None,
    })
}

fn generate_legal_moves(position: &ChessPosition) -> Vec<Move> {
    let mut moves = Vec::new();
    let files = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];
    let ranks = [
        Rank::One,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
    ];

    let fen = position.fen();
    let turn_str = fen.split_whitespace().nth(1).unwrap_or("w");
    let turn = if turn_str == "w" {
        Color::White
    } else {
        Color::Black
    };

    for f in files {
        for r in ranks {
            let sq = Square(f, r);
            if let Some(piece) = position.get(sq) {
                if piece.color == turn {
                    let candidates = position.pseudolegally_navigable(sq, turn);
                    for m in candidates {
                        let mut p_clone = position.clone();
                        if p_clone.apply_move(m).is_ok() {
                            moves.push(m);
                        }
                    }
                }
            }
        }
    }
    moves
}

fn move_matches(m: &Move, from: &str, to: &str) -> bool {
    // Helper to check if move matches coordinates
    // We need to extract start/end from Move
    // Move::Standard { initial_square, final_square, ... }
    // Move::Castle { ... } -> King start/end?
    // Move::EnPassant? (Not in snippet, maybe Standard covers it?)

    let (start, end) = match m {
        Move::Standard {
            initial_square,
            final_square,
            ..
        } => (*initial_square, *final_square),
        Move::Castle { side, color, .. } => {
            // Determine start/end for castling
            // White Kingside: e1 -> g1
            // White Queenside: e1 -> c1
            // Black Kingside: e8 -> g8
            // Black Queenside: e8 -> c8
            let rank = if *color == Color::White {
                Rank::One
            } else {
                Rank::Eight
            };
            let start = Square(File::E, rank);
            let end = match side {
                Side::Kingside => Square(File::G, rank),
                Side::Queenside => Square(File::C, rank),
            };
            (start, end)
        }
        _ => return false, // Handle other types if any
    };

    // Convert Square to string "e2"
    let start_str = square_to_str(start);
    let end_str = square_to_str(end);

    start_str == from && end_str == to
}

fn format_move(m: &Move) -> String {
    // Format as "e2e4"
    let (start, end) = match m {
        Move::Standard {
            initial_square,
            final_square,
            ..
        } => (*initial_square, *final_square),
        Move::Castle { side, color, .. } => {
            let rank = if *color == Color::White {
                Rank::One
            } else {
                Rank::Eight
            };
            let start = Square(File::E, rank);
            let end = match side {
                Side::Kingside => Square(File::G, rank),
                Side::Queenside => Square(File::C, rank),
            };
            (start, end)
        }
        _ => return "0000".to_string(),
    };
    format!("{}{}", square_to_str(start), square_to_str(end))
}

fn square_to_str(sq: Square) -> String {
    // Square(File, Rank)
    let file_char = match sq.0 {
        File::A => 'a',
        File::B => 'b',
        File::C => 'c',
        File::D => 'd',
        File::E => 'e',
        File::F => 'f',
        File::G => 'g',
        File::H => 'h',
    };
    let rank_char = match sq.1 {
        Rank::One => '1',
        Rank::Two => '2',
        Rank::Three => '3',
        Rank::Four => '4',
        Rank::Five => '5',
        Rank::Six => '6',
        Rank::Seven => '7',
        Rank::Eight => '8',
    };
    format!("{}{}", file_char, rank_char)
}
