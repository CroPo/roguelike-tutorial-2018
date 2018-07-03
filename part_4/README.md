# Part 4: 
## Field of View

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8vp3ya/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/4)

First things first. I don't think I will run into bigger problems in this part, because it's just about using an already
implemented FOV algorithm. 

### The Colors

Just to mention it: I created a `Colors` enum (which has a matching method) and moved it into a own module `color`.

Nothing special, so I rather just link it than paste all the code into this file:

[color.rs](map_objects/color.rs)

Usage is simple:

```rust
use map_objects::color::Colors;

Colors::LightWall.value();
```

The `value` method matches the enum to a `tcod::colors::Color` and returns that value.

### Initialize the FOV

The first thing I needed to do here was giving public access to the `dimensions` of the struct `GameMap`, because the 
FOV Map needs these values.

The rest here was pretty much as the tutorial says. Just create and initialize a `tcod::map::Map`.

