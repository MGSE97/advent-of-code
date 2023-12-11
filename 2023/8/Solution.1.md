[< Back to day](./README.md)

---

# --- Day 8: Haunted Wasteland ---

Source: [Advent of Code](https://adventofcode.com/2023/day/8)

## Solution for Part 1

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about **ghosts** a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of **network** of labeled nodes.

It seems like you're meant to use the **left/right** instructions to **navigate the network**. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: `AAA` and `ZZZ`. You feel like `AAA` is where you are now, and you have to follow the left/right instructions until you reach `ZZZ`.

This format defines each **node** of the network individually. For example:

```text
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
```

Starting with `AAA`, you need to **look up the next element** based on the next left/right instruction in your input. In this example, start with `AAA` and go **right** (`R`) by choosing the right element of `AAA`, `**CCC**`. Then, `L` means to choose the **left** element of `CCC`, `**ZZZ**`. By following the left/right instructions, you reach `ZZZ` in `**2**` steps.

Of course, you might not find `ZZZ` right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: `RL` really means `RLRLRLRLRLRLRLRL...` and so on. For example, here is a situation that takes `**6**` steps to reach `ZZZ`:

```text
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
```

Starting at `AAA`, follow the left/right instructions. **How many steps are required to reach `ZZZ`?**

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
Permalink_to_part_1_result_file

---

<h6 align="center">

[< Back to day](./README.md)
•>&nbsp; Part 1 &nbsp;<•
&nbsp; [Part 2](./Solution.2.md) &nbsp; •
[Continue to part 2 >](./Solution.2.md)

</h6>

<h6 align="center">

<b><a href="https://github.com/MGSE97" target="_blank">MGSE</a> ☕ 2023</b>

</h6>
