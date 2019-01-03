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
use ecs::component::EquipmentSlot;
use ecs::component::Equippable;
use tcod::Color;

/// Templates for common Creature types
pub enum ItemTemplate {
    HealthPotion(u32),
    LightningScroll(u8, u32),
    FireballScroll(u8, u32),
    ConfusionScroll,
    Weapon(String, i32,),
    Shield(String, i32,),
    Armor(String, i32, )
}

impl ItemTemplate {
    /// Create Some Entity from the Selected template, or None if the templates isn't implemented yet
    pub fn create(&self, ecs: &mut Ecs) -> Option<EntityId> {
        match *self {
            ItemTemplate::HealthPotion(amount) => ItemTemplate::create_health_potion_from_template(ecs, amount),
            ItemTemplate::LightningScroll(range, damage) => ItemTemplate::create_lightning_scroll_from_template(ecs, range, damage),
            ItemTemplate::FireballScroll(radius, damage) => ItemTemplate::create_fireball_scroll_from_template(ecs, radius, damage),
            ItemTemplate::ConfusionScroll => ItemTemplate::create_confusion_scroll_from_template(ecs),
            ItemTemplate::Weapon(ref name, power) => ItemTemplate::create_weapon_from_template(ecs, name.clone(), power),
            ItemTemplate::Shield(ref name, defense) => ItemTemplate::create_shield_from_template(ecs, name.clone(), defense),
            ItemTemplate::Armor(ref name, hp) => ItemTemplate::create_armor_from_template(ecs, name.clone(), hp),
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
            (ItemTemplate::Armor("Leather Armor".to_string(), 20), by_dungeon_level(Cow::Owned(vec![(10, 1),(0, 4)]), floor_number)),
            (ItemTemplate::Armor("Iron Armor".to_string(), 40), by_dungeon_level(Cow::Owned(vec![(5, 4),(0, 7)]), floor_number)),
            (ItemTemplate::Armor("Mithril Armor".to_string(), 40), by_dungeon_level(Cow::Owned(vec![(1, 7)]), floor_number)),
            (ItemTemplate::Weapon("Copper Dagger".to_string(), 1), by_dungeon_level(Cow::Owned(vec![(10, 1),(0, 4)]), floor_number)),
            (ItemTemplate::Weapon("Iron Axe".to_string(), 2), by_dungeon_level(Cow::Owned(vec![(5, 4),(0, 7)]), floor_number)),
            (ItemTemplate::Weapon("Mithril Sword".to_string(), 4), by_dungeon_level(Cow::Owned(vec![(1, 7)]), floor_number)),
            (ItemTemplate::Shield("Wooden Buckler".to_string(), 1), by_dungeon_level(Cow::Owned(vec![(10, 1),(0, 4)]), floor_number)),
            (ItemTemplate::Shield("Iron Shield".to_string(), 2), by_dungeon_level(Cow::Owned(vec![(5, 4),(0, 7)]), floor_number)),
            (ItemTemplate::Shield("Mithril Shield".to_string(), 4), by_dungeon_level(Cow::Owned(vec![(1, 7)]), floor_number)),
        ];

        let chances = available_creatures.iter().map(|(_,chance)|{
            *chance
        }).collect();

        let ref selection: (ItemTemplate, i32) = available_creatures[random_choice_index(chances)];
        selection.0.create_on_position(ecs, pos)
    }


    fn create_health_potion_from_template(ecs: &mut Ecs, amount: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::consumable(Spell::Heal(id, amount)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '!', colors::VIOLET, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Healing Potion".to_string() });
        Some(id)
    }

    fn create_lightning_scroll_from_template(ecs: &mut Ecs, range: u8, damage: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::consumable(Spell::Lightning(id, range, damage)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::YELLOW, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Lightning Scroll".to_string()});
        Some(id)
    }
    fn create_fireball_scroll_from_template(ecs: &mut Ecs, radius: u8, damage: u32) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::consumable(Spell::Fireball(id, radius, damage)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::RED, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Fireball Scroll".to_string() });
        Some(id)
    }

    fn create_confusion_scroll_from_template(ecs: &mut Ecs) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::consumable(Spell::Confusion(id)));
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, '#', colors::PINK, RenderOrder::Item));
        ecs.register_component(id, Name { name: "Confusion Scroll".to_string() });
        Some(id)
    }

    fn create_equippable(ecs: &mut Ecs, name: String, glyph: char, color: Color, power: i32, defense: i32, hp: i32, slot: EquipmentSlot ) -> Option<EntityId> {
        let id = ecs.create_entity();
        ecs.register_component(id, Item::equippable());
        ecs.register_component(id, Position::new(id, false));
        ecs.register_component(id, Render::new(id, glyph, color, RenderOrder::Item));
        ecs.register_component(id, Equippable::new(id, power, defense, hp, slot));
        ecs.register_component(id, Name { name });

        Some(id)
    }

    fn create_weapon_from_template(ecs: &mut Ecs, name: String, power: i32) -> Option<EntityId> {
        ItemTemplate::create_equippable(ecs, name, '/', colors::SKY, power, 0, 0, EquipmentSlot::MainHand)
    }

    fn create_shield_from_template(ecs: &mut Ecs, name: String, defense: i32) -> Option<EntityId> {
        ItemTemplate::create_equippable(ecs, name, '[', colors::DARKER_ORANGE, 0, defense, 0, EquipmentSlot::OffHand)
    }

    fn create_armor_from_template(ecs: &mut Ecs, name: String, hp: i32) -> Option<EntityId> {
        ItemTemplate::create_equippable(ecs, name, ')', colors::LIGHTER_CRIMSON, 0,0 , hp, EquipmentSlot::Armor)
    }
}