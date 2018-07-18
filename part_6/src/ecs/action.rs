use ecs::id::EntityId;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Creature;
use ecs::component::Render;
use tcod::colors;
use ecs::component::MonsterAi;
use ecs::component::Corpse;

/// All possible interactions between `Component`s
#[derive(PartialEq)]
pub enum EntityAction {
    TakeDamage(EntityId, i32),
    MoveTo(EntityId, (i32, i32)),
    MoveRelative(EntityId, (i32, i32)),
    Die(EntityId),
    Idle,
}

impl EntityAction {
    /// Execute the action
    pub fn execute(&self, ecs: &mut Ecs) {
        let reaction = match *self {
            EntityAction::MoveTo(entity_id, pos) => self.move_to_action(ecs, entity_id, pos),
            EntityAction::MoveRelative(entity_id, delta) => self.move_relative_action(ecs, entity_id, delta),
            EntityAction::TakeDamage(entity_id, damage) => self.take_damage_action(ecs, entity_id, damage),
            EntityAction::Die(entity_id) => self.die_action(ecs, entity_id),
            EntityAction::Idle => EntityAction::Idle // Idle - do nothing
        };

        if reaction != EntityAction::Idle {
            reaction.execute(ecs);
        }

    }

    fn move_to_action(&self, ecs: &mut Ecs, entity_id: EntityId, pos: (i32, i32)) -> EntityAction {
        if let Some(c) = ecs.get_component_mut::<Position>(entity_id) {
            c.move_absolute(pos)
        };
        EntityAction::Idle
    }

    fn move_relative_action(&self, ecs: &mut Ecs, entity_id: EntityId, delta: (i32, i32)) -> EntityAction {
        if let Some(c) = ecs.get_component_mut::<Position>(entity_id) {
            c.move_relative(delta)
        };
        EntityAction::Idle
    }

    fn take_damage_action(&self, ecs: &mut Ecs, entity_id: EntityId, damage: i32) -> EntityAction {
        if let Some(e) = ecs.get_component_mut::<Creature>(entity_id) {
            e.take_damage(damage);

            if e.hp <= 0 {
                return EntityAction::Die(entity_id)
            }
        }
        EntityAction::Idle
    }

    fn die_action(&self, ecs : &mut Ecs, entity_id: EntityId) -> EntityAction {
        // Override the Rendering with the default corpse glyph
        ecs.register_component(entity_id, Render::new('%', colors::DARK_CRIMSON));
        // Remove the AI and the Creature components
        ecs.remove_component::<MonsterAi>(entity_id);
        ecs.remove_component::<Creature>(entity_id);
        // Add the Corpse component
        ecs.register_component(entity_id, Corpse{});
        // Set non blocking
        match ecs.get_component_mut::<Position>(entity_id) {
            Some(p) => p.is_blocking = false,
            None => ()
        }

        EntityAction::Idle
    }
}