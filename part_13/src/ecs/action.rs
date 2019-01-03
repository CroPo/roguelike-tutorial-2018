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
use game::state::GameState;
use ecs::spell::SpellResult;
use ecs::spell::SpellStatus;
use tcod::Map;
use ecs::component::Level;
use settings::Settings;
use ecs::component::Equipment;
use ecs::component::Equippable;
use ecs::component::EquipmentSlot;

/// This struct defines the Result of one single action. A message can be created, and also
/// a reaction can happen.
struct ActionResult {
    reactions: Vec<EntityAction>,
    message: Option<Vec<Message>>,
    state: Option<GameState>,
}

impl ActionResult {
    /// Return a `ActionResult` with all values being `None`
    pub fn none() -> ActionResult {
        ActionResult {
            reactions: vec![],
            message: None,
            state: None,
        }
    }
}

/// All possible interactions between `Component`s
#[derive(PartialEq)]
pub enum EntityAction {
    MeleeAttack(EntityId, EntityId),
    TakeDamage(EntityId, u32, EntityId),
    MoveTo(EntityId, (i32, i32)),
    MoveRelative(EntityId, (i32, i32)),
    Die(EntityId),
    PickUpItem(EntityId, EntityId),
    DropItem(EntityId, u8),
    UseItem(EntityId, u8),
    ToggleEquipment(EntityId, u8),
    AddItemToInventory(EntityId, EntityId),
    RemoveItemFromInventory(EntityId, EntityId),
    SetAiTarget(EntityId, EntityId),
    RewardXp(EntityId, u32),
    LevelUp(EntityId),
    UpdateFov(EntityId),
    LookForTarget(EntityId),
    Idle,
}

impl EntityAction {
    /// Execute the action
    pub fn execute(&self, ecs: &mut Ecs, fov_map: &Map, log: Rc<MessageLog>, settings: &Settings) -> Option<GameState> {
        let result = match *self {
            EntityAction::MoveTo(entity_id, pos) => self.move_to_action(ecs, entity_id, pos),
            EntityAction::MoveRelative(entity_id, delta) => self.move_relative_action(ecs, entity_id, delta),
            EntityAction::MeleeAttack(attacker_id, target_id) => self.melee_attack_action(ecs, attacker_id, target_id),
            EntityAction::TakeDamage(entity_id, damage, attacker_id) => self.take_damage_action(ecs, entity_id, damage, attacker_id),
            EntityAction::Die(entity_id) => self.die_action(ecs, entity_id),
            EntityAction::PickUpItem(entity_id, item_id) => self.pick_up_item_action(ecs, entity_id, item_id),
            EntityAction::DropItem(entity_id, item_number) => self.drop_item_action(ecs, entity_id, item_number),
            EntityAction::AddItemToInventory(entity_id, item_id) => self.add_item_to_inventory_action(ecs, entity_id, item_id),
            EntityAction::RemoveItemFromInventory(entity_id, item_id) => self.remove_item_from_inventory_action(ecs, entity_id, item_id),
            EntityAction::UseItem(entity_id, item_number) => self.use_item_action(ecs, fov_map, entity_id, item_number),
            EntityAction::SetAiTarget(entity_id, target_id) => self.set_ai_target_action(ecs, entity_id, target_id),
            EntityAction::RewardXp(entity_id, xp) => self.reward_xp(ecs, entity_id, xp),
            EntityAction::LevelUp(entity_id) => self.level_up(ecs, entity_id),
            EntityAction::LookForTarget(entity_id)  => self.look_for_target_action(ecs, entity_id, settings),
            EntityAction::UpdateFov(entity_id) => self.update_fov_action(ecs, entity_id, settings),
            EntityAction::ToggleEquipment(entity_id, item_number) => self.toggle_item_action(ecs, entity_id, item_number),
            EntityAction::Idle => ActionResult::none() // Idle - do nothing
        };

        if let Some(messages) = result.message {
            for message in messages {
                log.add(message);
            }
        }


        let mut resulting_state = None;
        for reaction in result.reactions {
            resulting_state = if let Some(state) = reaction.execute(ecs, fov_map, Rc::clone(&log), settings) {
                Some(state)
            } else {
                resulting_state
            }
        }

        match result.state {
            None => {
                resulting_state
            }
            _ => {
                result.state
            }
        }
    }

