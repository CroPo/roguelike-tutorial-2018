use ecs::id::EntityId;
use ecs::Ecs;
use message::Message;
use ecs::component::Actor;
use ecs::component::Name;

use tcod::colors;
use ecs::component::Position;
use tcod::Map;
use ecs::action::EntityAction;

pub struct SpellResult {
    pub message: Option<Message>,
    pub status: SpellStatus,
    pub reaction: Option<EntityAction>
}

impl SpellResult {
    fn success(message: Option<Message>, reaction: Option<EntityAction>) -> SpellResult {
        SpellResult {
            message,
            status: SpellStatus::Success,
            reaction
        }
    }

    fn fail(message: Option<Message>) -> SpellResult {
        SpellResult {
            message,
            status: SpellStatus::Fail,
            reaction: None
        }
    }
}


pub enum SpellStatus {
    Success,
    Fail,
}

#[derive(Clone)]
pub enum Spell {
    Heal,
    Lightning(u8, u32),
}

impl Spell {
    pub fn cast(&self, ecs: &mut Ecs, fov_map: &Map, caster_id: EntityId) -> SpellResult {
        match *self {
            Spell::Heal => self.heal(ecs, caster_id),
            Spell::Lightning(range, damage) => self.lightning(ecs, fov_map, caster_id, range, damage)
        }
    }

    fn heal(&self, ecs: &mut Ecs, caster_id: EntityId) -> SpellResult {
        let entity_name = Self::get_entity_name(ecs, caster_id);

        if let Some(actor) = ecs.get_component_mut::<Actor>(caster_id) {
            if actor.hp == actor.max_hp {
                SpellResult::fail(Some(Message::new(format!("{} is already at full health", entity_name), colors::YELLOW)))
            } else {
                actor.hp = actor.max_hp;
                SpellResult::success(Some(Message::new(format!("{} was fully healed", entity_name), colors::GREEN)), None)
            }
        } else {
            SpellResult::fail(None)
        }
    }

    fn lightning(&self, ecs: &mut Ecs, fov_map: &Map, caster_id: EntityId, range: u8, damage: u32) -> SpellResult {
        let entity_name = Self::get_entity_name(ecs, caster_id);

        let target = if let Some(caster_position) = ecs.get_component::<Position>(caster_id) {
            match self.find_target(ecs, fov_map, &caster_position) {
                Some((entity_id, distance)) => {
                    if distance <= range {
                        Some(entity_id)
                    } else {
                        None
                    }
                }
                None => None
            }
        } else { None };

        if let Some(target_id) = target {
            let target_name = Self::get_entity_name(ecs, target_id).to_uppercase();

            /// We can unwrap this right at the place, because we already made sure that only `Actor` entities will be used
            let target = ecs.get_component::<Actor>(target_id).unwrap();
            let message= Message::new(format!("A lighting bolt strikes the {} with a loud thunder!", target_name),
                                      colors::LIGHT_BLUE);
            SpellResult::success(Some(message), Some(EntityAction::TakeDamage(target_id, damage)))
        } else {
            SpellResult::fail(Some(Message::new("No valid target in sight and in range".to_string(), colors::RED)))
        }
    }

    fn find_target(&self, ecs: &Ecs, fov_map: &Map, caster: &Position) -> Option<(EntityId, u8)> {
        let mut distances: Vec<(u8, u8)> = ecs.get_all::<Position>().iter().filter(|(id, p)| {
            **id != caster.entity_id
                && fov_map.is_in_fov(p.position.0, p.position.1)
                && ecs.has_component::<Actor>(**id)
        }).map(|(id, p)| {
            (*id, caster.distance_to(p.position) as u8)
        }).collect();

        distances.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        if let Some(d) = distances.first() {
            Some(d.clone())
        } else {
            None
        }
    }

    fn get_entity_name(ecs: &Ecs, id: EntityId) -> String {
        match ecs.get_component::<Name>(id) {
            Some(n) => n.name.clone(),
            None => format!("a nameless entity (#{})", id)
        }
    }
}
