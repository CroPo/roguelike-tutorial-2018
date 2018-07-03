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

### Compute the FOV

At this point of the tutorial, I can't really see why I need to wrap the `tcod::map::Map::compute_fov` method into a 
function which doesn't really provide anything put that wrapper, because I can't make use of optional function parameters.

But I will just follow along here, because that way I already have the function in an extra module, which helps me if I
ever going to do further encapsulation of the fov stuff.

Nothing much to do here, so I go on with the next section.

### Extend the rendering

Of course, the next logical step is to extend the render method, so I can actually make use of the calculated FOV

And now comes the time where I _really_ start to hate myself for the decision to create a `Render` trait. The solution
is quite simple: I will remove it entirely from the `GameMap` struct and will just have a `render` method there, which
will support additional parameters.

I pretty much have lost any form of consistent code, so I am not to unhappy about going that way.

And, because I needed to do the `blit` a bit different, I had to change the render_all bit. I actually needed to remove 
the offscreen console I once created, which I did because I could not blit the root console onto itself (because of mutability).

I also did some cleanup work, too. The whole thing should compile now with no errors.

### Map Exploration

The last section is simply making the map explorable. The `Tile` struct gets a new flag, and only explored tiles will be rendered.

### Conclusion

This step was done pretty fast, imo. No real problems encountered, just the ones I made by myself. 