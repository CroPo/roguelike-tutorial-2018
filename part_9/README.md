# Part 9: Ranged Scrolls and Targeting

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8zia4r/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/9)

I have quite the feeling that the whole targeting for the second spell might be a bit of a hassle, but let's see how 
I'll handle it in the end. 


Contents of this Writeup:  

1. [Colored Messages](#colored-messages)
2. [Extending the Items and the Inventory](#extending-the-items-and-the-inventory)
3. [The Lightning Spell](#the-lightning-spell)
4. [The Fireball Spell](#the-fireball-spell)
5. [The Confusion Spell](#the-confusion-spell)

## Colored Messages

Actually, the first thing I will do is adding a `color` value to the messages. I missed that at some point and never 
remembered to implement it afterwards.

Pretty much an easy task, though.

Also, I accidentally commited my testing values for the items (means: many items, nearly no monsters).

## Extending the Items and the Inventory

### First, the Inventory ...

The inventory needs a small update. Right now, a turn passes when I press the _use_ - key of an empty
inventory slot. Which is not exactly what I want to happen. What I could do here is let an `EntityAction` return
the value of a specific `GameState`. This _could_ cause a proble if an `EntityAction` and it's follow-up action
return differen `GameState`s. But I think in this case I will just use what the topmost `EntityAction` returns. If 
`None` resulting `GameState` is found, the one of the follow-up action will be used. This should work recursively.

Right now, the only place where a `GameState` change from an `EntityAction` does even make sense are the use and drop
item actions. These are only called from two `GameState`s, so I will simply ignore the returned state in any other place
until I need it.

I somehow like the ugly, ugly outcome of this:

```rust
let next_state = if let Some(state) = match *self {
    GameState::ShowInventoryDrop => EntityAction::DropItem(ecs.player_entity_id, item_number as u8),
    GameState::ShowInventoryUse => EntityAction::UseItem(ecs.player_entity_id, item_number as u8),
    _ => EntityAction::Idle
}.execute(ecs, log) {
    state
} else {
    GameState::EnemyTurn
};

```

Mostly because I'm curious if I know in a few years/months/weeks/days what this thing does without needing to 
have a head-ache causing thinking session before. But you know what. It doesn't only compile, no, it works, too!
(Just don't ask me to write tests about that. I won't)

### ... and then, the Spells

What I need to implement too is a way to determine if a using an `Item` was successful or not, and by that, if I 
need to consume it or not. I would even go so far to say to even consider a third state, _not successful and no turn
passed_, which would be triggered whenever the player tries to drink a health potion but is already at full health.

But for now, `Success` and `Fail` will do. In this case, `Fail` won't change the `GameState`, since I only have 
the `Heal` spell originating from a potion which just can't be used if the heal fails. I _could_ do this with 
a `bool` too, of course, but I am 100% sure that the upcoming spells _will_ need at least a third state.

I simply will add a `SpellResult` struct here, which can contain a `message` (to preserve funcitonality), and a 
`SpellStatus`, which I just created, featuring the two already names statuses.


## The Lightning Spell

With all the prequisites done, I can now (finally) start to implement the new spells, and the scrolls.

The first one is practically not much different to the `Item` we already have. Instead of healing the player, it only
needs to automatically target the nearest enemy and damage it. Should not be _that_ much of a struggle.

Passing both the fov and the maximum spell range to be used in that layer might be a bit tough. Practically, I can define
the range of the lightning spell when creating the lightning scroll, which means I need to add a parameter to 
`Spell::Lightning`. But getting the fov all the way down... could be rather challenging, but I really don't want
to hit enemies with it which aren't in my fov, so I need to find a solution. Even though I only need it with one `EntityAction`
at the moment, I won't add it as addintional parameter to the `UseItem` value, because this would force me to include
a way to complicated lifetime parameter at this point, because I would need to store a borrowed reference within a enum value.
So I will just add it to the `EntityAction::execute`, which will it pass all the way to the `Spell::cast` method.

I also changed the `TakeDamage` action, so I can use it for all kinds of damage. To re-create the previous behaviour, I 
created a `MeleeAttack`action.

Implementing the `Spell` itself is rather easy then. Just get all targets which are in FOV and check which one is the
nearest one and in range.

```rust
fn find_target(&self, ecs: &Ecs, fov_map: &Map, caster: &Position) -> Option<(EntityId, u8)> {
        let mut distances: Vec<(u8, u8)> = ecs.get_all::<Position>().iter().filter(|(id, p)| {
            **id != caster.entity_id
                && fov_map.is_in_fov(p.position.0, p.position.1)
                && ecs.has_component::<Actor>(**id)
        }).map(|(id, p)| {
            (*id, caster.distance_to(p.position) as u8)
        }).collect();

        distances.sort_by(|a, b| {
            a.1.cmp(&b.1)
        });

        if let Some(d) = distances.first() {
            Some(d.clone())
        } else {
            None
        }
    }
```

First I filter for all valid targets (not the caster, in fov and `Actors`), then I sort them by distance and then I
return the nearest one.

What's now left to do is randomly placing some scrolls on the map.

## The Fireball Spell

For the next Spell, I will need to make targeting by the mouse cursor happen. I will need a new `GameState` which will 
let me select a target, and I will need to trigger that state from within a `Spell`. After target selection, I will
need to continue the `Spell` calculation.

Let's begin with the `Spell` and add an Item so I can trigger the target selection.

A `Fireball` has a damage and radius value, as all the `Actors` around the impact will be affected by some colletaral 
damage (I will include the player to that, so one needs to carefully select a target).  No range, because every target 
in the fov should be targetable. 

This was the easy part (and nothing does really happen by now). The not-that-easy part comes now: Triggering a new `GameState`
to get a target. I will just focus on the actual targeing first, and then I will see how I continue to cast the spell
and actually let the fireball explode on the desired location. To trigger this from the Spell itself, I add a new `SpellStatus`: `Targeting`. 
From the `EntityAction`, this will trigger the `GameState`.

Triggering the `Targeting` game state works fine. Now I need to pass the selected entity back to the `Spell`. I need to
extend `SpellStatus::Targeting` first with a new value, the instance of the `Spell` itself, so I know to which `Spell` I 
need to pass `Entity`.  Also, I will add a `cast_on_target` method which I can call then on the `Spell`.   
Of course, the `Targeting` state needs to know of the `Spell` too, otherwise it can't work.

Since the `Fireball` spell can hit more than one targets, and therefor produce more than one `TakeDamage` actions, both
a `Spell` and an `EntityAction` can now trigger multiple reactions.

Casting the `Spell` works now. But the item stays in the player's pocket. So that's the next I will need to take care of.
To achieve that, I add a new value to the `Fireball` which is the `EntityId` of the used scroll.

## The Confusion Spell

This one is always funny. My implementation will be that the `Confusion` spell will change the `MonsterAi`s target to
the nearest other ai-controlled `Actor`. First of all, I need to change how the `MonsterAi` selects the target. Right
now, the player's `EntityId` is hardcoded. So I need to add some value here. Since I can't be sure that do have already
created a player instance, I add a `Option<EntityId>`. Since I already  need an `EntityAction` to switch the target for the `Spell`,
I will just use that here, too. Before every `EnemyTurn` I will update every `MonsterAi` which has no target.

I might find a cleaner solution for that at some point, but for now it will be sufficient (I know, I say this
often). Since the _AI_ here is rather dumb, I don't need to switch targets that often, so I'm fine with this solution. In
a bigger game, the AI might do much more than A* and hit (hopefully).

Creating the `Spell` and placing scrolls on the map is all I need to do now. Since I already have all parts I need, 
this won't be to much of a task at this point.