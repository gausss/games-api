use std::collections::HashMap;

use crate::Game;

pub trait GameRepository {
    fn save(&mut self, value: Game) -> Option<Game>;
    fn delete(&mut self, id: &u8) -> Option<Game>;
    fn get(&self, id: &u8) -> Option<&Game>;
    fn get_all(&self) -> Vec<&Game>;
    fn len(&self) -> usize;
}

#[derive(Clone)]
pub struct InMemoryGameRepository {
    data: HashMap<u8, Game>,
}

impl InMemoryGameRepository {
    pub fn new() -> InMemoryGameRepository {
        InMemoryGameRepository {
            data: HashMap::new(),
        }
    }

    pub fn init(init_data: HashMap<u8, Game>) -> InMemoryGameRepository {
        InMemoryGameRepository { data: init_data }
    }
}

impl GameRepository for InMemoryGameRepository {
    fn save(&mut self, value: Game) -> Option<Game> {
        self.data.insert(value.id, value)
    }

    fn delete(&mut self, id: &u8) -> Option<Game> {
        self.data.remove(id)
    }

    fn get(&self, id: &u8) -> Option<&Game> {
        self.data.get(id)
    }

    fn get_all(&self) -> Vec<&Game> {
        self.data.values().collect()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod test {
    use super::GameRepository;
    use super::InMemoryGameRepository;
    use crate::game::Game;
    use std::collections::HashMap;

    #[test]
    fn test_save_game() {
        let mut store: InMemoryGameRepository = InMemoryGameRepository::new();
        assert_eq!(store.len(), 0);

        let age_of_empires = Game::new(1, "Age of Empires");
        store.save(age_of_empires);

        assert_eq!(store.len(), 1);
        assert_eq!(store.get(&1).unwrap().name, "Age of Empires");
    }

    #[test]
    fn test_delete_game() {
        let mut store: InMemoryGameRepository =
            InMemoryGameRepository::init(HashMap::from([(1, Game::new(1, "Age of Empires"))]));
        assert_eq!(store.len(), 1);

        store.delete(&1);
        assert_eq!(store.len(), 0);
    }
}
