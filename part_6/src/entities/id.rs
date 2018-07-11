/// A unique type for `Entity` IDs. This is just here to make sure it's easy to change
/// to another actual primitive type if necessary.
pub type EntityId = u8;

/// Generator for IDs which are used to identify an `Entity`
struct IdGenerator {
    id: EntityId
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator {
            id: 0,
        }
    }

    /// Generate a new ID
    fn get_next_id(&mut self) -> EntityId {
        self.id+=1;
        self.id
    }
}