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

### Bumping into monsters

#### Preparing the `Entity`

A few changes to the `Entity` are necessary.  Well, maybe not necessary, because I could already build collision
detection with the data I already have, but a `blocks` flag is still usable, so I wouldn't need to change the whole
codebase if I ever want to include non-blocking entities.

#### Collision Detection

Now, all I need to do is to check for collisions. I am still trying to translate the Python tutorial into Rust, so I need
a static method in my `Entity` struct,

Well, this is the whole method:
```rust
pub fn get_blocking_entities_at(entities: &Vec<Self>, x: i32, y: i32) -> Vec<&Entity> {
    entities.iter().filter(|e| e.blocks && e.pos.0 == x && e.pos.1 == y).collect()
}
```
This is pretty much the point where I really begin to love Rust's incredible intuitive iterators. 

And no, `Vec<&Entity>` is not a typo. We get a Vector of borrowed references here, 
because we can't clone or move, nor do we want that.


#### Putting things together

Now, all that's left to do is putting all things together and check for collision when the player intends to move.
Easier said than done.

The first attempt extending my code may be similar to this:
```rust
//...
    Some(Action::MovePlayer(move_x, move_y)) => {
        let mut player = &mut entities[player_entity_index];
        let destination = ( player.pos.0 + move_x, player.pos.1 + move_y );

        if !map.is_move_blocked(destination.0, destination.1) {
            
            let targets = entity::Entity::get_blocking_entities_at(&entities, destination.0, destination.1);
                           
            if targets.is_empty() {
                fov_recompute = true;
                player.mv((move_x, move_y))
            }
            else {
                
            }
                    
        }
    }
//...
```

But this won't work.

Here, I borrow a mutable reference to a specific `Entitiy` from the vector:
```rust
let mut player = &mut entities[player_entity_index];
```

... and here, I try to passe a immutable borrow to the method:

```rust
let targets = entity::Entity::get_blocking_entities_at(&entities, destination.0, destination.1)
```

and that's not possible in Rust, because you can't borrow anything from a variable during the lifetime of an already
existing borrow.
 
```
error[E0502]: cannot borrow `entities` as immutable because it is also borrowed as mutable
```
 
So I need to change the lifetime of `let mut player` so it doesn't interfer with the other code, yet I need to keep
the part where I get the player's coordinates.

What would happen if I move the player variable into the if clause? Something like that:

```rust
let mut destination = ( entities[player_entity_index].pos.0 + move_x, entities[player_entity_index].pos.1 + move_y );

if !map.is_move_blocked(destination.0, destination.1) {
    let targets = entity::Entity::get_blocking_entities_at(&entities, destination.0, destination.1);

    if targets.is_empty() {
        fov_recompute = true;
        let mut player = &mut entities[player_entity_index];
        player.mv((move_x, move_y))
    } else {}
}
```

Pretty much the same, but the other way round:

```
error[E0502]: cannot borrow `entities` as mutable because it is also borrowed as immutable
```

Because `targets` is a `Vec<&Entitiy>`, which holds borrows from values of `entities`.

Solution? A boolean flag.

```rust
let mut destination = ( entities[player_entity_index].pos.0 + move_x, entities[player_entity_index].pos.1 + move_y );

if !map.is_move_blocked(destination.0, destination.1) {

    let bump_into =
    {
        let mut targets = entity::Entity::get_blocking_entities_at(&entities, destination.0, destination.1);
        !targets.is_empty()
    };

    if !bump_into {
        fov_recompute = true;
        let mut player = &mut entities[player_entity_index];
        player.mv((move_x, move_y))
    }
}

```

This compiles fine, because `targets` lifetime ends when it goes out of scope (the closing bracket). I'm still missing
the message at this point, which is added easily.

### Game States

Not really much to say here. Just did what had to be done.

### Conclusion

Once more, Rusts burrow and move functionality showed me that I need to think ahead when creating something. Of course, I
solved all problems I had with mutability, but the way I solved it will lead to other problems in the future.

Of course, I am following a tutorial, so I am used to just react to what I read. I don't know how long this is going to work
from this point on. 

But with every line I write, I feel more and more confident in Rust. A few weeks ago, it was mostly trial and error when
I did write something (eh, just look at my first week's code here, 'nuff said). Now, especially when working with moves
and borrows, I know that something probably won't work before I code it. Which is good.

The overall code quality led me to one further conclusion:  If I have some spare time within the next week, 
I will either refactor some parts to get a better consistency or I will completely re-write everything up to this point. Reason 
is simple - I have learned so much within the last 3 weeks, and I want a good code base to be able to successfully continue
the tutorial. 