# Part 11: Leveling Up

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/91do0i/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/11)

Two tasks in this part: Dungeon levels and player levels. Shouldn't be that hard to achieve, as nothing completely
new will be added. I'm just building on what I already have.

Contents of this Writeup:
1. [Dungeon Levels](#dungeons-levels)
    1. [Adding Stairs](#adding-stairs)


## Dungeons Levels

### Adding Stairs

I will start exactly as the Python tutorial suggests, by adding components to the game wich help me build a stair `Entity`.
Of course, this includes the whole serialization/deserialization, too.

A stair has a `Name`, a `Position`, can be `Render`ed and can be interacted with. The last `Component` ist the only one
missing here right now. Practically, I could create an `Interaction` component, which is similar to the `Item` - it 
would let me do _something_ on use, opening a chest for example, or in this case, get us down to the next level of 
the dungeon. Either I do it like this, or I have to make a own `Component` for each of these actions. The main
advantage of using one `Component` for all is that the key handling is multiple times easier, since I only need to
write it once. Disadvantage is - making more then one action for an `Entity` is quite a bit harder. In this case, I
will use one `Component` for one possible action. The key associated to 'interact with' should then give me a list of 
all posisble actions, which are collected by scanning the `Component`s of the `Entity` it's being used on. If there 
is only one possible action, it will be executed immediatelly. Since this game only has one kind of possible action
right now, I will skip the selection menu in this case.

The `Stair` component is written easily. Since we only can go further down, and not up again, no value needs to be added
to the `Component`. Serialization and deserialization can be copied from `Corpse` (which has no values either).

```rust
pub struct Stair {}

impl Serialize for Stair {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Stair",
        "data" => object!()
        )
    }
}

impl Deserialize for Stair {
    fn deserialize(_json: &JsonValue) -> Self {
        Stair {
        }
    }
}

impl Component for Stair {}
```

This is actually the whole code for `Stair` (Serialization needs to be extended in the `Ecs`, too).
Of course, this only designates an `Entity` as stair, and doesn't do anything else.

Exactly as the Python Tutorial suggests, I will add the stairs to the next dungeon level in the last created room. This
is not too much work, too. I built a simple function which does everything I need.

```rust
fn add_stair(&mut self, ecs: &mut Ecs, room: &Rect) {
    let id = ecs.create_entity();
    ecs.register_component(id, Stair {});
    ecs.register_component(id, Position {
        entity_id: id,
        position: room.center(),
        is_blocking: false,
    });
    ecs.register_component(id, Render::new(id, '>',
                                           colors::WHITE, RenderOrder::Stair));
    ecs.register_component(id, Name {
        name: String::from("Stairs"),
    });
}
```
