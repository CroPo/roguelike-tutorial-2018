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
    pub reactions: Vec<EntityAction>,
}

impl SpellResult {
    fn success(caster_id: EntityId, item_id: EntityId, message: Option<Message>, reaction: Option<EntityAction>) -> SpellResult {
        let reactions = if let Some(action) = reaction {
            vec![EntityAction::RemoveItemFromInventory(caster_id, item_id), action]
        } else {
            vec![EntityAction::RemoveItemFromInventory(caster_id, item_id)]
        };

        SpellResult {
            message,
            status: SpellStatus::Success,
            reactions,
        }
    }

    fn fail(message: Option<Message>) -> SpellResult {
        SpellResult {
            message,
            status: SpellStatus::Fail,
            reactions: vec![],
        }
    }

    fn targeting(spell: Spell, caster_id: EntityId) -> SpellResult {
        SpellResult {
            message: Some(Message::new("Select a target by clicking on it, or cancel with ESC".to_string(), colors::WHITE)),
            status: SpellStatus::Targeting(spell, caster_id),
            reactions: vec![],
        }
    }

    fn add_reaction(&mut self, reaction: EntityAction) {
        self.reactions.push(reaction)
    }
}


pub enum SpellStatus {
    Success,
    Targeting(Spell, EntityId),
    Fail,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Spell {
    Heal(EntityId),
    Lightning(EntityId, u8, u32),
    Fireball(EntityId, u8, u32),
}

impl Spell {
    pub fn cast(&self, ecs: &mut Ecs, fov_map: &Map, caster_id: EntityId) -> SpellResult {
        match *self {
            Spell::Heal(item_id) => self.heal(ecs, caster_id, item_id),
            Spell::Lightning(item_id, range, damage) => self.lightning(ecs, fov_map, caster_id, item_id, range, damage),
            Spell::Fireball(item_id, radius, damage) => self.fireball(ecs, caster_id)
        }
    }

    pub fn cast_on_target(&self, ecs: &mut Ecs, target_id: EntityId, caster_id: EntityId) -> SpellResult {
        match *self {
            Spell::Fireball(item_id, radius, damage) => self.fireball_on_target(ecs, target_id, caster_id, item_id, radius, damage),
            _ => SpellResult::fail(None)
        }
    }

    fn heal(&self, ecs: &mut Ecs, caster_id: EntityId, item_id: EntityId) -> SpellResult {
        let entity_name = Self::get_entity_name(ecs, caster_id);

        if let Some(actor) = ecs.get_component_mut::<Actor>(caster_id) {
            if actor.hp == actor.max_hp {
                SpellResult::fail(Some(Message::new(format!("{} is already at full health", entity_name), colors::YELLOW)))
            } else {
                actor.hp = actor.max_hp;
                SpellResult::success(
                    caster_id, item_id,
                    Some(Message::new(format!("{} was fully healed", entity_name), colors::GREEN)),
                    None)
            }
        } else {
            SpellResult::fail(None)
        }
    }

    fn fireball_on_target(&self, ecs: &mut Ecs, target_id: EntityId, caster_id: EntityId, item_id: EntityId, radius: u8, damage: u32) -> SpellResult {
        let target_name = Self::get_entity_name(ecs, target_id).to_uppercase();

        let message = Message::new(
            format!("The fireball explodes at {}, burning everything within {} tiles!", target_name, radius), colors::ORANGE,
        );
        let reaction = EntityAction::TakeDamage(target_id, damage);

        let mut spell_result = SpellResult::success(caster_id, item_id, Some(message), Some(reaction));

        // At this point, only entities with a `Positon` can be referenced by target_id, so this
        // unwrap is safe.
        let target = ecs.get_component::<Position>(target_id).unwrap();

        ecs.get_all::<Position>().iter().filter(|(id, p)| {
            **id != target_id && ecs.has_component::<Actor>(**id) && p.distance_to(target.position) <= radius as f64
        }).for_each(|(id, p)| {
            let reaction = EntityAction::TakeDamage(*id, damage / p.distance_to(target.position) as u32);
            spell_result.add_reaction(reaction);
        });

        spell_result
    }

    fn fireball(&self, ecs: &mut Ecs, caster_id: EntityId) -> SpellResult {
        SpellResult::targeting(*self, caster_id)
    }

    fn lightning(&self, ecs: &mut Ecs, fov_map: &Map, caster_id: EntityId, item_id: EntityId, range: u8, damage: u32) -> SpellResult {
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
            let message = Message::new(format!("A lighting bolt strikes the {} with a loud thunder!", target_name),
                                       colors::LIGHT_BLUE);
            SpellResult::success(caster_id, item_id, Some(message),
                                 Some(EntityAction::TakeDamage(target_id, damage)))
        } else {
            SpellResult::fail(Some(Message::new("No valid target in sight and in range".to_string(), colors::RED)))
        }
    }

    fn find_target(&self, ecs: &Ecs, fov_map: &Map, caster: &Position) -> Option<(EntityId, u8)> {
        let mut distances: Vec<(EntityId, u8)> = ecs.get_all::<Position>().iter().filter(|(id, p)| {
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
