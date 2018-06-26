# Part 2: 
## The generic Entity, the render functions, and the map

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8twiwa/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/2)

I tried to follow the python tutorial step by step, just like I did in part [part_1](../part_1). I try to go a bit more into detail for a few decisions.

### The `Entity` struct

#### One file to rule them all - or split up into multiple files?

The Python tutorial suggest to create an dedicated file which holds the `Entity` class. I chose follow along, even if the scope of this project is still rather small. As far as I know, and please correct me here, Rust doesn't really make any difference if you use one single file to hold everything, or split your stuff up into multiple files. Opposed to C or C++, compile times do not suffer (but again, the scope is small, we would barely - if even - recognize that here).

#### The implementation

First of all, Rust needs to know about the created file, which Rust calls _Module_. Think of them as Namespaces with some extra features. The Module name is always like the file name (of course, there is a second way to name modules, but I'm fine with that by now).

Simply adding one line below the `extern crate ...` will do the trick: 

```rust
mod entity; 
```

The file itself contains the `Entity` struct, which doesn't differ much from the Python counterpart.

```rust
pub struct Entity {
    pos: (i32, i32),
    glyph: char,
    color: ::tcod::colors::Color
}
```

The most notable difference is maybe that I use a tuple for storing the entity's position. And I changed the name of the representing character to `glyph`, because I dont want to use a type's name for a variable.

If you look close, you may see that I didn't grant public access for any of the struct's members. That's because I read a bit ahead and decided to take a slightly different approach on the rendering, because I simply don't like the idea of public access to everything.

The method implementations are pretty much as expected. I won't go into more detail here. 

#### Using the `Entity`

One thing straight ahead: This part will differ from the tutorial, and I honestly do not know how to handle it correctly at that level. 

Adding the player itself is not the problem. The problem starts when I have to put all entities (including the player) into one Collection for the sake of rendering them. Because, if I want to move the player around, I need to borrow a mutable reference to the `player` object whenever I call the `mv` method. And having a single mutable (means: writable) reference is exclusive to having any other kind of reference. Ownership and Borrwing are two core concepts which one needs to get used to when one wants to develop in Rust. Once you get it, you successfully took the first big hurdle (the first of many more, but by far the biggest, imo).

```rust
let mut entities = vec![npc, player];
```

Here, the ownership of both `npc` and `player` is passed to the `entities` vector. Which means I can't use both of them anymore - the compiler will throw an error and refuse to compile here. A pretty helpful error, but still an error:

```
   |
44 |     let mut entities = vec![npc, player];
   |                                  ------ value moved here
...
65 |             Some(Action::MovePlayer(move_x, move_y)) => player.mv((move_x, move_y)),
   |                                                         ^^^^^^ value used here after move
   |
   = note: move occurs because `player` has type `entity::Entity`, which does not implement the `Copy` trait
```

Regarding the note, just to mention it: I don't want to copy any of these entities here. Entities should only exist once.

So, the second attempt would be to either pass a borrow, or even a mutable borrow to the vector.

In both cases, I can't use the `player` (and the `npc`) variable any more.   
- To move the player, i need mutable access:  
```rust
// The player needs to be mutable, because we change it's value
player.mv((move_x, move_y)
``` 
- If I add non-mutable borrows to the vector, I can't use any mutable access to the `Entity`
- If I add mutable borrows to the vector, I can only use exactly this mutable borrow, and not a second one.

Without using more advanced Rust components (like shared pointers and stuff like that - which I won't use here because I simply haven't reached the chapter in my Rust book which covers those), I decided for the following solution. I'm not _that_ happy with it, so please suggest a better one - I'll appreciate it!

##### The solution

I move the ownership over to the vector (in fact, I don't. I just initialze the vector with both entities):

```rust
let mut entities = vec![
    Entity::new(screen_width / 2, screen_height / 2, '@', colors::WHITE),
    Entity::new(screen_width / 2 - 5, screen_height / 2, '@', colors::YELLOW),
];
```

... and then, I create a variable which indicates the player entity position in the vector:

```rust
let player_entity_index : usize = 0;
```

Of course, the player entity always needs to stay on position 0 in the vector. This way.

Whenever I need mutable access, I can simply access the vector:

```rust
entities[player_entity_index].mv((move_x, move_y))
```

As said, not absolutely happy here, but it compiles, and that's all I wanted for now.

#### Enity rendering

The Python tutorial suggests to move the rendering to a separate file, which I will follow. I won't create a dedicated rendering struct either, as this won't give me any advantage. Still, I want to try a different approach here (again). 

As said before, I don't really want to expose my `Entity` struct's members. So I am going to use a trait here.

```rust
pub trait Render {
    fn draw(&self, console: &mut Console);
    fn clear(&self, console: &mut Console);
}

```

All I need to do is implement this trait on each struct I want to render. The advantage is that the struct can decide by itself how it wants to get drawn. The render functions just need to know that an entity gets drawn onto the console. The second method is for removing a character. Of course, both need to be implemented.

On the `Entity`, this looks like that (just the draw method):

```rust
impl ::render::Render for Entity {
    fn draw(&self, console: &mut Console) {
        console.set_default_foreground(self.color);
        console.put_char(self.pos.0, self.pos.1, self.glyph, BackgroundFlag::None);
    }
    // ...
}
```

The rest of the render methods, just for the sake of completeness:

```rust
pub fn render_all<T: Render>(objs: &Vec<T>, console: &mut Root, screen_width: i32, screen_height: i32) {

    let mut offscreen = Box::new(Offscreen::new(screen_width, screen_height));

    for obj in objs {
        obj.draw(&mut offscreen);
    }

    console::blit(&offscreen,
                  (0, 0),
                  (screen_width, screen_height),
                  console,
                  (0, 0), 1.0, 1.0);
}

pub fn clear_all<T: Render>(objs: &Vec<T>,console: &mut Console) {
    for obj in objs {
        obj.clear(console);
    }
}
```

#### The Map and Tiles

Back to moving alongside the tutorial. No problems occured during the creation of `Tile`.

`GameMap`, however... The Python tutorial wants to use a two-dimensional array, where both indexes indicate a coordinate on the Map. 

Creating a two dimensional vector is a bit of a chore in Rust. I need to create a vector of vectors, just like this:
```rust
let mut tiles = vec![vec![Tile::new(false, false);height ]; width];
```
Ugly, and it is not compatible with the function siganture of the `render_all` method. Which expects a single vector of structs which implement Render, not a vector of vectors. So, how do I get a single Vector here?

Well..

```rust
let mut tiles = vec![Tile:new(false, false): width * height];
```

Obviously, getting the tile of a specific coordinate will be a bit harder, but at least I can use my `Render` trait on the `Tile` struct.
 
It's not that hard, though:
```rust
let x = 5,
let y = 10;

let tile_on_x_y = tiles[y*x+x];
```

Means: For each Y Coordinate (height), I need to 'skip' a full row (one times x). Not really good in explaining this, so I won't go any further here.

I made two helper methods in the `GameMap` struct

```rust
pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
    &self.tiles[y * x + x]
}

pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
    &mut self.tiles[y * x + x]
}
```

##### And a few steps back

I this point, I actually found one flaw in my whole design. Even though `Tile` implements `Render`, I simply can't render a tile, because a tile isn't aware of it's position (means: has no coordinates). This makes the whole thing far more challenging.  While sitting here, I literally banged my head onto the desk. 

But the answer is simple. Dirty, yet simple: `GameMap` will implement `Render`.   
Now I just need to expand the `render_all` function to accept my map as an additional parameter, just like the Python tutorial suggests. 

I personally wanted to do this a little bit different, and I failed. But I learned from it.
