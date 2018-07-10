# Part 6: 
## Doing (and taking) some damage

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/6)

First things first: I wanted to do some optimizations between last week and this week. But, exactly as i feared, I didn't
really have any spare minute, so I was only able to do two bugfixes:

1. Using a offscreen console to blit it onto the root console.  
I didn't get the whole point of this at the beginning, and misunderstood how python handles the root console. It fixed
a nasty display bug when I executed the game in windows, too
1. Only render NPCs which are in my FOV  
Title says everything. The Linux version of the game rendered all NPCs on top of the background (even when not in FOV),
so this was necessary.

Finally, it's the day to introduce some action to the game. So let's do this.


