# Part 3: 
## Generating a dungeon

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8twiwa/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/3)

Hopefully, I learned from my mistakes in [part_2](../part_2). We will see, I guess.

### Creating Rooms with a Rectangles

The first instructions from the tutorial were pretty clear so far, so just a quick mention that I actually _did_ it - remove all the wall tiles and create a `Rect`(angle) struct. Implementing `create_room` was pretty easy either, so I just go to the next step of the tutorial. After all, that's just what I needed right now after the somewhat intense last part.

In addition to the instructions, I moved `GameMap` and `Tile` into the `map_objects` module/folder, same as the Python counterpart did right from the start. I kind of missed that part.

### Tunnel Connection

The next step connected both the created rooms with each other. Nothing much to say here atm.

### Actual Generation

With all the preparations made, it's finally time to create a randomized dungeon. 

The start was pretty easy to follow, by just expanding the `Rect` with some methods to calculate the center and to check for intersections with another `Rect`. I didn't check both methods any further, I just trust the author of the Python tutorial here.

I got the room generation loot wrong at some point, so I worked around it on myself, and, somehow, it seems to work. 

No further problems were encountered


### Conclusion

Yes, this part of the tutorial went pretty well for me. Nothing more to say here.