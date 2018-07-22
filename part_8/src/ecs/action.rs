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
use ecs::component::Inventory;
use ecs::component::Item;

/// This struct defines the Result of one single action. A message can be created, and also
/// a reaction can happen.
struct ActionResult {
    reaction: Option<EntityAction>,
    message: Option<Vec<Message>>,
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
    PickUpItem(EntityId, EntityId),
    UseItem(EntityId, u8),
    AddItemToInventory(EntityId, EntityId),
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
            EntityAction::PickUpItem(entity_id, item_id) => self.pick_up_item_action(ecs, entity_id, item_id),
            EntityAction::AddItemToInventory(entity_id, item_id) => self.add_item_to_inventory_action(ecs, entity_id, item_id),
            EntityAction::UseItem(entity_id, item_number) => self.use_item_action(ecs, entity_id, item_number),
            EntityAction::Idle => ActionResult::none() // Idle - do nothing
        };

        if let Some(messages) = result.message {
            for message in messages {
                log.add(message);
            }
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
                    message: Some(vec![message]),
                }
            } else {
                ActionResult {
                    reaction: None,
                    message: Some(vec![message]),
                }
            };
        }
        ActionResult::none()
    }

    fn use_item_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_number: u8) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        let mut item_name = "".to_string();
        let mut item_id =0;

        let spell = if let Some(inventory) = ecs.get_component::<Inventory>(entity_id) {
            if inventory.items.len() > item_number as usize {
                item_id = inventory.items[item_number as usize];
                item_name = EntityAction::get_entity_name(ecs, item_id).to_uppercase();

                if let Some(i) = ecs.get_component::<Item>(item_id) {
                    i.use_item()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(s) = spell {
            let mut messages = vec![Message::new(format!("{} uses {}", entity_name, item_name))];
            let id = ecs.player_entity_id;

            if let Some(message) = s.execute(ecs, id) {
                messages.push(message)
            }

            ecs.destroy_entity(&item_id);

            if let Some(inventory) = ecs.get_component_mut::<Inventory>(entity_id) {
                inventory.remove_item(item_number as usize);
            }

            return ActionResult {
                message: Some(messages),
                reaction: None
            };
        }

        ActionResult::none()
    }

    fn pick_up_item_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_id: EntityId) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();
        let item_name = EntityAction::get_entity_name(ecs, item_id).to_uppercase();

        if ecs.has_component::<Inventory>(entity_id) {
            ecs.remove_component::<Position>(item_id);

            let message = Message::new(format!("{} picked up {}", entity_name, item_name));

            ActionResult {
                reaction: Some(EntityAction::AddItemToInventory(entity_id, item_id)),
                message: Some(vec![message]),
            }
        } else {
            ActionResult::none()
        }
    }

    fn add_item_to_inventory_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_id: EntityId) -> ActionResult {
        if let Some(inventory) = ecs.get_component_mut::<Inventory>(entity_id) {
            inventory.add_item(item_id);
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
            message: Some(vec![message]),
        }
    }

    fn get_entity_name(ecs: &Ecs, id: EntityId) -> String {
        match ecs.get_component::<Name>(id) {
            Some(n) => n.name.clone(),
            None => format!("a nameless entity (#{})", id)
        }
    }
}