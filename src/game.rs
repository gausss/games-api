use std::sync::Mutex;

use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use crate::game_repository::InMemoryGameRepository;
use crate::game_repository::GameRepository;

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub id: u8,
    pub name: String,
}

impl Game {
    pub fn new(id: u8, name: &str) -> Game {
        Game {
            id: id,
            name: name.to_string(),
        }
    }
}

// dyn GameRepository should be injected here but doesn't work yet
#[get("/games")]
pub async fn get_games(game_repository: Data<Mutex<InMemoryGameRepository>>) -> HttpResponse {
    let games = game_repository.lock().unwrap();
    HttpResponse::Ok().json(Json(games.get_all()))
}

#[get("/games/{game_id}")]
pub async fn get_game_by_id(
    path: Path<String>,
    game_repository: Data<Mutex<InMemoryGameRepository>>,
) -> HttpResponse {
    let id = path.into_inner().parse().unwrap();
    let games = game_repository.lock().unwrap();
    return match games.get(&id) {
        None => HttpResponse::NotFound().finish(),
        Some(game) => HttpResponse::Ok().json(game),
    };
}

#[put("/games")]
pub async fn update_game_by_id(
    game: Json<Game>,
    game_repository: Data<Mutex<InMemoryGameRepository>>,
) -> HttpResponse {
    let mut games = game_repository.lock().unwrap();
    return match games.save(game.0) {
        None => HttpResponse::Created().finish(),
        Some(_) => HttpResponse::Ok().finish(),
    };
}

#[delete("/games/{game_id}")]
pub async fn delete_game_by_id(
    path: Path<String>,
    game_repository: Data<Mutex<InMemoryGameRepository>>,
) -> HttpResponse {
    let id = path.into_inner().parse().unwrap();
    let mut games = game_repository.lock().unwrap();
    return match games.delete(&id) {
        None => HttpResponse::NotFound().finish(),
        Some(_) => HttpResponse::Ok().finish(),
    };
}
