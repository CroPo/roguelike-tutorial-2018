# Part 10: Saving and Loading

- [Reddit Post on /r/roguelikedev](https://www.reddit.com/r/roguelikedev/comments/91do0i/roguelikedev_does_the_complete_roguelike_tutorial/)
- [Direct Tutorial Link](http://rogueliketutorials.com/libtcod/10)

So, we're starting with loading and saving the game here. This time, I can just loosely resemble what the Python
tutorial does due to the obvious difference of both languages. Since I already have done a part of what the tutorial
wanted me to do - namely move all the game logic to another class - I don't expect much problems at all.

Contents of this Writeup:
1. [Settings](#settings)

## Settings

First thing to do is move alle the used coonfiguration values, like `screen_width`, which are spread across the `main.rs`
file into a own `struct`. It's not that hard, but it will take some time to do this. 

Since the configuration is read only, I don't need to mind about mutable acces being a problem.

Since it is pretty big in size with no logic, I won't put the `struct` here directly, but you can always view the source
[here](src/settings.rs). Practically, it's just a `struct` which holds all configuration `values`, a `new` method which
initializes the default values and a getter for each value.

For my next step, I simply remove all the values from the `main()` method, and resolve all resulting errors. Wherever 
it makes sense I will pass the whole `Settings` as a reference instead of each value individually. Even though this 
will remove the lose binding I now have, it will help me a bit because I won't have to pass more values than my screen 
can display in a line any more.

With these changes, the whole code looks quite a bit cleaner. Well, at least somewhat cleaner.