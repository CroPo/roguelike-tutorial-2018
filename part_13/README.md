# Part 13: Adventure gear

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/13)


Finally, the last part of the tutorial. Not much of an introduction here, I will just start with the development

Contents of this Writeup:

1. [Makinng stuff equippable](#making-stuff-equippable)
    1. [Building the components](#building-the-components)

## Making stuff equippable

### Building the components

Well, I haven't written a completely new component in a while, so I hope I will get this done in a good time. The 
`Equippable` component doesn't seem to complex to build, though. Just a few values, and no real method so far. This
component will be an additional component to those items which can be equipped (obviously).

```rust
enum EquipmentSlot {
    MainHand(Option<EntityId>),
    OffHand(Option<EntityId>)
}

pub struct Equippable {
    entity_id: EntityId,
    bonus_strength: i32,
    bonus_defense: i32,
    bonus_max_hp: i32,
    slot: EquipmentSlot
}

```

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