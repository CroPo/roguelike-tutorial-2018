# Part 6: 
## Doing (and taking) some damage

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/6)

Contents of this Writep:  
_Hint: You can actually skip the first one. It's just me struggling with everything_

1. [Fighters and AI](#fighters-and-ai)
2. [Managing the Entities](#managing-the-entities)
    1. [The ID Generator](#the-id-generatpr)
    2. [Wrapping it up](#wrapping-it-up)
    3. [Re-implement ... everything!](#re-implementing-...-everything!)
    4. [Getting rid of `Vec<Entity>`](#getting-rid-of-vec<entity>)

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

This is, of course, just my personal preference. I like to split up things in small files. Maybe because I do Java
for a living for some time now.

Next thing will be implementing the `EntityManager` itself. For now, I think this will be sufficient:

```rust
pub struct EntityManager {
    id_generator : IdGenerator,
    entities: HashMap<EntityId, Entity>,
    player_id : EntityId,   
}
```

The `IdGenerator`, a map for all entities, indexed by an `EntityId`, and the `EntityId` which refers to the `Entity` which
holds the player character information.

The first step on a long path is done.

#### Re-Implementing ... everything!
##### (One Entity at a time, though)

Yes. I need to re-implement all stuff which has anything to do with `Entities`, or, to be precise, everything that accesses 
the `Vec<Entity>`.  
Which is: `Entity` creation, player movement, rendering, ... Pretty much half of the game it is.

First of all, I need a method to add a new Entity. I want to only tell the `EntityManager` which kind of creature I 
want to create - right now, we can have a Player, and Orc or a Troll. The `EntityManager` itself then needs to take care
of which character or color to use for drawing, what name it should get, and other stuff. All I want to tell the 
`EntityManager` is _what_ creature should be placed _where_ on the Map.

The 'problem' is that the `new` Method of a creature is already pretty big in terms of parameters. So I shorten that by
only passing two parameters (type, and position), and let the manager select the correct values from a template. This
could be utilized to read monsters from configuration files or from a database in the future. But right now, I will
only use three predefined creature types from a `enum`:

```rust
pub enum CreatureTemplate {
    Player,
    Troll,
    Orc
}
```

The enum holds a public `create` method which creates an instance of the selected template. I won't paste the whole
implementation details in here, but you can look them up in the [file](src/entities/creature.rs).

Right now, only the player creature templates is implemented. Trying to create `Troll` or an `Orc` will result in `None`.

I also added the `is_player` method, which will (obviously) return true if the value of the template is `Player`. This 
method has 2 uses: First, I can tell which `EntityId` I need to save in`player_id`, and secondly, it helps me to prevent
more than one player entity from being added.

Back in the `EntityManager`, I added a method to create creatures from a template at a specific position, and two methods
to get the player `Entity`, one immutable, the other one for mutab;e access. 

Now is the time to actually put the `EntityManger` into use: I replace every player `Entity` access with it, which means
I have to change the `GameMap.make_map`, too. Following this strategy, replacing any occurrence of `Vec<Entity>` at a time,
while still maintaining compilability on the code,  I will slowly, bur steady, come to the point where I get rid of the
`Vec<Entity>` 

#### Getting rid of Vec<Entity>

Before I can actually improve anything on the `Entity` handling, I need to remove the `Vec<Entity>` entirely. Right now,
it's still used for collision detection, creation, and, of course, rendering. 

I won't change much of the procedure itself - All monsters will still be created when the map is generated, rendering will
still happen in the `render_all` function. Means: I will change _how_ it's done, not _where_ it's done.

As first step, I will change `make_map`, so that all creatures will be created with the `EntityManager` from a template. 
Of course, I need to implement both the `Orc` and the `Troll` Template first.

It took some time, but in the end I removed the old `Vec<Entity>`. Of course, at this point I didn't do any change or
optimization at all, I just replaced old code with some new code. The whole optimization comes now. 