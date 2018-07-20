# Part 6:  Creating the interface

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/7)

Contents of this Writep:  

1. [The health bar](#the-health-bar)
2. [In the meanwhile](#in-the-meanwhile)
3. [Message display](#message-display)

This part seems to be one of the shorter and easier ones. I am not _that_ diassapointed by that fact, because the last 
one actually took me more than a week to finish. Not much more to say at that point, let's just get started!

## The health bar

A few adjustments to the configuration variables, and I have enough place left on the application to put the new stuff to. 
The method to render a health bar wan't that hard, either:

```rust
pub fn render_bar(panel: &mut Offscreen, pos: (i32, i32), width: i32, name: &str, value: i32, max: i32, bar_color: Color, back_color: Color) {
    let filled_width = (value as f64 / max as f64  * width as f64).round() as i32;

    panel.set_default_background(back_color);
    panel.rect(pos.0, pos.1, width, 1, false, BackgroundFlag::Screen);

    if filled_width > 0 {
        panel.set_default_background(bar_color);
        panel.rect(pos.0, pos.1, filled_width, 1, false, BackgroundFlag::Screen)
    }

    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(pos.0 + width / 2, pos.1, BackgroundFlag::None,
                   TextAlignment::Center, format!("{}: {}/{}", name, value, max));
}
```

Nothing more to say about that I guess.

## In the meanwhile

Before I start the next section, I did a few things:

- Renamed the `Creature` to `Actor`, because its more fitting.
- Adressed a bug which caused a panic (crash) of the game when trying to load a not registered `Component`.
- In the `Die` action, the `Actor` component no longer gets removed. This is for showing health bars corectly even when 
the actor is already dead. 

## Message display

Until right now, all in-game messages are sent to stdout, because I always use the `println!` macro. In this section,
I will implement the the MessageLog GUI panel. The only thing I don't like about the tutorial here is that there
is no separation between the gui and the data here. Which means I will make a few changes, so my code won't completely 
resemble the Python counterpart. 

Practically, my `MessageLog` will be lacking all of it's position and size values. The `MessageLog` is only here to 
store the messages. The rendering needs to decide how many messages will be displayed.

The `MessageLog` will directly be accessed by the `EntityAction`'s `execute` method. Because each action could
trigger a `Message`. I don't really want any other things generating messages by now.

To be able to let my `EntityAction`s return both a `reaction` and a `message`, I have to modify the `EntityAction` 
slightly, and intrduce a `ActionResult` struct. I also added a method for quickly creating an empty result.

```rust
struct ActionResult {
    reaction: Option<EntityAction>,
    message: Option<Message>,
}

impl ActionResult {
    pub fn none() -> ActionResult {
        ActionResult {
            reaction: None,
            message: None,
        }
    }    
}
```

This means I have to work over every action I have defined so far. Which aren't that much, gladly. Since most of the
[actions.rc](src/ecs/action.rs) was changed I won't post everything right into this writeup, but you can follow
the link to the file to look at what I did.

Every message is now put into the log. This means all I need to do now is writing the messages to a specific place
on the bottom panel. To actually make sure that the text will fit into the panel, I included the 
[textwrap](https://github.com/mgeisler/textwrap) crate, as counterpart of the same named Python library.

To display the log in the gui, I created a `MessagePanel` struct, which places the log in specific coordinates in
a `Console`. Since the `MessagePanel` log always hold a reference to the `MessageLog`, and the `MessageLog` needs to
be used mutable on several other places wile the immutable reference still exists, I wrap the `Vec` which holds 
the individual `Message`s into a `RefCell`. Doing that, I can cretae mutable references to the `Vec` while accessing
the `MessageLog` through an immutable reference.

```rust
pub struct MessageLog {
    messages: RefCell<Vec<Message>>
}

impl MessageLog {
    pub fn new() -> MessageLog {
        MessageLog {
            messages: RefCell::new(vec![])
        }
    }

    pub fn add(&self, message: Message) {
        self.messages.borrow_mut().push(message);
    }

    pub fn messages(&self) -> Ref<Vec<Message>> {
        self.messages.borrow()
    }
}
```

The `render` method of the `MessagePanel` struct fetches all messages and iterates until it reached the maximum number
of lines.

```rust
pub struct MessagePanel {
    pos: (i32, i32),
    dimensions: (i32, i32),
    log: Rc<MessageLog>,
}

impl MessagePanel {
    pub fn new(pos: (i32, i32), dimensions: (i32, i32), log: Rc<MessageLog>) -> MessagePanel {
        MessagePanel {
            pos,
            dimensions,
            log,
        }
    }

    pub fn render(&self, panel: &mut Offscreen) {
        let mut total_lines = 0;

        'l: for m in self.log.messages().iter().rev() {
            let lines = wrap(&m.text, self.dimensions.0 as usize);

            panel.set_default_foreground(m.color);

            for l in lines {
                panel.print_ex(self.pos.0, self.pos.1+total_lines, BackgroundFlag::None, TextAlignment::Left, l.to_string());
                total_lines += 1;
                if self.pos.1 + total_lines > self.dimensions.1 {
                    break 'l;
                }
            }
        };
    }
}
```
