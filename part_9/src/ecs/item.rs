use tcod::colors;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Render;
use ecs::component::Name;
use ecs::id::EntityId;
use ecs::component::MonsterAi;
use ecs::component::Actor;
use render::RenderOrder;
use ecs::component::Item;
use ecs::spell::Spell;

/// Templates for common Creature types
pub enum ItemTemplate {
    HealthPotion,
    LightningScroll(u8, u32),
}

impl ItemTemplate {
    /// Create Some Entity from the Selected template, or None if the templates isn't implemented yet
    pub fn create(&self, ecs: &mut Ecs) -> Option<EntityId> {
        match *self {
            ItemTemplate::HealthPotion => ItemTemplate::create_health_potion_from_template(ecs),
            ItemTemplate::LightningScroll(range, damage) => ItemTemplate::create_lightning_scroll_from_template(ecs, range, damage),
        }
    }

    /// Creates the Entity on a given Position
    pub fn create_on_position(&self, ecs: &mut Ecs, pos: (i32, i32)) -> Option<EntityId> {
        match self.create(ecs) {
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

    fn create_health_potion_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::new(Spell::Heal));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '!', colors::VIOLET, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Healing Potion".to_string(), description: "Restores health when used".to_string() });
        Some(id)
    }

    fn create_lightning_scroll_from_template(ecs: &mut Ecs, range: u8, damage: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::new(Spell::Lightning(range, damage)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::YELLOW, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Lightning Scroll".to_string(), description: "Casts a Lightning which strikes the nearest enemy".to_string() });
        Some(id)
    }
}