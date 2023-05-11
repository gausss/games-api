#[macro_use(get, put, delete)]
extern crate actix_web;

use std::collections::HashMap;
use std::io::Result;
use std::sync::Mutex;

use actix_web::web::Data;
use actix_web::{App, HttpServer};

pub use game::Game;
pub use crate::game_repository::InMemoryGameRepository;

pub mod game;
mod game_repository;

#[actix_web::main]
async fn main() -> Result<()> {
    let store = Data::new(Mutex::new(InMemoryGameRepository::init(HashMap::from([
        (1, Game::new(1, "Demon Souls")),
        (2, Game::new(2, "Dark Souls")),
        (3, Game::new(3, "Bloodborn")),
    ]))));

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .service(game::get_games)
            .service(game::get_game_by_id)
            .service(game::update_game_by_id)
            .service(game::delete_game_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
