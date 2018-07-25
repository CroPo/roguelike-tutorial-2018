use std::rc::Rc;

use tcod;
use tcod::colors;
use tcod::input;
use tcod::input::{check_for_event, EventFlags, Event, Mouse, Key, KeyCode};

use EngineAction;
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

pub struct GameStateResult {
    pub next_state: GameState,
    pub engine_action: Option<EngineAction>,
}

/// Action are triggered by the input (mouse & keys)
enum InputAction {
    MovePlayer(i32, i32),
    MousePos(isize, isize),
    SelectOption(char),
    PickUp,
    ShowInventory,
    ShowInventoryDrop,
    Fullscreen,
    Exit,
}


fn handle_input(state: &GameState, event: Option<(EventFlags, Event)>) -> Option<InputAction> {
    if let Some(e) = event {
        match e {
            (tcod::input::KEY_PRESS, Event::Key(key)) => {
                match state {
                    GameState::PlayersTurn => handle_keys_player_turn(key),
                    GameState::PlayerDead => handle_keys_dead(key),
                    GameState::ShowInventoryUse | GameState::ShowInventoryDrop => handle_keys_selection_menu(key),
                    GameState::EnemyTurn => None,
                    _ => None
                }
            }
            (_, Event::Mouse(mouse)) => handle_mouse(mouse),
            _ => None
        }
    } else {
        None
    }
}


fn handle_mouse(mouse: Mouse) -> Option<InputAction> {
    match mouse {
        Mouse { .. } => Some(InputAction::MousePos(mouse.cx, mouse.cy)),
    }
}

fn handle_keys_player_turn(key: Key) -> Option<InputAction> {
    match key {
        Key { code: KeyCode::Left, .. } | Key { printable: 'h', .. } => Some(InputAction::MovePlayer(-1, 0)),
        Key { code: KeyCode::Right, .. } | Key { printable: 'l', .. } => Some(InputAction::MovePlayer(1, 0)),
        Key { code: KeyCode::Up, .. } | Key { printable: 'k', .. } => Some(InputAction::MovePlayer(0, -1)),
        Key { code: KeyCode::Down, .. } | Key { printable: 'j', .. } => Some(InputAction::MovePlayer(0, 1)),
        Key { printable: 'y', .. } => Some(InputAction::MovePlayer(-1, -1)),
        Key { printable: 'u', .. } => Some(InputAction::MovePlayer(1, -1)),
        Key { printable: 'b', .. } => Some(InputAction::MovePlayer(-1, 1)),
        Key { printable: 'n', .. } => Some(InputAction::MovePlayer(1, 1)),
        Key { printable: 'i', .. } => Some(InputAction::ShowInventory),
        Key { printable: 'd', .. } => Some(InputAction::ShowInventoryDrop),
        Key { printable: 'g', .. } => Some(InputAction::PickUp),
        Key { code: KeyCode::Escape, .. } => Some(InputAction::Exit),
        Key { code: KeyCode::Enter, alt: true, .. } => Some(InputAction::Fullscreen),
        _ => None
    }
}

fn handle_keys_selection_menu(key: Key) -> Option<InputAction> {
    match key {
        Key { code: KeyCode::Escape, .. } => Some(InputAction::Exit),
        Key { printable, .. } => Some(InputAction::SelectOption(printable)),
        _ => None
    }
}

fn handle_keys_dead(key: Key) -> Option<InputAction> {
    match key {
        Key { code: KeyCode::Escape, .. } => Some(InputAction::Exit),
        _ => None
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    PlayersTurn,
    EnemyTurn,
    PlayerDead,
    ShowInventoryUse,
    ShowInventoryDrop,
}

impl GameState {
    pub fn run(&self, ecs: &mut Ecs, log: Rc<MessageLog>, map: &GameMap) -> GameStateResult {
        let input_action = handle_input(self, check_for_event(EventFlags::all()));

        match *self {
            GameState::PlayersTurn => self.player_turn(ecs, input_action, log, map),
            GameState::EnemyTurn => self.enemy_turn(ecs, input_action, log, map),
            GameState::PlayerDead => self.player_dead(input_action),
            GameState::ShowInventoryUse | GameState::ShowInventoryDrop => self.show_inventory(ecs, input_action, log)
        }
    }

    fn show_inventory(&self, ecs: &mut Ecs, action: Option<InputAction>, log: Rc<MessageLog>) -> GameStateResult {
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
                    }.execute(ecs, log) {
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
                    next_state: GameState::PlayerDead,
                    engine_action: Some(EngineAction::Exit),
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

    fn player_turn(&self, ecs: &mut Ecs, action: Option<InputAction>, log: Rc<MessageLog>, map: &GameMap) -> GameStateResult {
        match action {
            Some(InputAction::Exit) => {
                GameStateResult {
                    next_state: GameState::PlayersTurn,
                    engine_action: Some(EngineAction::Exit),
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
                        a.execute(ecs, Rc::clone(&log));
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
                            Some(x) => EntityAction::TakeDamage(*target_id, x, id),
                            None => EntityAction::Idle
                        }
                    } else {
                        EntityAction::MoveRelative(id, (vel_x, vel_y))
                    }
                } else {
                    EntityAction::Idle
                };

                action.execute(ecs, Rc::clone(&log));

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

    fn enemy_turn(&self, ecs: &mut Ecs, action: Option<InputAction>, log: Rc<MessageLog>, map: &GameMap) -> GameStateResult {
        let entity_ids = ecs.get_all_ids::<MonsterAi>();

        entity_ids.iter().for_each(|entity_id| {
            let action = match ecs.get_component::<MonsterAi>(*entity_id) {
                Some(ai) => ai.calculate_turn(&ecs, &map),
                _ => EntityAction::Idle
            };
            action.execute(ecs, Rc::clone(&log));
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

