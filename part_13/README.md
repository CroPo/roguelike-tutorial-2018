# Part 13: Adventure gear

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/13)


Finally, the last part of the tutorial. Not much of an introduction here, I will just start with the development

Contents of this Writeup:


## The `Equippable` component

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

As you might notice, I defined the `EquipmentSlot` enum in a way that allows an `EntityId` to be passed
as an additional parameter. This way I can easily save which Actor wears which item on which slot. For simplicity -
if the `EntityId` is 0, nothing is worn on that slot. And, of course, the `Actor` gets another component, `Equipment`. 