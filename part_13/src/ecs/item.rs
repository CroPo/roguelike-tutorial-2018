use std::borrow::Cow;
use tcod::colors;
use ecs::Ecs;
use ecs::component::Position;
use ecs::component::Render;
use ecs::component::Name;
use ecs::id::EntityId;
use render::RenderOrder;
use ecs::component::Item;
use ecs::spell::Spell;
use random_utils::random_choice_index;
use random_utils::by_dungeon_level;

/// Templates for common Creature types
pub enum ItemTemplate {
    HealthPotion(u32),
    LightningScroll(u8, u32),
    FireballScroll(u8, u32),
    ConfusionScroll,
}

impl ItemTemplate {
    /// Create Some Entity from the Selected template, or None if the templates isn't implemented yet
    pub fn create(&self, ecs: &mut Ecs) -> Option<EntityId> {
        match *self {
            ItemTemplate::HealthPotion(amount) => ItemTemplate::create_health_potion_from_template(ecs, amount),
            ItemTemplate::LightningScroll(range, damage) => ItemTemplate::create_lightning_scroll_from_template(ecs, range, damage),
            ItemTemplate::FireballScroll(radius, damage) => ItemTemplate::create_fireball_scroll_from_template(ecs, radius, damage),
            ItemTemplate::ConfusionScroll => ItemTemplate::create_confusion_scroll_from_template(ecs),
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

    /// Create a random item
    pub fn create_random(ecs: &mut Ecs, pos: (i32, i32), floor_number: u8) -> Option<EntityId>  {
        let available_creatures = vec![
            (ItemTemplate::HealthPotion(40), 70),
            (ItemTemplate::ConfusionScroll, by_dungeon_level(Cow::Owned(vec![(25, 4)]), floor_number)),
            (ItemTemplate::FireballScroll(3, 25), by_dungeon_level(Cow::Owned(vec![(25, 6)]), floor_number)),
            (ItemTemplate::LightningScroll(5,40), by_dungeon_level(Cow::Owned(vec![(10, 2)]), floor_number)),
        ];

        let chances = available_creatures.iter().map(|(_,chance)|{
            *chance
        }).collect();

        let ref selection: (ItemTemplate, i32) = available_creatures[random_choice_index(chances)];
        selection.0.create_on_position(ecs, pos)
    }


    fn create_health_potion_from_template(ecs: &mut Ecs, amount: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::new(Spell::Heal(id, amount)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '!', colors::VIOLET, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Healing Potion".to_string() });
        Some(id)
    }

    fn create_lightning_scroll_from_template(ecs: &mut Ecs, range: u8, damage: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::new(Spell::Lightning(id, range, damage)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::YELLOW, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Lightning Scroll".to_string()});
        Some(id)
    }
    fn create_fireball_scroll_from_template(ecs: &mut Ecs, radius: u8, damage: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::new(Spell::Fireball(id, radius, damage)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::RED, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Fireball Scroll".to_string() });
        Some(id)
    }

    fn create_confusion_scroll_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::new(Spell::Confusion(id)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::PINK, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Confusion Scroll".to_string() });
        Some(id)
    }
}