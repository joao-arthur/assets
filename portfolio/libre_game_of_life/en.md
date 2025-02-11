# libre_game_of_life

> The Game of Life is a cellular automaton devised by the British mathematician John Horton Conway
> in 1970

This automaton occurs on a grid where each cell can be either dead or alive. At each time step, the
group of living cells is determined by the following rules:

- Living cells survive with 2 or 3 living neighbors
- Dead cells become alive with 3 living neighbors

## Implementation

### The Array Approach

The idea of this approach is to store the state of **every** cell.

```ts
const grid = [
    [0, 1, 0],
    [1, 1, 1],
    [0, 1, 0],
];
```

In the case of a square grid, the **number** of items in the array will be equal to the **square**
of the grid's **length**:

- 10x10 grid = 100 items
- 100x100 grid = 10.000 items
- 300x300 grid = 90.000 items

> This implies that memory usage grows faster than the grid width.

#### The Visual Problem

If all cells are confined **only within** the grid boundaries, they become **stuck** at the edges.
For instance, a glider on the edge **becomes a block**.

![glider transforms into block](/images/glider_to_block.gif)

#### The Performance Problem

Creating a new generation requires creating a new array of the same size as the current one.

You can choose between:

- Iterating over all items in the current array
  - **Advantage:** Simplicity
  - **Disadvantage:** Slower performance with larger grids
- Mapping the positions of alive cells and iterating over their neighbors
  - **Advantage:** Improved performance with fewer alive cells
  - **Disadvantage:** More complex to implement

### The Map Approach

The idea of this approach is to store only the state of the **living** cells.

```ts
const grid = new Map([
    ["(0, 1)", 1],
    ["(-4, 2)", 1],
    ["(7, -3)", 1],
]);
```

> A _Set_ would also suffice **in this scenario**, since a living cell has only one state

An important change is that, with a _Map_, we can no longer rely on array indices. Instead, we need
to use a **coordinate system** identify cell positions. The most simple is the **cartesian plane**.

#### The Visual Solution

Given that the coordinate system is **infinite**, the UI works like a **magnifying glass**:
Revealing only a small part of the landscape, with the ability to move around and zoom in and out.
The limitation at the edges no longer exists:

![glider slipping away](/images/glider_away.gif)

#### The Performance Solution

To create a new generation, a new _Map_ must be created by iterating over the living cells and their
neighborhoods. This operation is independent of the UI size and therefore, **much faster**.

## Final Application

[Click here to see!](/libre_game_of_life/index.html)
