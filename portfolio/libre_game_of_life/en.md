# libre_game_of_life

> The _Game of Life_ is a cellular automaton devised by the british mathematician John Horton Conway
> in 1970

## Rules

_Game of Life_ automaton occurs on a grid where each cell can be either **dead** or **alive**. At each time step, what determines the state of each cell are the following rules:

- **Alive cells** survive with 2 or 3 alive neighbors
- **Dead cells** become alive with 3 alive neighbors

## My implementation

I implemented a version of _Game of Life_ in _Rust_, called **libre_game_of_life**. It is an _Free Software (GNU AGPL v3.0)_, and you can [download it here!](https://github.com/joao-arthur/libre_game_of_life) 

```rust
let grid = HashMap::from([
    (Coordinate::from(0, 1), State::Alive),
    (Coordinate::from(-4, 2), State::Alive),
    (Coordinate::from(7, 3), State::Alive),
]);
```

> In the code above, the implementation of the automaton grid. A _HashSet_ would also work **in this scenario**, because only alive cells are considered. However, some variations of _Game of Life_ have more states

When using a _HashMap_, we decouple the **grid** and the **render**, allowing us to render any part of the grid, zooming in and out, without affecting the current state.

Another aspect is that, we need some **coordinate system** to identify a specific position. The most simple is the **cartesian plane**, which is the one used.

### Iterating

To create the next generation, the current _HashMap_ is traversed, and for each alive cell and its neighbors, the rules are applied. The resulting alive cells are saved in a new _HashMap_.

### The application

I implemented _Game of Life_ on the web, rendered on a _canvas_. The project has three layers:

### lib

A reusable, generic implementation of _Game of Life_ in _Rust_.

### web_backend

A _Web Assembly_ application that works as a bridge between _lib_ and _web_frontend_.

### web_frontend

The user application, responsible for render the canvas, the settings, and init the _Web Assembly_. Currently, the settings are the following:

- **Preset:** Allows the user to choose from many popular shapes (Glider, Blinker, etc.) 
- **Gap:** A visual option, for aesthetics
- **Size:** The amount of zoom in the screen
- **FPS:** The desired FPS for render

[Click here to try it!](/libre_game_of_life/index.html)

## Reference

https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
