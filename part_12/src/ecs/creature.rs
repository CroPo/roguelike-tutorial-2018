use tcod::colors;
use ecs::Ecs;
use ecs::component::{Position, Render, Name, MonsterAi, Actor, Inventory, Level};
use ecs::id::EntityId;
use render::RenderOrder;
use map_objects::map::GameMap;

/// Templates for common Creature types
pub enum CreatureTemplate {
    Player,
    Troll,
    Orc,
}

impl CreatureTemplate {
    /// Create Some Entity from the Selected template, or None if the templates isn't implemented yet
    pub fn create(&self, ecs: &mut Ecs, game_map: &GameMap) -> Option<EntityId> {
        match *self {
            CreatureTemplate::Player => CreatureTemplate::create_player_from_template(ecs),
            CreatureTemplate::Troll => CreatureTemplate::create_troll_from_template(ecs),
            CreatureTemplate::Orc => CreatureTemplate::create_orc_from_template(ecs),
        }
    }

    /// Creates the Entity on a given Position
    pub fn create_on_position(&self, ecs: &mut Ecs, game_map: &GameMap, pos: (i32, i32)) -> Option<EntityId> {
        match self.create(ecs, game_map) {
            Some(id) => {
                match ecs.get_component_mut::<Position>(id) {
                    Some(p) => p.position = pos,
                    _ => ()
                }
                Some(id)
            }
            _ => None
        }
    }

    fn create_player_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.player_entity_id = id;
        ecs.register_component(id, Inventory::new(26));
        ecs.register_component(id, Position::new(id, true));
        ecs.register_component(id, Render::new(id, '@', colors::WHITE, RenderOrder::Actor));
        ecs.register_component(id, Name { name: "Player".to_string()});
        ecs.register_component(id, Actor::new(id, 30, 5, 2, 0));
        ecs.register_component(id, Level::new(id, 1, 200, 0.75));
        Some(id)
    }

    fn create_orc_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Position::new(id, true));
        ecs.register_component(id, Render::new(id, 'o', colors::DESATURATED_GREEN, RenderOrder::Actor));
        ecs.register_component(id, Name { name: "Orc".to_string() });
        ecs.register_component(id, Actor::new(id, 10, 3, 0,35));
        ecs.register_component(id, Level::new(id, 1, 0, 0.0));
        ecs.register_component(id, MonsterAi::new(id));
        Some(id)
    }

    fn create_troll_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Position::new(id, true));
        ecs.register_component(id, Render::new(id, 'T', colors::DARKER_GREEN, RenderOrder::Actor));
        ecs.register_component(id, Name { name: "Troll".to_string()});
        ecs.register_component(id, Actor::new(id, 16, 4, 1,100));
        ecs.register_component(id, Level::new(id, 1, 0, 0.0));
        ecs.register_component(id, MonsterAi::new(id));
        Some(id)
    }
}