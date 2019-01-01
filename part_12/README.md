# Part 12: Monster And Item Progression

- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/12)

Quite some time has passed since I last did something here. Finally, after I moved amd after a somewhat stressful 
time at work I have some spare time again. 

This means - time for the last 2 pats of this tutorial. First of all, I will look into monster and item progression.

Contents of this Writeup:
1. [Preparations](#preparations)
    1. [Fixing some bugs](#fixing-some-bugs)
    2. [Monster Movement](#monster-movement)
    3. [Dungeon Generator](#dungeon-generator)
2. [Monster and Item progression](#monster-and-item-progression)
    1. [Updating the random generation](#updating-the-random-generation)
    2. [Dungeon Level Scaling](#dungeon-level-scaling)
        
        
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

I ran into bigger mutability-caused problems here, and it took some time to solve it all. To be fully honest at this 
point, the whole architecture at this point is quite a mess.

### Dungeon Generator

Also, there is one major flaw with the dungeon generator at this moment: It can easily become an endless loop if the
generator fails to find space for new rooms. This leads to the game not continuing either on start or when the player
goes down one level.

To address this problem, the loop needs to be controlled with a few new parameters:
1. A minimum of rooms which need to be generated
2. Once these are generated, another counter will be running which counts the failed attempts to create a room, too. If
these exceed a maximum, no ore rooms will be generated and the dungeon will be presented as-is. The counter is reset
for each room generated
3. The maximum number of rooms is still a break condition


## Monster and Item progression

The main topic of this tutorial part is rather easy to follow. I will modify a few functions slightly so they will fit
better to my design.

### Updating the random generation

I'll start out with the method to select one random item from a weighted list:
```rust
pub fn random_choice_index(chances: Vec<i32>) -> usize {
    let mut rng = thread_rng();
    let random_chance = rng.gen_range(1, chances.iter().sum());

    let mut running_sum = 0;
    let mut choice = 0;

    for chance in chances {
        running_sum += chance;
        if random_chance <= running_sum {
            break
        }
        choice+=1;
    }
    choice
}
```
I won't need the second method, because I'll handle creation of the actual `Entity` a bit different. For creatures,
it looks like this:
```rust
pub fn create_random(ecs: &mut Ecs, game_map: &GameMap, pos: (i32, i32)) -> Option<EntityId>  {
    let available_creatures = vec![
        (CreatureTemplate::Orc, 80),
        (CreatureTemplate::Troll, 20),
    ];

    let chances = available_creatures.iter().map(|(_,chance)|{
        *chance
    }).collect();

    let ref selection: (CreatureTemplate, i32) = available_creatures[random_choice_index(chances)];
    selection.0.create_on_position(ecs, game_map, pos)
}

```
The method for creating a random item is pretty much the same.

### Dungeon Level Scaling

The next thing on the list is to make the game more difficult with each new dungeon level. First of all, I will need
to adapt the function which selects a chance by dungeon level:

```rust
pub fn by_dungeon_level(chance_table: Vec<(i32, i32)>, level: i32) -> i32 {
    let mut value_of_level = 0;
    chance_table.iter().for_each(|(value, min_level)|{
        if level >= min_level {
            value_of_level = *value;
        }
    });
    value_of_level
}
```

The rest of this is just a bit fiddling around with the numbers to increase difficulty. Because, as of right now, 
you only need to invest one point into defense to be invulnerable to orc attacks. 