[< Back to day](./README.md)

---

# --- Day 1: Historian Hysteria ---

Source: [Advent of Code](https://adventofcode.com/2024/day/1)

## Solution for Part 1

The **Chief Historian** is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior Historians has asked you to accompany them as they check the places they think he was most likely to visit.

As each location is checked, they will mark it on their list with a **star**. They figure the Chief Historian **must** be in one of the first fifty places they'll look, so in order to save Christmas, you need to help them get **fifty stars** on their list before Santa takes off on December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants **one star**. Good luck!

You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations to check is currently **empty**. Eventually, someone decides that the best place to check first would be the Chief Historian's office.

Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?

Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number called the **location ID**. To make sure they don't miss anything, The Historians split into two groups, each searching the office and trying to create their own complete list of location IDs.

There's just one problem: by holding the two lists up **side by side** (your puzzle input), it quickly becomes clear that the lists aren't very similar. Maybe you can help The Historians reconcile their lists?

For example:

```
3   4
4   3
2   5
1   3
3   9
3   3
```

Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the **smallest number in the left list** with the **smallest number in the right list**, then the **second-smallest left number** with the **second-smallest right number**, and so on.

Within each pair, figure out **how far apart** the two numbers are; you'll need to **add up all of those distances**. For example, if you pair up a `3` from the left list with a `7` from the right list, the distance apart is `4`; if you pair up a `9` with a `3`, the distance apart is `6`.

In the example list above, the pairs and distances would be as follows:

- The smallest number in the left list is `1`, and the smallest number in the right list is `3`. The distance between them is **`2`**.
- The second-smallest number in the left list is `2`, and the second-smallest number in the right list is another `3`. The distance between them is **`1`**.
- The third-smallest number in both lists is `3`, so the distance between them is **`0`**.
- The next numbers to pair up are `3` and `4`, a distance of **`1`**.
- The fifth-smallest numbers in each list are `3` and `5`, a distance of **`2`**.
- Finally, the largest number in the left list is `4`, while the largest number in the right list is `9`; these are a distance **`5`** apart.

To find the **total distance** between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is `2 + 1 + 0 + 1 + 2 + 5`, a total distance of **`11`**!

Your actual left and right lists contain many location IDs. **What is the total distance between your lists?**

### Task

- Parse input file into two lists of numbers
- Sort these list ascending
- Compute distance between each (e.g. `a-b`)
- Return sum of these distances

### Edge cases

- Value form first list could be less than from other. In this case, distance could be negative, witch is wrong.
- Both lists needs to be same size.

### Solution

1. Loaded whole file into string variable.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L14>
2. Split it into lines.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L22>
3. Parsed each line into integer `a` and `b`
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L24>
4. Added these integers into lists.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L26-L29>
5. Sorted each of these lists.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L33-L34>
6. Got each element from lists and computed distance.
   If it was negative, it was inverted using `Abs` function.
   Resulting distance was added to current one.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L37-L41>
7. Returned resulting distance
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part1/solution.go#L43>

### Result

Q: What is the total distance between your lists?

A: Its **3714264**.

Program output:
<https://github.com/MGSE97/advent-of-code/blob/f177db212291637a3635580da5fe465e16997735/2024/1/data/part_1.out.txt#L1-L9>

---

<h6 align="center">

[< Back to day](./README.md)
•>&nbsp; Part 1 &nbsp;<•
&nbsp; [Part 2](./Solution.2.md) &nbsp; •
[Continue to part 2 >](./Solution.2.md)

</h6>

<h6 align="center">

<b><a href="https://github.com/MGSE97" target="_blank">MGSE</a> ☕ 2024</b>

</h6>
