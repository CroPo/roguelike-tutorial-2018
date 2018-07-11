# Part 6: 
## Doing (and taking) some damage

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/6)

1. [Fighters and AI](#fighters-and-ai)
2. [Managing the Entities](#managing-the-entities)
    1. [The ID Generator](#the-id-generatpr)

First things first: I wanted to do some optimizations between last week and this week. But, exactly as i feared, I didn't
really have any spare minute, so I was only able to do two bugfixes:

1. Using a offscreen console to blit it onto the root console.  
I didn't get the whole point of this at the beginning, and misunderstood how python handles the root console. It fixed
a nasty display bug when I executed the game in windows, too
2. Only render NPCs which are in my FOV  
Title says everything. The Linux version of the game rendered all NPCs on top of the background (even when not in FOV),
so this was necessary.

Finally, it's the day to introduce some action to the game. So let's do this.

### Fighters and AI

One big problem here: I don't know if it's technically possible to create a reference to a components owner in Rust, 
because of the mutability rules. I will just continue without it and the we'll see how it goes.

To expand the `Entity` trait with ai and fighter, I used the `Option` enum for both. The enum either has `Some` value 
or `None`, which resembles what is `null` in other languages while still preventing me from using actual null pointer
(and therefor prevent null pointer errors/exceptions)

So, my addition to the `Entity` looks like this:

```rust
pub fighter: Option<Fighter>,
pub ai: Option<Box<Ai>>
```
Boxing the AI is necessary, as AI is just a trait, which is not sized, and `Option` can only be used with sized types. For
the ai parameter in the `new` method, I can either pass `None`, or `Some(Box::new(..)))`.

The adapted enemy turn handling looks like this now:
```rust
for e in entities.iter().filter(|e| e.ai.is_some()) {
    e.ai.as_ref().unwrap().take_turn();
}
```
Instead of just skipping the player's entity, I filter for all entities which have some AI. That's why I can use `unwrap()`
without having to fear to run into a panic - because I won't have any entity with `None` AI.

Sooner or later, I need to find a solution for referencing the `Entity` from within the `Ai` (or the `Fighter`). It wouldn't 
be that hard if I only need a immutable borrow of the Entity. If I need a mutable, though, it can't work. 

The thing is - the whole component structure is built on Python's features, which support that behavior. If I plan to use
composition like this in Rust, I will have to go a bit of a different way.

I think I will conclude this section of part 6 here. 

### Managing the Entities

After a 3 hours straight session of trial and error, one thing seems very clear to me at this point: I need to rebuild
how Entities are stored and accessed. Entirely. Too many problems have occured to this point, and even if I find a
workaround for the most recent problems, it probably won't be long until I run into the next couple of blockers.

So, I will redo the whole `Entity` handling. I don't want to diverge too much form the Tutorial itself, too.

Summed up, I will build something like this:

- A manager structure which holds the entities and provides some methods to access and alter those.
- Splitting up an Entity's properties to multiple Lists if needed, so that I can iterate over one while having mutable 
access to another one.
- To achieve this, every Entity needs to get an unique ID. I first thought of using an UUID here, but that would 
be a bet over-engineered for this use case. An internal counter which gets updated for every added Entity will pretty
much do the same thing here. I don't use threading atm, so I don't have to take care of that too.

#### The ID Generatpr

A simple task, because I don't care what happens if there are multiple instances of the generator. Of course, from
a technical POV, I _could_ create as many instances as I want, with each instance holding it's own internal counter, 
so this could lead to ID collisions. But I simply know I will only use it in the entity manager struct, which I will
only use once, too.

The id will be a `u8`, because I really doubt I will ever get more than 256 entities at once during this 
Tutorial.

```rust
type EntityId = u8;
``` 

Just to make sure I don't have to change half of the code again if I should run out of IDs, I simply declare a 
type here

So, if I ever feel like having over 65,000 entities at once (or, if I _really_ want to try out UUIDs) - 
all I need to do is changing the actual type of the `EntityId`.

```rust
struct IdGenerator {
    id: EntityId
}

impl IdGenerator {
    fn new() -> IdGenerator {
        IdGenerator {
            id: 0,
        }
    }

    /// Generate a new ID
    fn get_next_id(&mut self) -> EntityId {
        self.id+=1;
        self.id
    }
}
```

A simple `struct` with a simple implementation. Using it this way, the first generated id will be 1, because `get_next_id` 
increments the id by one before returning.

#### Wrapping it up

As I mentioned before, I want to wrap all the `Entity` related stuff into a struct to handle them. I decided to name it
`EntityManager`, because that's what it actually does.

To keep an overview of everything, I will move all entity related stuff to a new directory, and split it up into several 
files. 

```
/src/entities
    - mod.rs
    - id.rs
```

`mod.rs` holds both the `Entity` and the `EntityManager` (for now), and the `id.rs` contains everything related to the 
`EntityId`