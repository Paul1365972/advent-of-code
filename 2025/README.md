# Advent of Code 2024

This is my second time participlating in Advent of Code, this year again using Rust.

All problems were solved without external help (AI, Forums, etc.), with the exception of the Rust documentation.

## Day 1

Part 1: 30 min
Part 2: 15 min

## Day 2

Part 1: 30 min
Part 2: 10 min

Tried to go for the most efficient algorithm immediately, could have saved some time via a more brute force approach, but am happy with my solution.

## Day 3

Part 1: 10 min
Part 2: 10 min

Easiest day yet, simply because there were no edge cases to account for.
Although my functional approach to find the maximum is pretty cursed and unfortunate.

## Day 4

Part 1: 10 min
Part 2: 5 min

Also quite an easy puzzle.
I really enjoy the ones based on 2d grids, though I am always a bit unsatisfied with the parsing approaches I tend to use for thses kinds of problems.

## Day 5

Part 1: 5 min
Part 2: 10 min

Again a suprisingly easy puzzle; now I wonder why the first two days took so long again.
I could (should) have used an interval tree instead of my O(n^2) solution, but from the input size it was obviously not needed, so I just chose the approach that was fastest/easiest to implement.

## Day 6

Part 1: 10 min
Part 2: 15 min

I dislike the "puzzles" like today that are just about parsing the input, those are always a bit annoying to deal with.

## Day 7

Part 1: 10 min
Part 2: 5 min

Awsome puzzle, first solution for Part 1 was already so good, that part 2 only needed minmal changes.
Anticipating what the next part might require you to do and then having correctly predicted it always feels nice.

## Day 8

Part 1: ?
Part 2: ?

Another fun one, this time I used `petgraph` to simplify some of the work.
You most likely wouldn't even need to use a Union-Find datastructure, but I simultaneously realized that it is most likely the optimal solution for this puzzle so I could not not use it.
Yet I didn't want to write it myself as it is quite cumbersome.

## Day 9

Part 1: 10 min
Part 2: 40 min

The most fun puzzle yet.
I have to reiterate how enjoyable these 2d problems are, especially part 2.
There are most likely tons of valid optimization strategies, but I instantly thought of compressing the distinct x and y values such that our rectangle search can skip nearly all tiles, really proud of that one.
