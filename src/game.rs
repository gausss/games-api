use std::sync::Mutex;

use crate::{
    store::{Identified, Store},
    InMemoryStore,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

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

impl Identified for Game {
    fn get_id(&self) -> u8 {
        self.id
    }
}

// InMemoryStore<Game> should be dyn Store<Game>, but it doesn't work at the moment.
#[get("/games")]
pub async fn get_games(game_store: Data<Mutex<InMemoryStore<Game>>>) -> HttpResponse {
    let store = game_store.lock().unwrap();
    HttpResponse::Ok().json(Json(store.get_all()))
}

#[get("/games/{game_id}")]
pub async fn get_game_by_id(
    path: Path<String>,
    game_store: Data<Mutex<InMemoryStore<Game>>>,
) -> HttpResponse {
    let id = path.into_inner().parse().unwrap();
    let store = game_store.lock().unwrap();
    return match store.get(&id) {
        None => HttpResponse::NotFound().finish(),
        Some(game) => HttpResponse::Ok().json(game),
    };
}

#[put("/games")]
pub async fn update_game_by_id(
    game: Json<Game>,
    game_store: Data<Mutex<InMemoryStore<Game>>>,
) -> HttpResponse {
    let mut store = game_store.lock().unwrap();
    return match store.save(game.0) {
        None => HttpResponse::Created().finish(),
        Some(_) => HttpResponse::Ok().finish(),
    };
}

#[delete("/games/{game_id}")]
pub async fn delete_game_by_id(
    path: Path<String>,
    game_store: Data<Mutex<InMemoryStore<Game>>>,
) -> HttpResponse {
    let id = path.into_inner().parse().unwrap();
    let mut store = game_store.lock().unwrap();
    return match store.delete(&id) {
        None => HttpResponse::NotFound().finish(),
        Some(_) => HttpResponse::Ok().finish(),
    };
}
