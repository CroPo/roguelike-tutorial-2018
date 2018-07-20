# Part 6:  Creating the interface

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/7)

Contents of this Writep:  

1. [The health bar](#the-health-bar)

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
