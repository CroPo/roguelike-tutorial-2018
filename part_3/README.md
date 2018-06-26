# Part 3: 
## Generating a dungeon

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8twiwa/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/3)

Hopefully, I learned from my mistakes in [part_2](../part_2). We will see, I guess.

### Creating Rooms with a Rectangles

The first instructions from the tutorial were pretty clear so far, so just a quick mention that I actually _did_ it - remove all the wall tiles and create a `Rect`(angle) struct. Implementing `create_room` was pretty easy either, so I just go to the next step of the tutorial. After all, that's just what I needed right now after the somewhat intense last part.

In addition to the instructions, I moved `GameMap` and `Tile` into the `map_objects` module/folder, same as the Python counterpart did right from the start. I kind of missed that part.

