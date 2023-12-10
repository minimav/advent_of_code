## Advent of code

* 2021 was done in Python and setup in a slightly different way.
* 2023 is a different language each day.

Commands below apply to years other than those two years.

Setup a new puzzle to solve in Rust for an existing year:

```bash
make year=2022 create
```

or just the latest year:

```bash
make create
```

Run unit tests for a puzzle:

```bash
make year=2022 puzzle=4 test
```

Run a puzzle:

```bash
make year=2020 puzzle=15 run
```
