# Part 10: Saving and Loading

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/91do0i/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/10)

So, we're starting with loading and saving the game here. This time, I can just loosely resemble what the Python
tutorial does due to the obvious difference of both languages. Since I already have done a part of what the tutorial
wanted me to do - namely move all the game logic to another class - I don't expect much problems at all.

Contents of this Writeup:
1. [Settings](#settings)
2. [Saving the Game](#saving-the-game)
3. [Saving the Game - Attempt 2](#saving-the-game---attempt-2)
4. [Loading the Game](#loading-the-game)
5. [The Main Menu](#the-main-menu)
6. [Conclusion](#conclusion)

## Settings

First thing to do is move alle the used coonfiguration values, like `screen_width`, which are spread across the `main.rs`
file into a own `struct`. It's not that hard, but it will take some time to do this. 

Since the configuration is read only, I don't need to mind about mutable acces being a problem.

Since it is pretty big in size with no logic, I won't put the `struct` here directly, but you can always view the source
[here](src/settings.rs). Practically, it's just a `struct` which holds all configuration `values`, a `new` method which
initializes the default values and a getter for each value.

For my next step, I simply remove all the values from the `main()` method, and resolve all resulting errors. Wherever 
it makes sense I will pass the whole `Settings` as a reference instead of each value individually. Even though this 
will remove the lose binding I now have, it will help me a bit because I won't have to pass more values than my screen 
can display in a line any more.

With these changes, the whole code looks quite a bit cleaner. Well, at least somewhat cleaner.

## Saving the Game

Let's face the problem here directly: Serialization and deserialization is way too much work for the scope of this tutorial
to not use a existing library. Even though I am curious about how this can be done in Rust, I will try to use [serde](https://serde.rs/) here,
one of the most popular serializers. Together with `serde-json` and `serde-derive`, this should do most of the work for me.

### Serializing the Components

I will start with the innermost layer of data: The `Component` structs. I simply add `#[derive(Serialize, Deserialize, Clone)]` 
to each, which should work fine for most of the Data.

The first error I encounter is the following one: 
```
error[E0277]: the trait bound `tcod::Color: serde::Serialize` is not satisfied
```

This could be a problem, but going through the `tcod-rs` source code, I found this:

```rust
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
```

This means that all I need to do is activating a feature here, so the `Color` can be serialized, too. 

This can be done by changing the dependencies in the `Cargo.toml`:

```toml
[dependencies.tcod]
version = "0.12"
features = ["serialization"]
```

All other errrors are similar: I need to implement the `Serialize` trait for every `enum` and `struct` which will be 
serialized when I serialize the `Component`s.

With this part of the serialization (and, of course, deserialization) running, I can go right to the next layer.

### Serializing the EcsStorage

This may be a bit harder, as `TypeId`, and `Any`, isn't supported by `serde_derive` out of the box as it seems:
```text
error[E0277]: the trait bound `std::any::TypeId: serde::Serialize` is not satisfied
```
I probably need to implement my own serializer for this. So let's just try that. 

Since I try to keep it as simple as possible, I will only serialize this to an array of the components. All other
data can be calculated again on the deserialization (namely, the `TypeId`).

A default serialization implementation isn't possible, either, since the `Components` are stored as `Box<Any>`, and `Any`
doesn't implement `Serialize` and `Deserialize` either. 

So I can't go on like this, either. The next attempt isnt really a nice one, though. This is my serializer:

```rust
impl Serialize for EcsStorage {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut storage = serializer.serialize_struct("Entity", self.data.len())?;

        if let Some(c) = self.get::<Position>() {
            storage.serialize_field("position", &c)?;
        }
        if let Some(c) = self.get::<Render>() {
            storage.serialize_field("render", &c)?;
        }
        if let Some(c) = self.get::<Name>() {
            storage.serialize_field("name", &c)?;
        }
        if let Some(c) = self.get::<Actor>() {
            storage.serialize_field("actor", &c)?;
        }
        if let Some(c) = self.get::<MonsterAi>() {
            storage.serialize_field("ai", &c)?;
        }
        if let Some(c) = self.get::<Corpse>() {
            storage.serialize_field("Name", &c)?;
        }
        if let Some(c) = self.get::<Item>() {
            storage.serialize_field("Name", &c)?;
        }
        if let Some(c) = self.get::<Inventory>() {
            storage.serialize_field("Name", &c)?;
        }
        storage.end()
    }
}
```
Ugly, but it compiles. Let's just all hope this actually works. Also, I need to create a matching deserializer...
later!

Also, the test output seems to be ok, too, so I'll just go with this here. 

### Serializing everything else

Only a few more things need to be serialized now, and none of them implements stuff I can't use out of the box anymore.
Well, mostly. For the `Ecs`, I need to find a way to serialize the value of `IdGenerator` so I save the last used Id.
I could also implement a custom deserializer, which calculates the last used id from the entities and updates the
`IdGenerator` once loaded - which is the way I will go here.

The next thing to seriaize is the `GameMap`, which is not much more than a vector of `Tile`s, so both of these can be derived.
So is the last `struct` to serialize: The `MessageLog`.

### Bringing it all together

One thing left to do for serialization: I need to bring all of the serialized values together into one `struct`, so I can
serialize them to one `String`.

For this, practically everything that is in the `main.rs` now needs to be moved somewhere else, into a new `struct` which
I will just call `Game`.

This is a rather lengthy task, and since it's only some code restructuring which doesn't directly affect the saving and loading
I won't go much further into detail here. You can always look at the source code if you want further implementation details.

### Saving

At last, we need to write the generated json into a file which can be loaded again. For now, I will simply do that when
I end the game with ESC. This, of course, has one major flaw: the game will be saved when the player is dead. Normally,
in the case of a player's death, the game would delete the save game (because of permadeath).

To address this issue, I will add a `bool` value to the `EngineAction::Exit`. If `true`, the game will be saved, if `false`,
a saved game will be deleted. In the context of this tutorial, no other possiblity exists - either save or delete, no
_Exit without saving_ or _die without deletion_.

This workes well so far. Now to actually load the saved game again.

## Saving the Game - Attempt 2

So, this whole thing didn't quite go as planned. Even though I was able to serialize all the data I need I ran into a 
dead end while trying to deserialie it. I came to the conclusion that it won't take as much time to implement a own
JSON serialization that it would take me to implement a deserializer for the `serde` library.

So, this is what I will do.

### Resetting

The first thing to do is to carefully remove all the Serialization and Deserialization code. I will simply do this
by trial and error, removing the crates from `Cargo.toml` and then working through each and every `rustc` error until
I can run the game again.

### Some thoughts

So, what do I really need to serialize in order to completely restor the game? Pretty much the same as I wanted before.
This means I need to implement serialization for the `MessageLog` (easy), the `GameMap` (easy) and the `Ecs` (not _that_ easy).
I will still be using the JSON format, since there is a nice JSON library for Rust which I am already somewhat familiar
with. Of course, I could implement my own saving format, but JSON already exists and is able to store everything I need here.

But to be fully honest with you: I wouldn't store a whole game map as JSON. JSON is just not made for that kind of data 
(imo). Some binary storage format would be better here, but for this tutorial I will simply store everything into one
big JSON file.

### Basic Serialization

First of all, I include the `json` library [(I'm using this particular one)](https://github.com/maciejhirsz/json-rust). 
After that, I will create a `Serialize` trait, which has one function: `serialize()`, returning a `JsonValue`. Of course,
this is way less dynamic and even more code than the previous attempt on the first look, but it will help me to
solve the problems I couldn't with the library.

Since I already have a `savegame` module I will simply put all the serialization and deserialization stuff in there too.

And with this simple first implementation, the first step is already done:
```rust
impl Serialize for Game {
    fn serialize(&self) -> JsonValue {
        object!(
            "ecs" => "",
            "log" => "",
            "map" => "",
        )
    }
}
```

Of course, this will only return empty values now, but I will expand it continously.

### Serializing the Map and the Log

The next thing I will implement serialization for is the `GameMap`. It has only three values: width, height, and a `Vec<Tile>`.

```rust
impl Serialize for GameMap {
    fn serialize(&self) -> JsonValue {

        let mut tiles = JsonValue::new_array();

        for tile in self.tiles.iter() {
            tiles.push(tile.serialize());
        }

        object!(
            "width" => self.dimensions.0,
            "height" => self.dimensions.1,
            "tiles" => tiles
        )
    }
}
```

Something similar was done with the `Tile` itself, and there it is: A fully serialized `GameMap`.

```rust
impl Serialize for Tile {
    fn serialize(&self) -> JsonValue {
        array![self.block_move, self.block_sight, self.explored]
    }
}
```

The `MessageLog` is similarly easy:

```rust
impl Serialize for MessageLog {
    fn serialize(&self) -> JsonValue {
        let mut messages = JsonValue::new_array();
        for message in self.messages.borrow().iter() {
            messages.push(message.serialize());
        }
        messages
    }
}
```

And, of course, the `Message`:

```rust
impl Serialize for Message {
    fn serialize(&self) -> JsonValue {

        let mut color = JsonValue::new_array();

        color.push(self.color.r);
        color.push(self.color.g);
        color.push(self.color.b);

        object!(
            "text" => self.text.clone(),
            "color" => color
        )
    }
}
```

Now that this is done: To the harder part

### Serializing the ECS

As before, each `Component` needs to be serialized individually. This will take some time, but it's not too complex
either. Of course, some, like the inventory, might be a bit more work, but nothing entirely unhandleable.

The main problem is I need to distinguish between the different `Component` implmentations in a way that I can 
deserialize it again. Practically, whatever I do, I need to `match` the structs against some primitive value.

This is what the `Position` structs serialization looks like:

```rust
impl Serialize for Position {
    fn serialize(&self) -> JsonValue {
        object!(
        "type" => "Position",
        "data" => object!(
                "x" => self.position.0,
                "y" => self.position.0,
                "blocking" => self.is_blocking,
            )
        )
    }
}
```

`Spell` and `RenderOrder` also needed to be serializeable. Putting all of this together gives us quite a big file
if compared against the game's scope. But optimizing the save file is nothing I intend to do at this point.

Now with this done (again), I will need to implement loading the game. This time for real!

## Loading the game

Similarly to the serialization, I will create a `Deserialize` trait which, well, uses the json data to rebuild the
game state.

On the `Game` struct, I will create a new static function, `from_json`, which returns the loaded `Game` and takes two
arguments: `Settings` and a `JsonValue`. 

To be able to go step by step through the deserialization, I will copy the body of the `new` function and replace
everything here step by step.

First of all, because it's the easiest, I will deserialize the `MessageLog`.  A few small issues beside this works
exactly as I thought it will. 

This, for example, is the deserialization code of a `Message`:

```rust
impl Deserialize for Message {
    fn deserialize(json: &JsonValue) -> Self {

        Message {
            text: json["text"].as_str().unwrap().to_string(),
            color: Color {
                r : json["color"][0].as_u8().unwrap(),
                g : json["color"][1].as_u8().unwrap(),
                b : json["color"][2].as_u8().unwrap(),
            }
        }
    }
}
```

I just came to the conclusion that it's fine to unwrap every value. Either everything is in place, or the save file is
corrupted. So this is fine, imo.

Deserializing the map will be pretty similar, with one exception: All `Entities` are created while creating the Map
right now. This means the Game will ineviatebly crash on load because I have no player `Entity` which can be accessed.
This means a bit of tinkering will be needed to get everything running again. But once It runs, it will run.

I will do the `GameMap` first, since it's pretty similar to the `MessageLog`. Pretty much as easy as I thought it would 
be. And last but not least, I need to deserialize the `Ecs`, too. This might cause some problems, and I will also need
to add some additional functions to the `Ecs` and the `EcsStorage` structs.

After implementing everything, and after everythng compiles... the game crashes at the start.
```text
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', libcore/option.rs:345:21
```
This means that an `Component` is either missing or added to the wrong `Entity`. 

As it turns out, besides a few typos, one of my main mistakes was the deserialization of the `glyph` value for `Render`

```rust
impl Deserialize for Render {
    fn deserialize(json: &JsonValue) -> Self {
        Render {
            entity_id: json["id"].as_u16().unwrap(),
            glyph: json["glyph"].as_str().unwrap().chars().next().unwrap(),
            order: RenderOrder::deserialize(&json["order"]),
            color: Color {
                r : json["color"][0].as_u8().unwrap(),
                g : json["color"][1].as_u8().unwrap(),
                b : json["color"][2].as_u8().unwrap(),
            }
        }
    }
}
```

As you see, the on line for deserializing it is rather complex. I simply got it wrong on my first try.

Deserilization now seems to work, becaus I got a completely different problem now:
```text
thread 'main' panicked at 'assertion failed: x < width && y < height',  .cargo/registry/src/github.com-1ecc6299db9ec823/tcod-0.12.1/src/map.rs:53:9
```
That practically means I got something with the map sizes wrong I think. It turned out a bit differently, though: 
I serialized the x coordinate of `Position` two times, instead of x and y.

Loading a Game basically works now. However, using a `Spell` does not. I got some bug in deserializing these then.
I had a typo here, too. However, this typo happened in the serialization process, and not in deserializing.

After these few issues were resolved, I am able to save a game on exit, and load it when I start the game again.

### The Main Menu

The one thing left to do to finish this tutorial part is creating a main menu, so the game won't immediatelly start in
a dungeon when the application is run. Also, the player should be able to choose between continuing the game or starting
a new one. I will display both option independentally of a save file being present. If the player chooses to load a game
without a save file being present, a new game will be run (just like it happens now).

Also, when pressing the escape key while the game is running a selection will be shown with the option to either quit, save
and quit, or cancel. Except when the player is dead - he will immediatelly be returned to the main menu.

To Achieve this in a smooth way, I need to separate the `run` function of the `Game` struct, and put it into another place.
After that I can actually add the main menu as an own `GameState`. 

It was quite some work, but now everything is running again same as before. That initial work was needed. I will also
have to modify the creation of a new game, too, at least a bit.

The main menu was added, and handling the ESC key while the game is running was modified - a selection is shown which
lets you either save and quit, or cancel and continue the game.

And... that's pretty much it. This part is done (finally).

## Conclusion

Most of the things I wanted to say are already written down. I really lost motivation here for a short time after I spent
hours and hours for serializing just to find out I won't be able to deserialize it.

Separating the `Engine` and the `Game` took some time, too, but was not that much of a problem since I utilize a lot
of `RefCell<>` values here. Thank the Rust devs for that.

One more thing: I _will_ finish the Game. Just three parts left, and I am normally not the guy who gives up because some
things won't work. 