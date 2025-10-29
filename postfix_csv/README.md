## The Problem

- Create a cli app that parses a csv file, calculates its content
- The spreadsheet should be able to support:
	- [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) calculations
		- Implement only the four most common operators: `+`,`-`,`/` and `*`
	- References to other cells via letter number notation (`A1`, `BZ987` or `AAZ5`)
		- Letters refer to columns, numbers to rows
		- The letter system: `A`,`B`,`C`..`Z`,`AA`,`AB`,`AC`..`AZ`,`BA`,`BB`,`BC`..`ZY`,`ZZ`,`AAA`,`AAB` etc…
		- The coordinates start with 1 _(not zero)_
- When errors are found, the output for that cell should be `#ERR`
- You're not allowed to use any external packages (usage of standard library is allowed)
- Print the output to `stdout`

The spreadsheet coordination system should be `[Col (alphabetical)][Row (numeric)]`

```
| A1 | B1 | C1 |
----------------
| A2 | B2 | C2 |
----------------
| A3 | B3 | C3 |
```

Test csv file:

```
B1 B2 +,2 B2 3 * -,+,26
A1,5,7 2 /,2 20 * 2 / 3 4 + 3 2 * * + 6 - 15 +
C2 3 *,1 B4,5 1 2 + 4 * + 3 -,0.08 6 15 *
5 7 7 - /,67.5 B3 *,-14 A5 +,
```

Which should output:

```
-8,-13,#ERR,26
-8,5,3.5,71
10.5,#ERR,14,#ERR
#ERR,#ERR,#ERR,#ERR
```
