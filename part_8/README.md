# Part 8: Items and Inverntory

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/7)

I am looking forward to a very interesting tutorial part. I already know roughly where the game is heading with this,
so I thing it might be quite a challenge, yet doable in a reasonable time.

Contents of this Writep:  

1. [Placing Items](#placing-items)
2. [Picking up Items](#picking-up-items)
3. [The Inventory](#the-inventory)

## Placing Items 

Before I can actually place items, I will need to introduce at least a way to create items. I think I will handle this
with some counterpart to the `CreatureTemplate`. I will probably handle this in a future game much different, but 
for the scope of the tutorial it will do just fine. But much more than just a handful of different items and creatures
aren't handlable with this method. Especiuially when you create item and creature types procedurally.

But before I will start writing some code, let's talk about what an an item _is_:

- Has a name and a description
- If there is trading in the game, it will probably have a specific buy and sell monetary value
- It can be placed both on the map and in an inventory
- Something will happen when they're used

If we rule out the second one, we have a list of all needed `Component`s for this tutorial. 

So let's start by just placing some items on the map, just like the tutorial does. No further `Component`s are needed
to do this. 

I add the (non-blocking) `Position`, `Name` and, of course, the `Render` component. Since the `RenderOrder` for an item
is already in the game I can use that too.

## Picking up Items

The next logical step is to pick up the items. To do this, I need to introdcue two new `Component`s: `Inventory` and `Item`.
`Item` will be kept empty for now - it's just there to mark an `Entity` as item. And `Inventory` will holds a `Vec` of
`EntityId`s, which reference the item entities, and a method to add an item.

When picking up, the `Position` will be removed from the item (so it will be removed from the map), and the 
`Item`'s `EntityId` will be stored in the `Inventory`.

## The Inventory

Now that I can pick up items and show them in my inventory, I need a way to access the Inventory to be able to use items.
So we need a nice menu to show all of the available items.

First of all, I need to change a bit on how I handle `GameState`s. I moved the individual state handling over to the 
`GameState` enum and built an own input key matcher for each state.

Implementing the selection menu was rather easy, too:

```rust
pub fn selection_menu(console: &mut Root, title: &str, options: Vec<String>, width: i32, screen_width: i32, screen_height: i32) {
    let header_height = console.get_height_rect(0, 0, width, screen_height, title);
    let height = header_height + options.len() as i32;
    let mut menu_panel = Offscreen::new(width, height);

    menu_panel.set_default_foreground(colors::WHITE);
    menu_panel.print_rect_ex(0, 0, width, height, BackgroundFlag::None, TextAlignment::Left, title);

    let mut y = header_height;
    let mut letter_index = 'a' as u8;

    for option in options {
        let text = format!("({}) {}", letter_index as char, option);
        menu_panel.print_ex(0, y, BackgroundFlag::None, TextAlignment::Left, text);
        y+=1;
        letter_index+=1;
    }

    let x = screen_width / 2 - width / 2;
    let y = screen_height / 2 - height / 2;

    blit(&menu_panel, (0, 0),
         (width, height),
         console, (x, y),
         1.0, 1.0);
}
```

All seems to work fine, so I will just continue to the next section.

