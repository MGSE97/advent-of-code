[< Back to day](../README.md)

---

# --- Day 1: Trebuchet?! ---

Source: [Advent of Code](https://adventofcode.com/2023/day/1)

## Solution for Part 1

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all **fifty stars** by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants **one star**. Good luck!

You try to ask why they can't just use a [weather machine](https://adventofcode.com/2015/day/1) ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a [trebuchet](https://en.wikipedia.org/wiki/Trebuchet) ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been **amended** by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific **calibration value** that the Elves now need to recover. On each line, the calibration value can be found by combining the **first digit** and the **last digit** (in that order) to form a single **two-digit number**.

For example:

```text
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

In this example, the calibration values of these four lines are `12`, `38`, `15`, and `77`. Adding these together produces **`142`**.

Consider your entire calibration document. **What is the sum of all of the calibration values?**

### Task

- Elves needs us to parse calibration document.
- Each line contains calibration value.
- Calibration value consits of 2 digits combined together. (eg. `1` and `2` became `12`)
- First digit is from start of line.
- Second digit is from end of line.

### Edge cases

- There can be mutliple numbers on the line.
- If there is only one number, it is both first and last. (eg. `1` became `11`)

### Solution

1. First we load calibration file.
https://github.com/MGSE97/advent-of-code/blob/4fac38fae44539297c829797e2e54216af0bf961/2023/1/src/part1.rs#L35-L37
2. Then we read it line by line.
https://github.com/MGSE97/advent-of-code/blob/4fac38fae44539297c829797e2e54216af0bf961/2023/1/src/part1.rs#L43-L45
3. For each line ve create character iterator, that keeps only digits.
https://github.com/MGSE97/advent-of-code/blob/4fac38fae44539297c829797e2e54216af0bf961/2023/1/src/part1.rs#L48
4. Now we're almost done, we take first and last item from iterator.
https://github.com/MGSE97/advent-of-code/blob/4fac38fae44539297c829797e2e54216af0bf961/2023/1/src/part1.rs#L49-L50
5. And combine it to number. After that we add it to global sum.
https://github.com/MGSE97/advent-of-code/blob/4fac38fae44539297c829797e2e54216af0bf961/2023/1/src/part1.rs#L53

### Results

Q: What is the sum of all of the calibration values?

A: Puzzle answer was `55002`

Program output:
<https://github.com/MGSE97/advent-of-code/blob/2918fa25acf79566ecb38cb8c4bc50ab8215e90d/2023/1/data/output.1.txt#L1-L2>

---

<h6 align="center">

[< Back to day](../README.md)
•>&nbsp; Part 1 &nbsp;<•
&nbsp; [Part 2](../Solution.2.md) &nbsp; •
[Continue to part 2 >](../Solution.2.md)

</h6>

<h6 align="center">

<b><a href="https://github.com/MGSE97" target="_blank">MGSE</a> ☕ 2023</b>

</h6>
