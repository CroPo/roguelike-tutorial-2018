use ecs::id::EntityId;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Actor;
use ecs::component::Render;
use tcod::colors;
use ecs::component::MonsterAi;
use ecs::component::Corpse;
use render::RenderOrder;
use message::{Message, MessageLog};
use ecs::component::Name;
use std::rc::Rc;

/// This struct defines the Result of one single action. A message can be created, and also
/// a reaction can happen.
struct ActionResult {
    reaction: Option<EntityAction>,
    message: Option<Message>,
}

impl ActionResult {
    /// Return a `ActionResult` with all values being `None`
    pub fn none() -> ActionResult {
        ActionResult {
            reaction: None,
            message: None,
        }
    }
}

/// All possible interactions between `Component`s
#[derive(PartialEq)]
pub enum EntityAction {
    TakeDamage(EntityId, i32, EntityId),
    MoveTo(EntityId, (i32, i32)),
    MoveRelative(EntityId, (i32, i32)),
    Die(EntityId),
    Idle,
}

impl EntityAction {
    /// Execute the action
    pub fn execute(&self, ecs: &mut Ecs, log: Rc<MessageLog>) {
        let result = match *self {
            EntityAction::MoveTo(entity_id, pos) => self.move_to_action(ecs, entity_id, pos),
            EntityAction::MoveRelative(entity_id, delta) => self.move_relative_action(ecs, entity_id, delta),
            EntityAction::TakeDamage(entity_id, damage, attacker_entity_id)
            => self.take_damage_action(ecs, entity_id, damage, attacker_entity_id),
            EntityAction::Die(entity_id) => self.die_action(ecs, entity_id),
            EntityAction::Idle => ActionResult::none() // Idle - do nothing
        };

        if let Some(message) = result.message {
            log.add_message(message);
        }

        if let Some(reaction) = result.reaction {
            reaction.execute(ecs, log);
        }
    }

    fn move_to_action(&self, ecs: &mut Ecs, entity_id: EntityId, pos: (i32, i32)) -> ActionResult {
        if let Some(c) = ecs.get_component_mut::<Position>(entity_id) {
            c.move_absolute(pos)
        };
        ActionResult::none()
    }

    fn move_relative_action(&self, ecs: &mut Ecs, entity_id: EntityId, delta: (i32, i32)) -> ActionResult {
        if let Some(c) = ecs.get_component_mut::<Position>(entity_id) {
            c.move_relative(delta)
        };
        ActionResult::none()
    }

    fn take_damage_action(&self, ecs: &mut Ecs, entity_id: EntityId, damage: i32, attacker_entity_id: EntityId) -> ActionResult {

        let entity_name = EntityAction::get_entity_name(ecs, entity_id);
        let attacker_name = EntityAction::get_entity_name(ecs, attacker_entity_id).to_uppercase();

        if let Some(e) = ecs.get_component_mut::<Actor>(entity_id) {
            e.take_damage(damage);

            let message = Message::new(if damage > 0 {
                format!("The {} attacks {} for {} hit points.", attacker_name, entity_name, damage)
            } else {
                format!("The {} attacks {} but does no damage.", attacker_name, entity_name)
            });

            return if e.hp <= 0 {
                ActionResult {
                    reaction: Some(EntityAction::Die(entity_id)),
                    message: Some(message),
                }
            } else {
                ActionResult {
                    reaction: None,
                    message: Some(message),
                }
            }
        }
        ActionResult::none()
    }

    fn die_action(&self, ecs: &mut Ecs, entity_id: EntityId) -> ActionResult {

        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        let message = Message::new(if entity_id == ecs.player_entity_id {
            "YOU DIED".to_string()
        } else {
            format!("The {} died.", entity_name)
        });

        // Override the Rendering with the default corpse glyph
        ecs.register_component(entity_id, Render::new(entity_id, '%', colors::DARK_CRIMSON, RenderOrder::Corpse));
        // Remove the AI and the Creature components
        ecs.remove_component::<MonsterAi>(entity_id);
        // Add the Corpse component
        ecs.register_component(entity_id, Corpse {});
        // Set non blocking
        match ecs.get_component_mut::<Position>(entity_id) {
            Some(p) => p.is_blocking = false,
            None => ()
        }

        ActionResult {
            reaction: None,
            message: Some(message),
        }
    }

    fn get_entity_name(ecs: &Ecs, id: EntityId) -> String {
        match ecs.get_component::<Name>(id) {
            Some(n) => n.name.clone(),
            None => format!("a nameless entity (#{})", id)
        }
    }
}