use entities::Entity;
use tcod::colors;

/// Templates for common Creature types
pub enum CreatureTemplate {
    Player,
    Troll,
    Orc
}

impl CreatureTemplate {
    /// Create Some Entity from the Selected template, or None if the templates isn't implemented yet
    pub fn create(&self) -> Option<Entity> {
        match *self {
            CreatureTemplate::Player => Some(CreatureTemplate::create_player_from_template()),
            CreatureTemplate::Troll => Some(CreatureTemplate::create_troll_from_template()),
            CreatureTemplate::Orc => Some(CreatureTemplate::create_orc_from_template()),
            _ => None
        }
    }

    /// Returns true, if the selected template is the player creature
    pub fn is_player(&self) -> bool {
        match *self {
            CreatureTemplate::Player => true,
            _ => false
        }
    }

    fn create_player_from_template() -> Entity {
        Entity {
            pos: (0, 0),
            glyph: '@',
            color: colors::WHITE,
            name: "Player".to_string(),
            blocks: true,
        }
    }

    fn create_orc_from_template() -> Entity {
        Entity {
            pos: (0, 0),
            glyph: 'o',
            color: colors::DESATURATED_GREEN,
            name: "Orc".to_string(),
            blocks: true,
        }
    }

    fn create_troll_from_template() -> Entity {
        Entity {
            pos: (0, 0),
            glyph: 'T',
            color: colors::DARKER_GREEN,
            name: "Orc".to_string(),
            blocks: true,
        }
    }
}