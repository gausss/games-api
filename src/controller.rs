use crate::game::Game;
use crate::store::in_memory::InMemoryStore;
use crate::store::Store;
use actix_web::web::{Data, Json, Path};
use actix_web::HttpResponse;
use std::sync::Mutex;

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
    return match store.save(&game.id, &game.0) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{test, App};
    use std::collections::HashMap;

    #[actix_web::test]
    async fn test_get_games() {
        let test_store = Data::new(Mutex::new(InMemoryStore::init(HashMap::from([
            (1, Game::new(1, "Demon Souls")),
            (2, Game::new(2, "Age of Empires")),
        ]))));
        let app =
            test::init_service(App::new().app_data(test_store.clone()).service(get_games)).await;
        let req = test::TestRequest::get().uri("/games").to_request();

        let resp: Vec<Game> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.len(), 2);
        assert_eq!(resp.contains(&Game::new(2, "Age of Empires")), true);
    }

    #[actix_web::test]
    async fn test_get_game_by_id() {
        let test_store = Data::new(Mutex::new(InMemoryStore::init(HashMap::from([(
            1,
            Game::new(1, "Demon Souls"),
        )]))));
        let app = test::init_service(
            App::new()
                .app_data(test_store.clone())
                .service(get_game_by_id),
        )
        .await;
        let req = test::TestRequest::get().uri("/games/1").to_request();

        let resp: Game = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.name, "Demon Souls");
    }

    #[actix_web::test]
    async fn test_delete_game_by_id() {
        let test_store = Data::new(Mutex::new(InMemoryStore::init(HashMap::from([(
            1,
            Game::new(1, "Demon Souls"),
        )]))));
        let app = test::init_service(
            App::new()
                .app_data(test_store.clone())
                .service(delete_game_by_id),
        )
        .await;
        let req = test::TestRequest::delete().uri("/games/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::delete().uri("/games/3").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_add_game() {
        let test_store = Data::new(Mutex::new(InMemoryStore::init(HashMap::from([(
            1,
            Game::new(1, "Demon Souls"),
        )]))));
        let app = test::init_service(
            App::new()
                .app_data(test_store.clone())
                .service(get_games)
                .service(update_game_by_id),
        )
        .await;

        let req = test::TestRequest::get().uri("/games").to_request();
        let resp: Vec<Game> = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.len(), 1);

        let req = test::TestRequest::put()
            .uri("/games")
            .set_json(Game::new(2, "Age of Empires"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let req = test::TestRequest::put()
            .uri("/games")
            .set_json(Game::new(1, "Dark Souls"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let req = test::TestRequest::get().uri("/games").to_request();
        let resp: Vec<Game> = test::call_and_read_body_json(&app, req).await;
        assert!(resp.contains(&Game::new(1, "Dark Souls")));
    }
}
