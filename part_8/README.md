# Part 8: Items and Inverntory

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/8xlo9k/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/7)

I am looking forward to a very interesting tutorial part. I already know roughly where the game is heading with this,
so I thing it might be quite a challenge, yet doable in a reasonable time.

Contents of this Writep:  

1. [Placing Items](#placing-items)


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