[< Back to day](./README.md)

---

# --- Day 1: Historian Hysteria ---

Source: [Advent of Code](https://adventofcode.com/2024/day/1)

## Solution for Part 2

Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.

Or are they?

The Historians can't agree on which group made the mistakes **or** how to read most of the Chief's handwriting, but in the commotion you notice an interesting detail: a lot of location IDs appear in both lists! Maybe the other numbers aren't location IDs at all but rather misinterpreted handwriting.

This time, you'll need to figure out exactly how often each number from the left list appears in the right list. Calculate a total **similarity score** by adding up each number in the left list after multiplying it by the number of times that number appears in the right list.

Here are the same example lists again:

```
3   4
4   3
2   5
1   3
3   9
3   3
```

For these example lists, here is the process of finding the similarity score:

- The first number in the left list is `3`. It appears in the right list three times, so the similarity score increases by `3 * 3 =`**`9`**.
- The second number in the left list is `4`. It appears in the right list once, so the similarity score increases by `4 * 1 =`**`4`**.
- The third number in the left list is `2`. It does not appear in the right list, so the similarity score does not increase (`2 * 0 = 0`).
- The fourth number, `1`, also does not appear in the right list.
- The fifth number, `3`, appears in the right list three times; the similarity score increases by **`9`**.
- The last number, `3`, appears in the right list three times; the similarity score again increases by **`9`**.

So, for these example lists, the similarity score at the end of this process is **`31`** (`9 + 4 + 0 + 0 + 9 + 9`).

Once again consider your left and right lists. **What is their similarity score?**

### Task

- Parse input file into two lists of numbers
- Sort these list ascending
- Compute distance between each (e.g. `a-b`)
- Return sum of these distances

### Edge cases

- Value form first list could be missing in similarity map.

### Solution

1. Loaded whole file into string variable.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L13>
2. Split it into lines.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L21>
3. Parsed each line into integer `a` and `b`
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L23>
4. Added integer `a` into `listA`.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L26>
5. Updated `similarity` map using integer `b` as key.
   If it existed in map, its value was increased by 1 in map.
   Otherwise, key was added with value `1`.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L29-L34>
6. Got each element from `listA`, its similarity and computed distance.
   Resulting distance was added to current one.
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L39-L43>
7. Returned resulting distance
   <https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/src/part2/solution.go#L45>

### Result

Q: What is their similarity score?

A: Its **18805872**.

Program output:
<https://github.com/MGSE97/advent-of-code/blob/af1a060c8d752be6e8ddd351efba9e3e026d2ed0/2024/1/data/part_2.out.txt#L1-L9>

---

<h6 align="center">

[< Back to day](./README.md)
• &nbsp; [Part 1](./Solution.1.md) &nbsp;
•>&nbsp; Part 2 &nbsp;<•
[Continue to advent calendar >](../README.md)

</h6>

<h6 align="center">

<b><a href="https://github.com/MGSE97" target="_blank">MGSE</a> ☕ 2024</b>

</h6>
