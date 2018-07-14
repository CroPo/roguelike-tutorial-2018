# Part 6:  Doing (and taking) some damage

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
    5. [Dissecting the Entity](#dissecting-the-entity)
    6. [Storing the components away](#storing-the-components-away)
    7. [Creating the component storage](#creating-the-component-storage)
    8. [The Position component](#the-position-component)

First things first: I wanted to do some optimizations between last week and this week. But, exactly as i feared, I didn't
really have any spare minute, so I was only able to do two bugfixes:

1. Using a offscreen console to blit it onto the root console.  
I didn't get the whole point of this at the beginning, and misunderstood how python handles the root console. It fixed
a nasty display bug when I executed the game in windows, too
2. Only render NPCs which are in my FOV  
Title says everything. The Linux version of the game rendered all NPCs on top of the background (even when not in FOV),
so this was necessary.

Finally, it's the day to introduce some action to the game. So let's do this.

## Fighters and AI

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

## Managing the Entities

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

### The ID Generatpr

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

### Wrapping it up

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

### Re-Implementing ... everything!
#### (One Entity at a time, though)

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
implementation details in here, but you can look them up in the [file](src/ecs/creature.rs).

Right now, only the player creature templates is implemented. Trying to create `Troll` or an `Orc` will result in `None`.

I also added the `is_player` method, which will (obviously) return true if the value of the template is `Player`. This 
method has 2 uses: First, I can tell which `EntityId` I need to save in`player_id`, and secondly, it helps me to prevent
more than one player entity from being added.

Back in the `EntityManager`, I added a method to create creatures from a template at a specific position, and two methods
to get the player `Entity`, one immutable, the other one for mutable access. 

Now is the time to actually put the `EntityManger` into use: I replace every player `Entity` access with it, which means
I have to change the `GameMap.make_map`, too. Following this strategy, replacing any occurrence of `Vec<Entity>` at a time,
while still maintaining compilability on the code,  I will slowly, bur steady, come to the point where I get rid of the
`Vec<Entity>` 

### Getting rid of Vec<Entity>

Before I can actually improve anything on the `Entity` handling, I need to remove the `Vec<Entity>` entirely. Right now,
it's still used for collision detection, creation, and, of course, rendering. 

I won't change much of the procedure itself - All monsters will still be created when the map is generated, rendering will
still happen in the `render_all` function. Means: I will change _how_ it's done, not _where_ it's done.

As first step, I will change `make_map`, so that all creatures will be created with the `EntityManager` from a template. 
Of course, I need to implement both the `Orc` and the `Troll` Template first.

It took some time, but in the end I removed the old `Vec<Entity>`. Of course, at this point I didn't do any change or
optimization at all, I just replaced old code with some new code. The whole optimization comes now. 

### Dissecting the Entity

Now it's time to dissect the `Entity` itself. Right now, the struct looks like this:

```rust
pub struct Entity {
    pub pos: (i32, i32),
    glyph: char,
    color: colors::Color,
    pub name: String,
    blocks: bool,
}
```

We have just all needed data scrambled into one `struct` right now. All of these values are needed by every `Entity` which
is in the game right now. And naturally, I would put pretty much every possible value right which I ever going to need
right into this struct, expanding it incrementally. Since the title of this part of the tutorial is 'Doing (and taking)
some damage', the next few things added will surely be values for tracking the HP of a creature, and maybe some combat 
stats. This is the point where I might run into problems with Rust.

1. While doing a computer controlled `Entity`s move in a loop, I can't get access to modify another `Entity`s values.  
Because I can't get mutable access to a Collections Element when already iterating over it in any form, being a `iter()`
or `iter_mut()`.  
Of course, this might be solved by a queueing up all actions which should be done, and afterwards looping over the queue
to actually perform all this actions. But, especially when calculaitng an `Entity`'s movement, it could happen that some
actions interfer with each other, so I would need to double and triple check everything. Which I won't do.

2. Way more information than ever needed would be created (In technical terms: Allocate more memory than needed). This 
may not sound that much of a problem at this point. We only have some creatures moving around, every value here is
actually used. But the struct is called `Entity`, and not `Creature`. For example, a weapon. As long as it is lying on
the floor, it shares all the values any `Entity` has now. But once picked up by another `Entity`, it doesn't need to
have a position anymore, but an owner. I could go do far and set the position simply to `(-1, -1)`, and define this as
_not on the map_, and create a `owner` value, which is a `Option<Entity>`, means it will be `None` for all creatures 
per default. But I don't want to have a list of 10 `Option` values from which nine are always `None`. 

So, we need a way of composing an `Entity` of several components. I won't go as far as calling th thing I intend to 
creat an _entity component system_, but it will be some `Entity` which can be built with several `Component`s.

Of course, this describes what is widely known as an ECS. So I am going to tinker together my own one.

#### Storing the components away

In order to achieve this, especially the separation of an `Entity` and it's values so I can access each independent of
the other, I can't store these right in the `Entity` struct. I will instead add another `HashMap` to the `EntityManager`.
Each `Entity`'s components will be just linked to the `Entity` with it's ID. 

In best case, each `Component` would be able to interact with _any_ `Component` of _any_ `Entity` (including the owner
Entity itself). For example, the tutorial mentions an `Ai` component, which controls all actions, including the movement, 
of one `Entity`, and therefor needs to access the owner's position to change it. While this is not directly possible 
from within the `Component`s method itself, I can utilize some kind of action queue in small scale.

This means: On each `Entity`s turn, all actions are calculated first, and then executed right afterwards before the 
next `Entity` is calculated. This is compliant with Rust's value acces.

#### Creating the component storage

The storage will be a simple `HashMap`, but storing (and, of course, accessing) the components will be a much more 
challenging task which needs some Tinkering with Rust. Or, needed. In this case I already worked on solving this 
tasks for a few hours. 

First step was, of course, renaming all the stuff (once more). `entities::` became `ecs::`. `EntityManager` became
`Ecs`.

Now I need to expand the `Ecs` struct. For now, I will keep the `entities` with their current value, but I need another
value which holds all components for each entity. This value needs to hold both the `EntityId`, and all the stored
components with another identifier.

My implementation looks like this:
```rust
trait Component {}

struct EcsStorage {
    entity_id: EntityId,
    data: HashMap<TypeId, Box<Component>>
}

pub struct Ecs {
    id_generator: IdGenerator,
    player_entity_id: EntityId,
    
    storage: HashMap<EntityId, EcsStorage>,
    
    pub entities: HashMap<EntityId, Entity>,
}
```
And, to be fully Honest upfront: Most of the knowledge here was gathered through several articles and tutorials I did
read within the last week.

The `EcsStorage` itself is the struct which will hold each component for one Entity. Just to make sure I do not regret
not adding the `EntityId` afterwards, I will iclude it for now. Removing a unused value is way more easy than adding a 
needed one.

`TypeId` is a special type of Rust. It provides an unique identifier for a `struct` or `trait`. And we need to `Box`
the `Component`, because at compile time, the size of `Component` is not defined, as it's a `trait`, which can have
different sizes (more or less values of different length) for each `struct` implementing the `trait`.

Of course, this way any `struct` which implements the `Any` trait will have `'static` lifetime, but I don't think it 
will affect the game too much in a bad way. And this is pretty much the most simple way to do it.

The next thing to do is change the initialization. And henever I create a new `Entity`, I need to allocate a storage
entry for it, too.

All that's left to do for this inital step is to add some methods to `Ecs` to register components for a specific `Entity`:
```rust
impl ComponentStorage {
    fn register<T>(&mut self, component: T) where T: Component {
        self.data.insert(TypeId::of::<T>(), Box::new(component));
    }
}

impl Ecs {
    //...
    fn register_component<T>(&mut self, entity_id: EntityId, component: T)
     where T: Component {
     self.storage.get_mut(&entity_id)
         .map(|storage| storage.add(component) );
    }
    //..
}
```

#### The Position component

Finally, the system is at a stage where I am able to remove values from the `Entity` and put them into a `Component`. The
First one I will move are the `position`, and the `blocks` values. Both are only needed, when an `Entity` is placed 
somewhere on the Map.

First of all, the creature templates will need to be modified: The values which don't exist anymore will be removed, and
the `Positon` component will be added directly in the `add_creature` method of `Ecs`. This will be temporarily,
as I will change the way how the templates will be used later. 

Since the `add_creature` method also allows to directly set a creatures position, I will need a method to access a specific 
`Component` of any `Enity`. Because I used the `TypeId` to identify a `Component`, this is as simple as using `get` on 
`storage`. But since this method cpould be used form outside the `Ecs`, too, I will wrap it into another method:

```rust
impl ComponentStorage {
    //..
    fn get<T>(&self) -> Option<&T>
        where T: Component + Any {
        self.data.get(&TypeId::of::<T>()).map(|e| e.downcast_ref()).unwrap()
    }
    //..
}
```
Some explanaiton here: I use the `map` because every `Component` is stored in a `Box`. And the `downcast_ref` converts
the `Component` type to the actual type of `T`.

```rust
impl Ecs {
    //...
    pub fn get_component<T>(&self, entity_id: EntityId) -> Option<&T>
        where T: Component + Any {

        self.storage.get(&entity_id).map(|storage| {
            storage.get::<T>().unwrap()
        })
    }
    //..
}
```
In Both cases, the us of `unwrap` is potentially unsafe. Means: If the result of the get is `None`, this will cause 
the game to panic (==crash). If needed, I will get to that again.

Next, all methods which did access the `pos` value of an `Entity` need a way to access the same information again. To
achieve this, I create the following method:

```rust
impl Ecs {
    //...
    pub fn get_all<T: Component + Any>(&self) -> HashMap<EntityId, &T> 
    where T: Component {
        let entity_ids = self.storage.keys().cloned();

        let mut component_map = HashMap::new();

        for entity_id in entity_ids.filter(|id| {
            self.has_component::<T>(*id)
        }) {
            component_map.insert(entity_id, self.get_component::<T>(entity_id).unwrap());
        }
        component_map
    }


    pub fn has_component<T>(&self, entity_id: EntityId) -> bool 
    where T: Component {
        let is_registered = self.storage.get(&entity_id)
            .map(|storage| storage.is_registered::<T>());

        if is_registered.is_some() {
            is_registered.unwrap()
        }
        false
    }
    //..
}
```

The second methd just checks a spcific `Entity` for a component. The Method `is_registered` doesn't do enything besides
executing `contains_key()` on the `HashMap` in `EcsStorage`.

Now I can replace any access to an `Entity`s positon. Keep in mind that these replacements are mostlyu temporary now, 
because I will move some methods, and access to the player entity will be changed at some point, too, so I just temporarily
made the acccess to the `player_entity_id` public.

With all these changes I made, everything compiles again. But it's far form what I actually had before. The next thing
I need to do is changing the rendering, so that everything will be rendered again.