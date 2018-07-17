use ecs::id::EntityId;
use ecs::Ecs;
use ecs::component::Position;

/// All possible interactions between `Component`s
pub enum EntityAction {
    Move(EntityId, (i32, i32)),
    Idle,
}

impl EntityAction {
    /// Execute the action
    pub fn execute(&self, ecs: &mut Ecs) {
        match *self {
            EntityAction::Move(entity_id, vel) => self.move_action(ecs, entity_id, vel),
            EntityAction::Idle => () // Idle - do nothing
        }
    }

    fn move_action(&self, ecs: &mut Ecs, entity_id: EntityId, vel: (i32, i32)) {
        match ecs.get_component_mut::<Position>(entity_id) {
            Some(c) => c.mv(vel),
            None => ()
        }
    }
}