# Part 13: Adventure gear

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/13)


Finally, the last part of the tutorial. Not much of an introduction here, I will just start with the development

Contents of this Writeup:

1. [Making stuff equippable](#making-stuff-equippable)
    1. [Building the components](#building-the-components)
    2. [Placing equipment in the dungeon](#placing-equipment-in-the-dungeon)

## Making stuff equippable

### Building the components

Well, I haven't written a completely new component in a while, so I hope I will get this done in a good time. The 
`Equippable` component doesn't seem to complex to build, though. Just a few values, and no real method so far. This
component will be an additional component to those items which can be equipped (obviously).

```rust
enum EquipmentSlot {
    MainHand,
    OffHand,
    Armor
}

pub struct Equippable {
    entity_id: EntityId,
    bonus_strength: i32,
    bonus_defense: i32,
    bonus_max_hp: i32,
    slot: EquipmentSlot
}

```

You may have noticed that I added an extra equipment slot here, `Armor`. It's use should be pretty obvious.

In addition to this, I will also add an `new` method to serve as constructor, and an implementation for `Serialize` 
and `Deserialize`, for both the equipment slot and the component.

My `Equipment` implementation holds a HashMap of all already equipped slots, indexed by the slot itself. This way
I can prevent that one slot will be equipped multiple times. The value of the Map is the `EntityId` of the equipped
item.


```rust
pub struct Equipment {
    entity_id: EntityId,
    slots: HashMap<EquipmentSlot, EntityId>
}
```

`Serialization` and `Deserialization` are also done for this component. Both are a bit tricky, since I work with
a `HashMap` here, but nothing which I can't handle with some time

```rust
impl Serialize for Equipment {

    fn serialize(&self) -> JsonValue {
        let mut slots = JsonValue::new_array();
        self.slots.iter().for_each(|(slot, entity_id)| {
            slots.push(object!(
                "slot" => slot.serialize(),
                "id" => *entity_id
            ));
        });

        object!(
        "type" => "Equipment",
            "data" => object!(
                "id" => self.entity_id,
                "slots" => slots
            )
        )
    }
}

impl Deserialize for Equipment {
    fn deserialize(json: &JsonValue) -> Self {
        let mut slots : HashMap<EquipmentSlot, EntityId> = HashMap::new();

        for entity_json in json["slots"].members() {

            let slot = EquipmentSlot::deserialize(&json["slot"]);
            let id = entity_json["id"].as_u16().unwrap();

            slots.insert(slot, id);
        }

        Equipment {
            entity_id: json["id"].as_u16().unwrap(),
            slots, 
        }
    }
}
```

Of course, both components need to be saved and loaded correctly, which means I have to adapt the `Serialize` and
`Deserialize` implementation of `Ecs`, which is just copypasting some already existing lines.

### Placing equipment in the dungeon

The next logical step is, in my opinion, to add new items to the world. To achieve this, I have to alter the `Item` 
component quite a bit, since, as of right now, only a `Spell` can be added to these. I just have to make the `Spell`
optional (via `Option<Spell>`). If no spell exists, the `Item` will count as 'equippable'. Also, I will rename the current
`new` method of the `Item` trait to `consumable`, and create another constructor named `equippable`, which has no parameters. 

Serialization and Deserialization need to be altered a bit, too, since I have to deal with an optional value.

The item creation has to be adapted, too, to be able to create equippable items at all. I extended the `ItemTemplate` by
three new entries, with four parameters each: The item's name, and an integer for one of the three stats (power, defense, hp).
For the sake of simplicity, armor can only ever increase HP, a weapon can only increase the power and a shield can
only increase defense. The actual slot will be determined by the `enum` value itself.

```rust
pub enum ItemTemplate {
    HealthPotion(u32),
    LightningScroll(u8, u32),
    FireballScroll(u8, u32),
    ConfusionScroll,
    Weapon(String, i32,),
    Shield(String, i32,),
    Armor(String, i32,)
}
```

Now I have to add a few of these items to the random item generator, and I should be able to find and collect them in the
game. I will actually add a few of each type with different potency, so the player will find better gear in deeper levels.
Side note: The numbers are _already_ extremely unbalanced at the moment, since the player has already the potential to
get pretty much immortal. So I won't really mess up anything hear, just want to try out stuff.

Picking up seems to work fine, so on to the next part: Equipping an item.
