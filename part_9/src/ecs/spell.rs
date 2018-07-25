use ecs::id::EntityId;
use ecs::Ecs;
use message::Message;
use ecs::component::Actor;
use ecs::component::Name;

use tcod::colors;

pub struct SpellResult {
    pub message: Option<Message>,
    pub status: SpellStatus,
}

impl SpellResult {
    fn success(message: Option<Message>) -> SpellResult {
        SpellResult {
            message,
            status: SpellStatus::Success,
        }
    }

    fn fail(message: Option<Message>) -> SpellResult {
        SpellResult {
            message,
            status: SpellStatus::Fail,
        }
    }
}


pub enum SpellStatus {
    Success,
    Fail,
}

#[derive(Clone)]
pub enum Spell {
    Heal
}

impl Spell {
    pub fn cast(&self, ecs: &mut Ecs, caster_id: EntityId) -> SpellResult {
        match *self {
            Spell::Heal => self.heal(ecs, caster_id)
        }
    }

    fn heal(&self, ecs: &mut Ecs, caster_id: EntityId) -> SpellResult {
        let entity_name = Self::get_entity_name(ecs, caster_id);

        if let Some(actor) = ecs.get_component_mut::<Actor>(caster_id) {
            if actor.hp == actor.max_hp {
                SpellResult::fail(Some(Message::new(format!("{} is already at full health", entity_name), colors::YELLOW)))
            } else {
                actor.hp = actor.max_hp;
                SpellResult::success(Some(Message::new(format!("{} was fully healed", entity_name), colors::GREEN)))
            }
        } else {
            SpellResult::fail(None)
        }
    }

    fn get_entity_name(ecs: &Ecs, id: EntityId) -> String {
        match ecs.get_component::<Name>(id) {
            Some(n) => n.name.clone(),
            None => format!("a nameless entity (#{})", id)
        }
    }
}
