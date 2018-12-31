# Part 12: Monster And Item Progression

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/12)

Quite some time has passed since I last did something here. Finally, after I moved amd after a somewhat stressful 
time at work I have some spare time again. 

This means - time for the last 2 pats of this tutorial. First of all, I will look into monster and item progression.

Contents of this Writeup:
1. [Preparations](#preparations)
    1. [Fixing some bugs](#fixing-some-bugs)
        
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