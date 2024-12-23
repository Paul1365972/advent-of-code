# Advent of Code 2024

First Advent of Code I am participating in.

All problems were solved without external help (AI, Forums, etc.), with the exception of the Rust documentation.

## Day 1/2/3

Total: ~ 100 min

Caught up the first three doors, the solutions are implemented in Rust with the goal of just getting the answer as quickly as possible.

## Day 4

Part 1/2: 30 min

Finally wrote code for part 1 that is more easily adaptable for any part 2.

## Day 5

Part 1: 25 min, Part 2: 3 min

The solution is awful time complexity wise O(n^2 + m), but was really easy to quickly implement!

## Day 6

Part 1: 25 min, Part 2: 10 min

## Day 7

Part 1: 15 min, Part 2: 1 min

## Day 8

Part 1: 25 min, Part 2: 5 min

## Day 9

Part 1: 15 min, Part 2: 45 min

I thought I had a nice solution for Part 2, but debugging it was hell.
In the end I just used the obvious suboptimal, but at least working, algorithm.

## Day 10

Part 1: 15 min, Part 2: 1 min

Wow that part 2 was suprisingly simple, turn a single Set into a List and done.

## Day 11

Part 1: 5 min, Part 2: 5 min

## Day 12

Part 1: 10 min, Part 2: 40 min

## Day 13

Part 1: 30 min, Part 2: 5 min

Parsing took waaay to long, all that time and it's not even pretty or performant.

Part 2 was predictable, but exciting.
Had to check to make sure that solutions are unique, but who would have thought that the linear algebra classes would be useful someday?!

## Day 14

Part 1: 15 min, Part 2: ? min

Parsing went way better this time around, but I forgot that Rust's unsigned types use remainder and not modulus, that took some time to find out.

Part 2 was so different that I nearly wanted to check each image by hand, but luckily my other idea with RLE worked out.
One of my favorite problems yet, although sadly also the first problem for which I needed external assistance.

## Day 15

Part 1: 25 min, Part 2: 20 min

## Day 16

Part 1: 35 min, Part 2: 10 min

I initally wanted to use A*, but I am not sure if there even is a good heuristic for this problem, so Dijkstra's it is.

Part 2 was easier than expected, I was prepared for the worst.

## Day 17

Part 1: 25 min, Part 2: ~150 min

Tried the naive brute force approach for part 2 and checked in the range of 0 to 2^32-1, but after finally analyzing the instructions is became very clear this would never work.

Also the second problem where I had to ask for help.

## Day 18

Part 1: 20 min, Part 2: 5 min

I very much expected the second problem to be pathfinding with the twist that the map changes every step, but just having to check until a path is blocked was kind of disappointing.

## Day  19

Part 1: 10 min, Part 2: 3 min

## Day  20

Part 1: 20 min, Part 2: 5 min

## Day 21

Part 1: 25 min, Part 2: Unsolved

First problem that is too difficult to solve, I now have a (hopefully working) solution using dynamic programming in my head, but I have already spent too much time on this problem.

## Day 22

Part 1: 8 min, Part 2: 10 min

## Day 23

Part 1: 10 min, Part 2: 10 min

First Advent of Code challenge I stayed awake for, 6 am is an awful time to solve these problems, luckily this one was really easy!
