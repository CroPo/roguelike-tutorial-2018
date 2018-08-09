use std::rc::Rc;

use tcod::colors;
use tcod::input::{check_for_event, EventFlags};

use ecs::Ecs;
use ecs::action::EntityAction;
use ecs::component::MonsterAi;
use ecs::component::Corpse;
use ecs::component::Position;
use ecs::component::Item;
use ecs::component::Actor;
use message::MessageLog;
use tcod::Map;
use map_objects::map::GameMap;
use message::Message;
use ecs::spell::Spell;
use ecs::id::EntityId;
use game::input::*;
use map_objects::fov::recompute_fov;
use settings::Settings;
use game::Game;
use engine::Engine;
use std::cell::RefMut;
use engine::EngineAction;


pub struct GameStateResult {
    pub next_state: GameState,
    pub engine_action: Option<EngineAction>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    PlayersTurn,
    EnemyTurn,
    PlayerDead,
    ShowInventoryUse,
    ShowInventoryDrop,
    ShowQuitGameMenu,
    Targeting(Spell, EntityId),
    MainMenu,
}

impl GameState {
    pub fn run(&self, engine: &Engine, game: &RefMut<Game>) -> GameStateResult {
        let input_action = handle_input(self, check_for_event(EventFlags::all()));
        let log = game.log.clone();
        
        let mut ecs = game.ecs.borrow_mut();
        let mut fov_map = game.fov_map.borrow_mut();
        let map = game.map.borrow();

        match *self {
            GameState::PlayersTurn => self.player_turn(&mut ecs, &mut fov_map, input_action, log, &map, &engine.settings),
            GameState::EnemyTurn => self.enemy_turn(&mut ecs, &fov_map, log, &map),
            GameState::PlayerDead => self.player_dead(input_action),
            GameState::MainMenu => self.main_menu(input_action),
            GameState::ShowQuitGameMenu => self.quit_game_menu(input_action),
            GameState::ShowInventoryUse | GameState::ShowInventoryDrop => self.show_inventory(&mut ecs, &fov_map, input_action, log),
            GameState::Targeting(spell, caster_id) => self.targeting(&mut ecs, &fov_map, input_action, log, spell, caster_id),
        }
    }

