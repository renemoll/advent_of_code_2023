

# Plan of attack

1. Solve the puzzles as naively as possible;
1. Benchmark execution time and memory usage;
1. Apply lessons learnt along the way;
1. Optimize until I reach an acceptable total solution time.

In the meantime, I will be going through the Rust book and (hopefully) improving my skills along the way.

# Puzzels

## Day 1

I started to learn Rust again after giving it a try about a year ago. Meaning this solution is not the prettiest. But, with all the parsing in place, it works. My first solution was based on an inverted search, replacing each written-out number with the numerical value. Which, of course, broke with combined sequences so I ended up with word-for-word replacement.

## Day 2

I enjoy the first few puzzles. They are easy to solve, meaning I can focus on getting back into the language and figuring out basic input parsing.

## Day 3

I decided to represent the map as a matrix in memory, hence I had to write a `struct` with some methods. I like the creational pattern. 
Next to the map, I store the symbols with their location to find the adjacent numbers instead of finding the numbers and then searching for a symbol. Based on the hunch that there would be fewer symbols to find compared to the amount of numbers and associated boundary boxes to search through.

## Day 4

Part one was solved quickly, once I got the parsing working. While I like the method-chaining, I am not convinced this is the nicest/readable way to parse the input. I might get back to that. The challenge was the administration of the copies. Looking back, I think a standard container would have been nicer compared to manually shifting a fixed-sized array.

## Day 5

I decided to parse the input into Ranges and then naively follow the mapping functions. Of course, this takes a long time for the second part. It might be faster to combine the various maps into a single transformation. I did read about searching backwards using the smallest location which I find interesting to see how that would work.

## Day 6

An easy one, except I had to wrestle a bit with the parsing. I think the solution can be written more elegantly, which I will leave for later on.

## Day 7

I figured calculating the kind of hand was doable based on the occurrence of each kind of card, as there are no flushes in this game. Combining the hand with the score into a single number was copied from `maneatingape`, very effective.

## Day 8

The first part was, of course, easy. For the second part, I attempted to re-use my naive solution from the first part and traverse the map until all keys indicated an end. After roughly half an hour, I aborted the brute force implementation. Realizing I only needed one solution for each path (cycle) and then determining when all paths coincide made for a quick solution.