# Part 1: 
## Drawing the `@` symbol and moving it around

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8s5x5n/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/1)

Basically, I tried to follow the original tutorial step by step, except that I use Rust instead of Python. 

### Major issues

Certainly, the most time consuming issues was simply that I got a bit _lazy_ during the last years of my developer carreer. I am simply used to automatic imports, perfect syntax highlighting, contextual autocomplete, error detection in the code and all that other fancy stuff modern IDEs bring with them. 

That, paired with my practically non-existant experience in Rust, and with the fact that I never used libtcod before (not only the rust bindings - I never used libtcod at all with any language), hence even read the documenation before, made this whole, seemingly trivial task of modifying two variables and printing a character onto the screen a rather time consuming act.

But, thankfully, Rust's compiler is pretty verbose and helpful. It always told me pretty much exactly what I had to do. 


### About the (key) input handling

The input handling was the biggest part I had to re-think. Until that, I just had to look up Rust's equivalent function names on the documentation. But the input handling needed a diverging approach.

The usage of Python's `libtcod.sys_check_for_event(libtcod.EVENT_KEY_PRESS, key, mouse)` was the main cause of problems. In python, this seems to work well - just pass a variable for the mouse and the keuboard input to the function, resulting in these getting set with the event that occured.

In the Rust bindings, we don't have that directly. We get `tcod::input::check_for_event(EventFlags)`, which returns a `Option<(EventFlags, Event)>`. The Event Itself is an Enum, which contains either the key or the mouse event. This means, I would need a mininum of 3 layers of matches to get to get to the information I need. 

The following snipped is what I had in my code, before removing it.

```rust

let event = tcod::input::check_for_event(tcod::input::KEY | tcod::input::MOUSE);

match event {
    Some((.. ,input)) => {
        match input {
            tcod::input::Event::Key(key_state) => {
                // another match here for the Key itself
            },
            _ => ()
        }
    }
    _ => ()
}

```

I resolved this problem by simply using `root.check_for_keypress(tcod::input::KEY_PRESSED)` instead. This does only handle key inputs, but the returned information's complexity is better to work with - especially when the mouse events aren't needed at this moment. And both functions do not wait for the player's input.

The whole code for handling key events is now done by one single match.

One additional note: The flag I used, `tcod::input::KEY_PRESSED`, filters all the key events for those when the key gets pressed. If I wouldn't have done that, I would get at least two events for every key press: One when it's pressed, and one when it gets released again. This could have been filtered in the `match` too, by adding a second parameter to each case. Eeach would have looked like this one then: 
`Some( Key { code: KeyCode::Left, pressed: true, ..} )`


### Responding to the player's input

Just as I had resolved the key handling issue, the Tutorial brought up another. The `handle_key` method returns stuff like `return {'fullscreen': True}`. I literally don't have a clue how this is even called, but it isn't possible in the same form in Rust (at least as far as I know). The nearest language construct may be a Tuple, but even this wouldn't have worked here, because the second value always has at least two different types in the tutorial.

So naturally, I made an Enum which defines all my possible actions, and wrapped it into an Option-Type so I have the possibility of returning no action (the `None`). By defining the Enum, I can losely resemble what the tutorial does, because each Enum value can have it's very Paramtere, too, which do not need to be the same as the other value's ones. And I don't need to scratch on the surface of an undefined state by using a Tuple with a not defined String index. 
