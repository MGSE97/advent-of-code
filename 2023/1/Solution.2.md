[< Back to day](../README.md)

---

# --- Day 1: Trebuchet?! ---

Source: [Advent of Code](https://adventofcode.com/2023/day/1)

## Solution for Part 2

Your calculation isn't quite right. It looks like some of the digits are actually **spelled out with letters**: `one`, `two`, `three`, `four`, `five`, `six`, `seven`, `eight`, and `nine` **also** count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

```text
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

In this example, the calibration values are `29`, `83`, `13`, `24`, `42`, `14`, and `76`. Adding these together produces **`281`**.

**What is the sum of all of the calibration values?**

### Task

- We have to update [part 1](./Solution.1.md) to parse text as numbers.
- Same rules as before applies here.

### Edge cases

- Text numbers can are somewhere in the line.
- They don't have to be valid. (eg. character is missing or different)
- They can overlap. (eg. `twone` became `2` if we read from start or `1` if we read from back)

### Solution

1. We add string to numer mapping map.
https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L34-L45
2. Then we update our parsing to not remove non digit characters.
3. After that, we create function to parse text into digits from start to end.
  - We add character to current number buffer, that is created from previous function run.
    (eg. `o` to `tw` becames `two`)
    https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L105
  - Then we try to match this combined characters to any part of number keys.
    (eg. `tw` is start of number `two`)
    https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L109
  - If it matches whole key, we have our number.
    https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L111-L116
  - If not, we add it to buffer.
    https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L117-L122
  - If there is no key starting with buffer, we keep removing characters, until it matches, or buffer is empty.
    (eg. `xtw` is trimmed to `tw`)
    https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L124-L128
5. And create simillar function for reverse
https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L137-L169
7. Parse first digit from start
https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L65-L71
8. Parse second digit from end
https://github.com/MGSE97/advent-of-code/blob/eb57e7a6a180ae7d2621bb874d67eda6e98a4b19/2023/1/src/part2.rs#L73-L81
9. Finaly, compute result as before.

### Results

Q: What is the sum of all of the calibration values?

A: Puzzle answer was `55093`.

Program output:
<https://github.com/MGSE97/advent-of-code/blob/2918fa25acf79566ecb38cb8c4bc50ab8215e90d/2023/1/data/output.2.txt#L1-L2>

---

<h6 align="center">

[< Back to day](../README.md)
• &nbsp; [Part 1](./Solution.1.md) &nbsp;
•>&nbsp; Part 2 &nbsp;<•
[Continue to advent calendar >](../README.md)

</h6>

<h6 align="center">

<b><a href="https://github.com/MGSE97" target="_blank">MGSE</a> ☕ 2023</b>

</h6>
