use tcod::colors;
use ecs::Ecs;
use ecs::component::{Position, Render, Name, MonsterAi, Actor, Inventory, Level};
use ecs::id::EntityId;
use std::borrow::Cow;
use render::RenderOrder;
use map_objects::map::GameMap;
use random_utils::random_choice_index;
use random_utils::by_dungeon_level;

/// Templates for common Creature types
pub enum CreatureTemplate {
    Troll,
    Orc,
    Player, // The player must stay on the last position, otherwise the random creation will create players
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

    /// Creates the Entity on a given Position-> Option<EntityId>
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

    /// Create a random creature
    pub fn create_random(ecs: &mut Ecs, game_map: &GameMap, pos: (i32, i32), floor_number: u8) -> Option<EntityId>  {
        let available_creatures = vec![
            (CreatureTemplate::Orc, 80),
            (CreatureTemplate::Troll, by_dungeon_level(Cow::Owned(vec![(15, 3), (30, 5), (60, 7)]), floor_number)),
        ];

        let chances = available_creatures.iter().map(|(_,chance)|{
            *chance
        }).collect();

        let ref selection: (CreatureTemplate, i32) = available_creatures[random_choice_index(chances)];
        selection.0.create_on_position(ecs, game_map, pos)
    }


    fn create_player_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.player_entity_id = id;
        ecs.register_component(id, Inventory::new(26));
        ecs.register_component(id, Position::new(id, true));
        ecs.register_component(id, Render::new(id, '@', colors::WHITE, RenderOrder::Actor));
        ecs.register_component(id, Name { name: "Player".to_string()});
        ecs.register_component(id, Actor::new(id, 100, 4, 1, 0));
        ecs.register_component(id, Level::new(id, 1, 200, 0.75));
        Some(id)
    }

    fn create_orc_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Position::new(id, true));
        ecs.register_component(id, Render::new(id, 'o', colors::DESATURATED_GREEN, RenderOrder::Actor));
        ecs.register_component(id, Name { name: "Orc".to_string() });
        ecs.register_component(id, Actor::new(id, 20, 4, 0,35));
        ecs.register_component(id, Level::new(id, 1, 0, 0.0));
        ecs.register_component(id, MonsterAi::new(id));
        Some(id)
    }

    fn create_troll_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Position::new(id, true));
        ecs.register_component(id, Render::new(id, 'T', colors::DARKER_GREEN, RenderOrder::Actor));
        ecs.register_component(id, Name { name: "Troll".to_string()});
        ecs.register_component(id, Actor::new(id, 30, 8, 2,100));
        ecs.register_component(id, Level::new(id, 1, 0, 0.0));
        ecs.register_component(id, MonsterAi::new(id));
        Some(id)
    }
}