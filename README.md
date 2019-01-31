# game_of_life
An implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life) in rust.
Made just for fun and as a little exercise to learn rust.

## How to run
game_of_life offers two command line arguments: filename and fps.
filename can be the name of any of the map.txt files provided in this repo, or any map files you created yourself.
fps is optional and limits the animation speed. fps can range from 0(renders the initial state, then stops) to u32 max(limited by the applications performance and your hardware).

## Creating custom map files
Create a text file in an editor of your choice, the first line will be the width of your map, the second line will be the height.
The following lines will be filled with 0 and 1. 0 is a dead cell, 1 is a living cell. Whitespaces for readability are allowed.
Make sure the count of 1s and 0s in each line matches the width and the count of rows matches the height.
