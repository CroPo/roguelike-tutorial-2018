# Part 12: Monster And Item Progression

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/12)

Quite some time has passed since I last did something here. Finally, after I moved amd after a somewhat stressful 
time at work I have some spare time again. 

This means - time for the last 2 pats of this tutorial. First of all, I will look into monster and item progression.

Contents of this Writeup:
1. [Preparations](#preparations)
    1. [Fixing some bugs](#fixing-some-bugs)
    2. [Monster Movement](#monster-movement)
        
## Preparations

### Fixing some bugs

First of all, some bugs need to be fixed. At least one is bugging me ( :-) ) for a while now: Corpses are blocking 
monster movement. 

Reason is simple - `is_blocked_by` does not check if an entity is dead, it just checks the `is_blocking` property.

```rust
pub fn is_blocked_by(ecs: &Ecs, position: (i32, i32)) -> Vec<EntityId> {
    ecs.get_all::<Self>().iter().filter(|(_, p)| {
        p.position.0 == position.0 && p.position.1 == position.1 && p.is_blocking
    }).map(|(i, _)| *i).collect()
}
``` 

Another filter has to be added to filter out those entities which are dead `Actor`s:

```rust
pub fn is_blocked_by(ecs: &Ecs, position: (i32, i32)) -> Vec<EntityId> {
    ecs.get_all::<Self>().iter().filter(|(entity_id, p)| {
        let is_blocking = p.position.0 == position.0 && p.position.1 == position.1 && p.is_blocking;
        if let Some(a) = ecs.get_component::<Actor>(**entity_id) {
            is_blocking && !a.is_dead()
        } else {
            is_blocking
        }
    }).map(|(i, _)| *i).collect()
}
```

Of course, the A* calculation needs to handle the dead `Actor`s correctly, too.

### Monster Movement

The next issue I want to solve is the general monster movement. The `MonsterAi` always knows the exact location of the player,
and will start chasing said `Entity` from across the whole dungeon in the moment the layer sets foot onto a dungeon floor.

Due to simplicity, I will solve this by giving each monster an own FOV. Once they see the player they will start chasing,
even if the player is no longer in the FOV. The latter is done due to simplicity. In fact, this is the most easy way to
handle this. Theoretically, the `MonsterAi` could remember the last known position of the player and move to (or near),
and could look for a specific threshold of turns in a specific area if the player can be found again, to name just one
example of better AI behavior. Maybe I will look into that later, since I personally like enemy behaviour development. A lot. 

But this is not today's concern. I want to do two concrete things: 

First, I want to give monsters the ability to see and chase the player. Again, I am choosing the most simple solution
here, again: Once the monster spotted the player, it will start chasing until one of both is dead. Also, each entity
will have the same FOV radius.

Secondly, I want monsters to only do _anything_ (including looking for the player) if they are within a specific 
distance to the player. This should help to increase performance, since the fov doesnt need to be updated for every
entity in every turn.

