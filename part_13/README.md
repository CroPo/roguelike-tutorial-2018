# Part 13: Adventure gear

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/13)


Finally, the last part of the tutorial. Not much of an introduction here, I will just start with the development

Contents of this Writeup:

1. [Making stuff equippable](#making-stuff-equippable)
    1. [Building the components](#building-the-components)
    2. [Placing equipment in the dungeon](#placing-equipment-in-the-dungeon)
    3. [Equipping an item](#equipping-an-item)
    4. [Making equipment work](#making-equipment-work)
2. [(Final) Conclusion](#final-conclusion)

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


### Equipping an item

First of all, the `Equipment` component needs to be extended a bit. Right now, it can't do anything at all. Also,
the player `Entity` needs an `Equipment` component, too. In order to do this, a `new` method will be added which serves
as default constructor, as usual.

Other than that, I need a bunch of methods:
- equip an item
- unequip an item
- check if an item is equipped

```rust
pub fn equip(&mut self, ecs: &Ecs, item_id: EntityId) {
    match ecs.get_component::<Equippable>(item_id) {
        Some(equippable) => { self.slots.insert(equippable.slot, item_id); },
        None => ()
    };
}
```

Equipping an item will simply override the equipped status of any previously equipped item.

```rust
    pub fn unequip(&mut self, ecs: &Ecs, item_id: EntityId) {
        if let Some(slot) = self.is_equipped(item_id) {
            self.slots.remove(&slot);
        }
    }

    pub fn is_equipped(&self, item_id: EntityId) -> Option<EquipmentSlot>{
        for (slot, item_in_slot) in &self.slots {
            if item_id == *item_in_slot {
                return Some(*slot);
            }
        }
        None
    }
```

For unequipping, I simply iterate over all slots and see if the `EntityId` of the item in this slot matches. If the item
was found `Some` slot will be returned, otherwise `None`.

With that feature being fully functional now, the equip menu is next on the list. If the player selects an not equipped
item from the list, it will be equipped. If the item is already equipped, it will be unequipped.

The hotkey for opening the menu will be `e`.

### Making equipment work

Well, now we can pick up, drop and equip armor and weapons. They just don't do much until now. Time to change that! 
First of all I will disable direct access to the values `max_hp`, `power` and `defense` of the `Actor` component, and
replace them with functions of the same name, which calculate the total, including equipment.

Also, since the level up needs to still be working I need methods to increase these three values.

After that, I just have to get the Equipment and add all stat bonuses together.

## (Final) Conclusion

Well, that's it. The tutorial is done, with a, let's say, usual delay in software dev business ;-)

All I can say at this point is that, even if it was a very big struggle from start to end, I learned incredibly much.
My main intention with following the tutorial was to learn Rust. And I think I really gained some useful skills, since
I pretty much made every possible mistake you can make in a language like Rust at least once while following the 
tutorial.

But that's not all. I learned about game development, about game architecture. 

And, probably most important for me: I finally _finished_ a hobby project.

Maybe I will occasionally continue developing this game, but it will be more experimenting around and stuff. But one
thing I can say for sure: I _will_ develop my own, new game. A roguelike. And maybe you will see even a release this
year.


