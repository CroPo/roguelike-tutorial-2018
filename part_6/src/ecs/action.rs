use ecs::id::EntityId;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Creature;

/// All possible interactions between `Component`s
pub enum EntityAction {
    TakeDamage(EntityId, i32),
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
            EntityAction::TakeDamage(entity_id, damage) => self.take_damage_action(ecs, entity_id, damage),
            EntityAction::Idle => () // Idle - do nothing
        }
    }

    fn move_to_action(&self, ecs: &mut Ecs, entity_id: EntityId, pos: (i32, i32)) {
        if let Some(c) = ecs.get_component_mut::<Position>(entity_id) {
            c.move_absolute(pos)
        }
    }

    fn move_relative_action(&self, ecs: &mut Ecs, entity_id: EntityId, delta: (i32, i32)) {
        if let Some(c) = ecs.get_component_mut::<Position>(entity_id) {
            c.move_relative(delta)
        }
    }

    fn take_damage_action(&self, ecs: &mut Ecs, entity_id: EntityId, damage: i32) {
        if let Some(e) = ecs.get_component_mut::<Creature>(entity_id) {
            e.take_damage(damage)
        }
    }
}