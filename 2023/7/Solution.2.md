[< Back to day](./README.md)

---

# --- Day 7: Camel Cards ---

Source: [Advent of Code](https://adventofcode.com/2023/day/7)

## Solution for Part 2

To make things a little more interesting, the Elf introduces one additional rule. Now, `J` cards are [jokers](<https://en.wikipedia.org/wiki/Joker**(playing**card)>) - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, **`J` cards are now the weakest** individual cards, weaker even than `2`. The other cards stay in the same order: `A`, `K`, `Q`, `T`, `9`, `8`, `7`, `6`, `5`, `4`, `3`, `2`, `J`.

`J` cards can pretend to be whatever card is best for the purpose of determining hand type; for example, `QJJQ2` is now considered **four of a kind**. However, for the purpose of breaking ties between two hands of the same type, `J` is always treated as `J`, not the card it's pretending to be: `JKKK2` is weaker than `QQQQ2` because `J` is weaker than `Q`.

Now, the above example goes very differently:

```text
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
```

- `32T3K` is still the only **one pair**; it doesn't contain any jokers, so its strength doesn't increase.
- `KK677` is now the only **two pair**, making it the second-weakest hand.
- `T55J5`, `KTJJT`, and `QQQJA` are now all **four of a kind**! `T55J5` gets rank 3, `QQQJA` gets rank 4, and `KTJJT` gets rank 5.

With the new joker rule, the total winnings in this example are **`5905`**.

Using the new joker rule, find the rank of every hand in your set. **What are the new total winnings?**

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