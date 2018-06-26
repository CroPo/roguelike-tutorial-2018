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


