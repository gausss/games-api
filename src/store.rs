pub trait Store<T>
where
    T: Identified,
{
    fn save(&mut self, value: T) -> Option<T>;
    fn delete(&mut self, id: &i8) -> Option<T>;
    fn get(&self, id: &i8) -> Option<&T>;
    fn get_all(&self) -> Vec<&T>;
    fn len(&self) -> usize;
}

pub trait Identified {
    fn get_id(&self) -> i8;
}

pub mod in_memory {
    use crate::store::{Identified, Store};
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct InMemoryStore<T> {
        data: HashMap<i8, T>,
    }

    impl<T> InMemoryStore<T> {
        pub fn new() -> InMemoryStore<T> {
            InMemoryStore {
                data: HashMap::new(),
            }
        }

        pub fn init(init_data: HashMap<i8, T>) -> InMemoryStore<T> {
            InMemoryStore { data: init_data }
        }
    }

    impl<T> Store<T> for InMemoryStore<T>
    where
        T: Identified,
    {
        fn save(&mut self, value: T) -> Option<T> {
            self.data.insert(value.get_id(), value)
        }

        fn delete(&mut self, id: &i8) -> Option<T> {
            self.data.remove(id)
        }

        fn get(&self, id: &i8) -> Option<&T> {
            self.data.get(id)
        }

        fn get_all(&self) -> Vec<&T> {
            return self.data.values().collect();
        }

        fn len(&self) -> usize {
            self.data.len()
        }
    }

    #[cfg(test)]
    mod test {
        use crate::game::Game;
        use crate::store::in_memory::InMemoryStore;
        use crate::store::Store;
        use std::collections::HashMap;

        #[test]
        fn test_save_game() {
            let mut store: InMemoryStore<Game> = InMemoryStore::new();
            assert_eq!(store.len(), 0);

            let age_of_empires = Game::new(1, "Age of Empires");
            store.save(age_of_empires);

            assert_eq!(store.len(), 1);
            assert_eq!(store.get(&1).unwrap().name, "Age of Empires");
        }

        #[test]
        fn test_delete_game() {
            let mut store: InMemoryStore<Game> =
                InMemoryStore::init(HashMap::from([(1, Game::new(1, "Age of Empires"))]));
            assert_eq!(store.len(), 1);

            store.delete(&1);
            assert_eq!(store.len(), 0);
        }
    }
}
