# Part 9: Ranged Scrolls and Targeting

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8zia4r/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/9)

I have quite the feeling that the whole targeting for the second spell might be a bit of a hassle, but let's see how 
I'll handle it in the end. 


Contents of this Writeup:  

1. [Colored Messages](#colored-messages)
2. [Extending the Items and the Inventory](#extending-the-items-and-the-inventory)


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