    fn melee_attack_action(&self, ecs: &mut Ecs, attacker_id: EntityId, target_id: EntityId) -> ActionResult {
        let attacker_name = EntityAction::get_entity_name(ecs, attacker_id).to_uppercase();
        let target_name = EntityAction::get_entity_name(ecs, target_id);

        match ecs.get_component::<Actor>(attacker_id) {
            Some(actor) => {
                match actor.calculate_attack(ecs, target_id) {
                    Some(damage) => {
                        ActionResult {
                            message: Some(vec![Message::new(format!("The {} attacks the {} .", attacker_name, target_name), colors::WHITE)]),
                            reactions: vec![EntityAction::TakeDamage(target_id, damage, attacker_id)],
                            state: None,
                        }
                    }
                    None => ActionResult::none()
                }
            }
            None => ActionResult::none()
        }
    }

    fn reward_xp(&self, ecs: &mut Ecs, entity_id: EntityId, xp: u32) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        if let Some(l) = ecs.get_component_mut::<Level>(entity_id) {
            let reactions = if l.reward_xp(xp) {
                vec![EntityAction::LevelUp(entity_id)]
            } else {
                vec![]
            };

            let message = Message::new(format!("{} gains {} XP", entity_name, xp), colors::WHITE);

            ActionResult {
                reactions,
                message: Some(vec![message]),
                state: None,
            }
        } else {
            ActionResult::none()
        }
    }

    fn level_up(&self, ecs: &mut Ecs, entity_id: EntityId) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        if let Some(l) = ecs.get_component_mut::<Level>(entity_id) {

            l.level_up();
            let message = Message::new(format!("{} feels stronger: Reached level {}.", entity_name, l.level), colors::YELLOW);

            ActionResult {
                reactions: vec![],
                message: Some(vec![message]),
                state: Some(GameState::ShowLeveUpMenu),
            }
        } else {
            ActionResult::none()
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

    fn take_damage_action(&self, ecs: &mut Ecs, entity_id: EntityId, damage: u32, attacker_id: EntityId) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();
        if let Some(e) = ecs.get_component_mut::<Actor>(entity_id) {
            e.take_damage(damage);

            let message = Message::new(if damage > 0 {
                format!("The {} takes {}  damage.", entity_name, damage)
            } else {
                format!("The {} takes no damage.", entity_name)
            }, colors::WHITE);

            return if e.hp <= 0 {
                ActionResult {
                    reactions: vec![
                        EntityAction::Die(entity_id),
                        EntityAction::RewardXp(attacker_id, e.xp_reward)
                    ],
                    message: Some(vec![message]),
                    state: None,
                }
            } else {
                ActionResult {
                    reactions: vec![],
                    message: Some(vec![message]),
                    state: None,
                }
            };
        }
        ActionResult::none()
    }

    fn get_equippable_item_id_by_number(&self, ecs: &Ecs,entity_id: EntityId, item_number: u8) -> Option<EntityId>{
        let inventory = ecs.get_component::<Inventory>(entity_id).unwrap();

        let equippables : Vec<&EntityId> = inventory.items.iter().filter(|item_id| {
            ecs.has_component::<Equippable>(**item_id)
        }).collect();

        if equippables.len() > ( item_number as usize ) {
            Some(*equippables[item_number as usize])
        } else {
            None
        }

    }

    fn get_equipment_slot_of_item(&self, ecs: &Ecs,item_id: EntityId) -> Option<EquipmentSlot> {
        match ecs.get_component::<Equippable>(item_id) {
            Some(equippable) => Some(equippable.slot),
            None => None
        }
    }



    fn toggle_item_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_number: u8) -> ActionResult {

        if !ecs.has_component::<Equipment>(entity_id) || !ecs.has_component::<Inventory>(entity_id){
            return ActionResult {
                message: None,
                reactions: vec![],
                state: Some(GameState::ShowInventoryUse),
            }
        }

        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        let item = self.get_equippable_item_id_by_number(ecs, entity_id, item_number);

        match item {
            Some(item_id) => {


                let slot = match self.get_equipment_slot_of_item(ecs, item_id){
                    Some(s) => s,
                    None => EquipmentSlot::None
                };


                let item_name = EntityAction::get_entity_name(ecs, item_id).to_uppercase();

                if let Some(equipment) = ecs.get_component_mut::<Equipment>(entity_id) {

                    let message = Message::new( if let Some(_) = equipment.is_equipped(item_id) {
                        equipment.unequip(item_id);
                        format!("{} unequipped {}", entity_name, item_name)
                    } else {
                        equipment.equip(slot, item_id);
                        format!("{} equipped {}", entity_name, item_name)
                    }, colors::LIGHT_FUCHSIA);

                    ActionResult {
                        message: Some(vec![message]),
                        reactions: vec![],
                        state: Some(GameState::ShowInventoryEquip),
                    }
                } else {
                    ActionResult {
                        message: None,
                        reactions: vec![],
                        state: Some(GameState::ShowInventoryEquip),
                    }
                }

            },
            _ => {
                ActionResult {
                    message: None,
                    reactions: vec![],
                    state: Some(GameState::ShowInventoryEquip),
                }
            }
        }
    }

    fn use_item_action(&self, ecs: &mut Ecs, fov_map: &Map, entity_id: EntityId, item_number: u8) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        let mut item_name = "".to_string();
        let mut item_id = 0;

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
            let mut messages = vec![Message::new(format!("{} uses {}", entity_name, item_name), colors::WHITE)];
            let id = ecs.player_entity_id;

            let cast_result = s.cast(ecs, fov_map, id);

            let state = match cast_result {
                SpellResult { status: SpellStatus::Success, .. } => {
                    self.use_item_success(ecs, item_id)
                }
                SpellResult { status: SpellStatus::Targeting(spell, caster_id), .. } => {
                    Some(GameState::Targeting(spell, caster_id))
                }
                SpellResult { status: SpellStatus::Fail, .. } => {
                    Some(GameState::ShowInventoryUse)
                }
            };

            match cast_result {
                SpellResult { message: Some(message), .. } => messages.push(message),
                _ => ()
            }


            return ActionResult {
                message: Some(messages),
                reactions: cast_result.reactions,
                state,
            };
        } else {

            ActionResult {
                message: None,
                reactions: vec![],
                state: Some(GameState::ShowInventoryUse),
            }
        }
    }

    fn use_item_success(&self, ecs: &mut Ecs, item_id: EntityId) -> Option<GameState> {
        ecs.destroy_entity(&item_id);
        None
    }

    fn remove_item_from_inventory_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_id: EntityId) -> ActionResult {
        if let Some(inventory) = ecs.get_component_mut::<Inventory>(entity_id) {
            inventory.remove_item_id(item_id);
        }
        ActionResult::none()
    }

    fn drop_item_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_number: u8) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();
        let mut item_name = "".to_string();
        let mut item_id = 0;

        let item_position = if let Some(inventory) = ecs.get_component::<Inventory>(entity_id) {
            if inventory.items.len() > item_number as usize {
                item_id = inventory.items[item_number as usize];
                item_name = EntityAction::get_entity_name(ecs, item_id).to_uppercase();

                if let Some(p) = ecs.get_component::<Position>(entity_id) {
                    let mut item_position = Position::new(entity_id, false);
                    item_position.position = p.position;
                    Some(item_position)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(p) = item_position {
            let message = Message::new(format!("{} dropped {} on the floor", entity_name, item_name), colors::YELLOW);

            ecs.register_component(item_id, p);
            if let Some(inventory) = ecs.get_component_mut::<Inventory>(entity_id) {
                inventory.remove_item(item_number as usize);
            }

            ActionResult {
                reactions: vec![],
                message: Some(vec![message]),
                state: None,
            }
        } else {
            ActionResult {
                reactions: vec![],
                message: None,
                state: Some(GameState::ShowInventoryDrop),
            }
        }
    }

    fn pick_up_item_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_id: EntityId) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();
        let item_name = EntityAction::get_entity_name(ecs, item_id).to_uppercase();

        if let Some(inventory) = ecs.get_component::<Inventory>(entity_id) {
            if inventory.free_space() > 0 {
                let message = Message::new(format!("{} picked up the {}", entity_name, item_name),
                                           colors::BLUE);

                ActionResult {
                    reactions: vec![EntityAction::AddItemToInventory(entity_id, item_id)],
                    message: Some(vec![message]),
                    state: None,
                }
            } else {
                let message = Message::new(format!("You can't pick up {}: Inventory is full.",
                                                   item_name), colors::YELLOW);

                ActionResult {
                    reactions: vec![],
                    message: Some(vec![message]),
                    state: None,
                }
            }
        } else {
            ActionResult::none()
        }
    }

    fn add_item_to_inventory_action(&self, ecs: &mut Ecs, entity_id: EntityId, item_id: EntityId) -> ActionResult {
        ecs.remove_component::<Position>(item_id);

        if let Some(inventory) = ecs.get_component_mut::<Inventory>(entity_id) {
            inventory.add_item(item_id);
        }
        ActionResult::none()
    }


    fn die_action(&self, ecs: &mut Ecs, entity_id: EntityId) -> ActionResult {
        let entity_name = EntityAction::get_entity_name(ecs, entity_id).to_uppercase();

        let message = if entity_id == ecs.player_entity_id {
            Message::new("YOU DIED".to_string(), colors::RED)
        } else {
            Message::new(format!("The {} died.", entity_name), colors::ORANGE)
        };

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
            reactions: vec![],
            message: Some(vec![message]),
            state: None,
        }
    }

    fn set_ai_target_action(&self, ecs: &mut Ecs, entity_id: EntityId, target_id: EntityId) -> ActionResult {
        if let Some(ai) = ecs.get_component_mut::<MonsterAi>(entity_id) {
            ai.set_target(target_id);
        }

        ActionResult {
            reactions: vec![],
            message: None,
            state: None,
        }
    }

    fn look_for_target_action(&self, ecs: &mut Ecs, entity_id: EntityId, settings: &Settings) -> ActionResult {

        let target_in_fov =  {
            if let Some(ai) = ecs.get_component::<MonsterAi>(entity_id) {
                ai.is_within_ai_distance(ecs, settings) && ai.is_target_in_fov(ecs, settings)
            } else {
                false
            }
        } ;

        if let Some(ai) = ecs.get_component_mut::<MonsterAi>(entity_id) {
            ai.set_chasing_target(target_in_fov);
        }


        ActionResult {
            reactions: vec![],
            message: None,
            state: None,
        }
    }

    fn update_fov_action(&self, ecs: &mut Ecs, entity_id: EntityId, settings: &Settings) -> ActionResult {

        let position = {
            if let Some(p) = ecs.get_component::<Position>(entity_id) {
                Some((p.x(), p.y()))
            } else {
                None
            }
        };


        match position {
            Some((x, y)) => {
                if let Some(ai) = ecs.get_component_mut::<MonsterAi>(entity_id) {
                    ai.recompute_fov(settings, x, y)
                }
            },
            _ => {}
        }


        ActionResult {
            reactions: vec![],
            message: None,
            state: None,
        }
    }

    fn get_entity_name(ecs: &Ecs, id: EntityId) -> String {
        match ecs.get_component::<Name>(id) {
            Some(n) => n.name.clone(),
            None => format!("nameless entity (#{})", id)
        }
    }
}