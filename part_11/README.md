# Part 11: Leveling Up

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/91do0i/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/11)

Two tasks in this part: Dungeon levels and player levels. Shouldn't be that hard to achieve, as nothing completely
new will be added. I'm just building on what I already have.

Contents of this Writeup:
1. [Dungeon Levels](#dungeons-levels)
    1. [Adding Stairs](#adding-stairs)
    2. [Making the Stairs work](#making-the-stairs-work)
    3. [Creating the next floor](#creating-the-next-floor)
2. [Actor Levels](#actor-levels)
    1. [Extending the Actor](#extending-the-actor)
    2. [Gaining XP](#gaining-xp)
    3. [Leveling Up](#leveling-up)
    4. [Character Screen](#character-screen)
        
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

### Making the Stairs work

In order to make the `Stair` actually do something, I have to define an interaction key. Also, I have to tell the 
`Engine` to create a new dungeon level and clean up the current one (means: remove every `Entity` which has a `Position`).

First one is the easiest. I just add a new `InputAction` which is only used by the `PlayersTurn` game state. It checks
if the player is standing on some `Stairs`, and tells the `Engine` to create a new level.

```rust
Some(InputAction::UseStairs) => {
    let id = ecs.player_entity_id;
    let p = {
        let pos = ecs.get_component::<Position>(id).unwrap();
        (pos.position.0, pos.position.1)
    };
    
    let used_stairs = ecs.get_all_ids::<Stairs>().iter().any(|stair_id| {
        if let Some(stair_pos) = ecs.get_component::<Position>(*stair_id) {
            p.0 == stair_pos.position.0 && p.1 == stair_pos.position.1
        } else {
            false
        }
    });
    
    if used_stairs {
        log.add(Message::new("You go down one level deeper...".to_string(), colors::GREEN));
        GameStateResult {
            next_state: GameState::PlayersTurn,
            engine_action: Some(EngineAction::CreateNextFloor),
        }
    } else {
        log.add(Message::new("No stairs to use here".to_string(), colors::YELLOW));
        GameStateResult {
            next_state: GameState::PlayersTurn,
            engine_action: None,
        }
    }
}
```

### Creating the next floor

Before I can create additional floors I want to know on which floor I am at the moment. Since this information is missing,
I will add a new value to the `Game`: `floor_number` as an indicator for that.

`GameMap` already has a `make_map` method which already does everything I need to make new floors, except for the cleanup
process. I will add the floor number as a new parameter here, and only change the player's `Position` if the `Entity` was
already created.

And I will wrap that into a new method, `next_floor`, which does all the cleanup stuff before generating the map. This
function will be placed in the `Game` struct.

The level transistion works fine after I solved a few issues. First I forgot to reset the map, then I forgot to 
reinitilaize the fov_map.

Now Just display the floor on the ui, and I am done.

## Actor Levels

### Extending the Actor

The second section of this part is about collecting experience and leveling up. First of all, I need to extend `Actor` a 
bit, because I want to know how much XP is rewarded for killing it. I will also add a value which represents the 
`Actors`s level. Doing this I am also able to set an enemy to a different level. Since I do have that information 
available now, I can also update the mouseover display.

### Gaining XP

As a next step, I want the player to be able to collect experience. 

Every time an `Actor` dies, the killer should beawarded the correct amount of XP. I add a new `EntityAction` to be able
to pass an amount of XP to any `Entity`.  The main problem here is that I have no information about who killed the 
`Actor`. This information is not passed to the `TakeDamage` action, but it's dropped immediatelly afert all the time
so it's no big deal to pass it to `TakeDamage`.

To be actually able to do something with the rewarded XP, I need to create yet another `Component` for handling all
the leveling stuff: `Level`. It needs a few values: the total collected xp, and the xp needed for the next level up. 
Also a base value and a factor to calculate the XP needed for the next level. 

Adding the `level` to the component was not that much of a good idea - that's why I will move it over to the `Level`
component.

That's how my XP calculation and the xp rewarding works:

```rust
pub struct Level {
    entity_id: EntityId,
    base: u32,
    factor: f32,
    pub level: u8,
    pub xp_total: u32
}

impl Level {
    pub fn new(entity_id: EntityId, level: u8, base: u32, factor: f32) -> Self {
        Self {
            entity_id,
            xp_total: 0,
            level,
            base,
            factor
        }
    }

    pub fn xp_to_level(&mut self, level: i32) -> u32 {
        (self.base as f32 * (1.0 + self.factor).powi(level as i32 - 1)).floor() as u32
    }

    pub fn reward_xp(&mut self, xp: u32) {
        self.xp_total+=xp;

        let next_level = self.level as i32 + 1;

        if self.xp_total >= self.xp_to_level(next_level) {
            self.level += 1;
        }
    }
}
```

### Leveling Up

Gaining XP works fine, and the level of the `Entity` is also already increased. Now it's time to let something happen
here, too. To keep it simple - on each level up, one stat (HP, defense, power) can be increased.

First of all, the `EntityAction` needs to know if rewarding xp also triggered a level up. I will let the `reward_xp`
function return either true or false, with the latter meaning _no level up_. Also, I only increase the level after
the level up is triggered, not when rewarding the xp.

That's why I did split the method up:

```rust
pub fn reward_xp(&mut self, xp: u32) -> bool {
    self.xp_total+=xp;
    let next_level = self.level as i32 + 1;
    self.xp_total >= self.xp_to_level(next_level)
}

pub fn level_up(&mut self) {
    self.level+=1;
}
```

I need a new `EngineAction` which does the levle up stuff for me. It needs to send a message, actually increase the
level and trigger a new `GameState` - a selection menu which lets me choose the stat I want to increase.

### Character Screen

Since I am now able to level my character up I also want some kind of character Screen to display my current stats.
I start by adding a new `GameState` and a key binding to open it during the `PlayersTurn` state. 

After that, I only need to create a function for it in the `render` module and call it in the right `GameState`.