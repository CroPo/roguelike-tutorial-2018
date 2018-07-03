# Part 5: 
## Placing Enemies and kicking them (harmlessly)

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8vp3ya/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/5)

I completely agree with the tutorial: What good is a dungeon with no monsters to bash? So let's do this.

### Placing entities on the map

Doing this is simple. I just follow the tutorial here again. 

This statement here kept me thinking a bit:
```python
if not any([entity for entity in entities if entity.x == x and entity.y == y])
```

... until I realized it just ensures that the game doesn't place an entity on top of another one.

Translated to  Rust, this same statement looks like this:

```rust
if !entities.iter().any(|&e| e.pos.0 == x && e.pos.1 == y)
```

Of course, this is not the only existing solution, but it's short and should be quite fast if I'm not wrong. 

I could use a for loop which pretty much does the same thing, too. This would be obviously a bit longer, and also, 
I can't modify a collection while iterating through it. The Rust compiler would simply refuse to compile then. So I 
would need an `occupied` flag or similar.

For completion, this would would look something like this:

```rust
let mut occupied = false;

for e in entities.iter() {
    if e.pos.0 == x && e.pos.1 == y {
        occupied = true;
        break;
    }
}
if !occupied {
    // add monster
}

```

So, monsters are beeing generated. Of course, FOV handling is still missing, so every monster is visible all the time.

