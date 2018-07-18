use ecs::id::EntityId;
use ecs::Ecs;
use ecs::component::Position;

/// All possible interactions between `Component`s
pub enum EntityAction {
    MoveTo(EntityId, (i32, i32)),
    MoveRelative(EntityId, (i32, i32)),
    Idle,
}

impl EntityAction {
    /// Execute the action
    pub fn execute(&self, ecs: &mut Ecs) {
        match *self {
            EntityAction::MoveTo(entity_id, pos) => self.move_to_action(ecs, entity_id, pos),
            EntityAction::MoveRelative(entity_id, delta) => self.move_relative_action(ecs, entity_id, delta),
            EntityAction::Idle => () // Idle - do nothing

        }
    }

    fn move_to_action(&self, ecs: &mut Ecs, entity_id: EntityId, pos: (i32, i32)) {
        match ecs.get_component_mut::<Position>(entity_id) {
            Some(c) => c.move_absolute(pos),
            None => ()
        }
    }
    fn move_relative_action(&self, ecs: &mut Ecs, entity_id: EntityId, pos: (i32, i32)) {
        match ecs.get_component_mut::<Position>(entity_id) {
            Some(c) => c.move_relative(pos),
            None => ()
        }
    }
}