    fn targeting(&self, ecs: &mut Ecs, fov_map: &Map, action: Option<InputAction>,
                 log: Rc<MessageLog>, spell: Spell, caster_id: EntityId) -> GameStateResult {
        match action {
            Some(InputAction::Exit) => {
                log.add(Message::new("Target selection was canceled".to_string(), colors::WHITE));
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: None,
                }
            }
            Some(InputAction::MousePos(x, y)) => {
                GameStateResult {
                    next_state: *self,
                    engine_action: Some(EngineAction::MousePos(x as i32, y as i32)),
                }
            }
            Some(InputAction::SelectEntity(x, y)) => {
                let targets: Vec<EntityId> = ecs.get_all::<Position>().iter().filter(|(id, p)| {
                    ecs.has_component::<Actor>(**id) && p.position.0 == x as i32 && p.position.1 == y as i32
                }).map(|(id, _)|{*id}).collect();

                if let Some(target) = targets.first() {
                    let spell_result = spell.cast_on_target(ecs, *target, caster_id);

                    if let Some(message) = spell_result.message {
                        log.add(message)
                    }

                    for action in spell_result.reactions {
                        action.execute(ecs, fov_map, Rc::clone(&log));
                    }

                    GameStateResult {
                        next_state: GameState::EnemyTurn,
                        engine_action: None,
                    }
                } else {
                    log.add(Message::new("No valid target at the selected position".to_string(), colors::YELLOW));

                    GameStateResult {
                        next_state: *self,
                        engine_action: None,
                    }
                }
            }
            _ => {
                GameStateResult {
                    engine_action: None,
                    next_state: *self,
                }
            }
        }
    }

    fn show_inventory(&self, ecs: &mut Ecs, fov_map: &Map, action: Option<InputAction>, log: Rc<MessageLog>) -> GameStateResult {
        match action {
            Some(InputAction::Exit) => {
                GameStateResult {
                    engine_action: None,
                    next_state: GameState::PlayersTurn,
                }
            }
            Some(InputAction::SelectOption(item_key)) => {
                if item_key as u8 >= 'a' as u8 {
                    let item_number = item_key as u8 - 'a' as u8;

                    let next_state = if let Some(state) = match *self {
                        GameState::ShowInventoryDrop => EntityAction::DropItem(ecs.player_entity_id, item_number as u8),
                        GameState::ShowInventoryUse => EntityAction::UseItem(ecs.player_entity_id, item_number as u8),
                        _ => EntityAction::Idle
                    }.execute(ecs, fov_map, log) {
                        state
                    } else {
                        GameState::EnemyTurn
                    };

                    GameStateResult {
                        engine_action: None,
                        next_state,
                    }
                } else {
                    GameStateResult {
                        engine_action: None,
                        next_state: GameState::PlayersTurn,
                    }
                }
            }
            _ => {
                GameStateResult {
                    engine_action: None,
                    next_state: *self,
                }
            }
        }
    }

    fn player_dead(&self, action: Option<InputAction>) -> GameStateResult {
        match action {
            Some(InputAction::Exit) => {
                GameStateResult {
                    next_state: GameState::MainMenu,
                    engine_action: Some(EngineAction::QuitGame(false)),
                }
            }
            _ => {
                GameStateResult {
                    next_state: GameState::PlayerDead,
                    engine_action: None,
                }
            }
        }
    }

    fn main_menu(&self, action: Option<InputAction>) -> GameStateResult {
        match action {
            Some(InputAction::Exit) | Some(InputAction::SelectOption('c')) => {
                GameStateResult {
                    next_state: GameState::MainMenu,
                    engine_action: Some(EngineAction::Exit),
                }
            }
            Some(InputAction::SelectOption('a')) => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: Some(EngineAction::StartGame(false)),
                }
            }
            Some(InputAction::SelectOption('b')) => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: Some(EngineAction::StartGame(true)),
                }
            }
            _ => {
                GameStateResult {
                    next_state: GameState::MainMenu,
                    engine_action: None,
                }
            }
        }
    }
    fn quit_game_menu(&self, action: Option<InputAction>) -> GameStateResult {
        match action {
            Some(InputAction::Exit) | Some(InputAction::SelectOption('b')) => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: None,
                }
            }
            Some(InputAction::SelectOption('a')) => {
                GameStateResult {
                    next_state: GameState::MainMenu,
                    engine_action: Some(EngineAction::QuitGame(true)),
                }
            }
            _ => {
                GameStateResult {
                    next_state: GameState::ShowQuitGameMenu,
                    engine_action: None,
                }
            }
        }
    }

    fn player_turn(&self, ecs: &mut Ecs, fov_map: &mut Map, action: Option<InputAction>, log: Rc<MessageLog>, map: &GameMap, settings: &Settings) -> GameStateResult {

        recompute_fov(ecs, fov_map, settings);

        match action {
            Some(InputAction::Exit) => {
                GameStateResult {
                    next_state: GameState::ShowQuitGameMenu,
                    engine_action: None,
                }
            }
            Some(InputAction::MousePos(x, y)) => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: Some(EngineAction::MousePos(x as i32, y as i32)),
                }
            }
            Some(InputAction::Fullscreen) => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: Some(EngineAction::ToggleFullscreen),
                }
            }
            Some(InputAction::ShowInventory) => {
                GameStateResult {
                    next_state: GameState::ShowInventoryUse,
                    engine_action: None,
                }
            }
            Some(InputAction::ShowInventoryDrop) => {
                GameStateResult {
                    next_state: GameState::ShowInventoryDrop,
                    engine_action: None,
                }
            }
            Some(InputAction::PickUp) => {
                let id = ecs.player_entity_id;
                let p = {
                    let pos = ecs.get_component::<Position>(id).unwrap();
                    (pos.position.0, pos.position.1)
                };
                let mut actions: Vec<EntityAction> = vec![];

                ecs.get_all_ids::<Item>().iter().filter(|item_id| {
                    if let Some(item_pos) = ecs.get_component::<Position>(**item_id) {
                        p.0 == item_pos.position.0 && p.1 == item_pos.position.1
                    } else {
                        false
                    }
                }).for_each(|item_id| {
                    actions.push(EntityAction::PickUpItem(id, *item_id))
                });

                let next_state = if actions.is_empty() {
                    log.add(Message::new("Nothing to pick up here".to_string(), colors::YELLOW));
                    GameState::PlayersTurn
                } else {
                    actions.iter().for_each(|a| {
                        a.execute(ecs, fov_map, Rc::clone(&log));
                    });
                    GameState::EnemyTurn
                };

                GameStateResult {
                    next_state,
                    engine_action: None,
                }
            }
            Some(InputAction::MovePlayer(vel_x, vel_y)) => {
                let id = ecs.player_entity_id;

                let destination = {
                    let p = ecs.get_component::<Position>(id).unwrap();
                    (p.position.0 + vel_x, p.position.1 + vel_y)
                };

                let action = if !map.is_move_blocked(destination.0, destination.1) {
                    let targets = Position::is_blocked_by(&ecs, destination);

                    if let Some(target_id) = targets.iter().next() {
                        let player_creature = ecs.get_component::<Actor>(id).unwrap();
                        match player_creature.calculate_attack(&ecs, *target_id) {
                            Some(_x) => EntityAction::MeleeAttack(id, *target_id),
                            None => EntityAction::Idle
                        }
                    } else {
                        EntityAction::MoveRelative(id, (vel_x, vel_y))
                    }
                } else {
                    EntityAction::Idle
                };

                action.execute(ecs, fov_map, Rc::clone(&log));

                GameStateResult {
                    next_state: GameState::EnemyTurn,
                    engine_action: None,
                }
            }
            _ => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: None,
                }
            }
        }
    }

    fn update_enemy_ai_target(&self, ecs: &mut Ecs, fov_map: &Map, log: Rc<MessageLog>) {
        let player_id = ecs.player_entity_id;

        let actions :Vec<EntityAction> = ecs.get_all::<MonsterAi>().iter().filter(|(_, ai)|{
            ai.has_no_target()
        }).map(|(id, _)| {
            EntityAction::SetAiTarget(*id, player_id)
        }).collect();

        actions.iter().for_each(|action| {
            action.execute(ecs, fov_map, Rc::clone(&log));
        });
    }

    fn enemy_turn(&self, ecs: &mut Ecs, fov_map: &Map, log: Rc<MessageLog>, map: &GameMap) -> GameStateResult {
        self.update_enemy_ai_target(ecs, fov_map, Rc::clone(&log));

        let entity_ids = ecs.get_all_ids::<MonsterAi>();

        entity_ids.iter().for_each(|entity_id| {
            let action = match ecs.get_component::<MonsterAi>(*entity_id) {
                Some(ai) => ai.calculate_turn(&ecs, &map),
                _ => EntityAction::Idle
            };
            action.execute(ecs, fov_map, Rc::clone(&log));
        });

        if ecs.has_component::<Corpse>(ecs.player_entity_id) {
            GameStateResult {
                next_state: GameState::PlayerDead,
                engine_action: None,
            }
        } else {
            GameStateResult {
                next_state: GameState::PlayersTurn,
                engine_action: None,
            }
        }
    }
}

