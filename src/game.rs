use crate::store::Identified;
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub id: i8,
    pub name: String,
    pub played: bool,
}

impl Game {
    pub fn new(id: i8, name: &str) -> Game {
        Game {
            id: id,
            name: name.to_string(),
            played: false,
        }
    }

    fn mark_played(&mut self) {
        self.played = true;
    }
}

impl Identified for Game {
    fn get_id(&self) -> i8 {
        self.id
    }
}

#[cfg(test)]
mod test {
    use crate::game::Game;
    use crate::store::Identified;

    #[test]
    fn test_game() {
        let game = Game::new(1, "Age of Empires");
        assert_eq!(game.played, false);
        assert_eq!(game.name, "Age of Empires");
        assert_eq!(game.get_id(), 1);

        let mut game = game;
        game.mark_played();
        assert_eq!(game.played, true);
    }
}
