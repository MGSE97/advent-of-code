[< Back to day](./README.md)

---

# --- Day 9: Mirage Maintenance ---

Source: [Advent of Code](https://adventofcode.com/2023/day/9)

## Solution for Part 2

Of course, it would be nice to have **even more history** included in your report. Surely it's safe to just **extrapolate backwards** as well, right?

For each history, repeat the process of finding differences until the sequence of differences is entirely zero. Then, rather than adding a zero to the end and filling in the next values of each previous sequence, you should instead add a zero to the **beginning** of your sequence of zeroes, then fill in new **first** values for each previous sequence.

In particular, here is what the third example history looks like when extrapolating back in time:

```text
5  10  13  16  21  30  45
  5   3   3   5   9  15
   -2   0   2   4   6
      2   2   2   2
        0   0   0
```

Adding the new values on the left side of each sequence from bottom to top eventually reveals the new left-most history value: **`5`**.

Doing this for the remaining example data above results in previous values of **`-3`** for the first history and **`0`** for the second history. Adding all three new values together produces **`2`**.

Analyze your OASIS report again, this time extrapolating the **previous** value for each history. **What is the sum of these extrapolated values?**

### Task

- Describe what we need to do.

### Edge cases

- Describe edge cases, that can cause issues.

### Solution

- Describe each step how we solved it.
  Perma_link_to_code_part

### Result

Q: Question

A: Answer

Program output:
Permalink_to_part_2_result_file

---

<h6 align="center">

[< Back to day](./README.md)
• &nbsp; [Part 1](./Solution.1.md) &nbsp;
•>&nbsp; Part 2 &nbsp;<•
[Continue to advent calendar >](../README.md)

</h6>

<h6 align="center">

<b><a href="https://github.com/MGSE97" target="_blank">MGSE</a> ☕ 2023</b>

</h6>
