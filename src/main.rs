#[macro_use(get, put, delete)]
extern crate actix_web;

use crate::game::Game;
use crate::store::in_memory::InMemoryStore;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::collections::HashMap;
use std::io::Result;
use std::sync::Mutex;

mod controller;
mod game;
mod store;

#[actix_web::main]
async fn main() -> Result<()> {
    let store = Data::new(Mutex::new(InMemoryStore::<Game>::init(HashMap::from([
        (1, Game::new(1, "Demon Souls")),
        (2, Game::new(2, "Dark Souls")),
        (3, Game::new(3, "Bloodborn")),
    ]))));

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .service(controller::get_games)
            .service(controller::get_game_by_id)
            .service(controller::update_game_by_id)
            .service(controller::delete_game_by_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
