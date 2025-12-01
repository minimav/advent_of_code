## Advent of code

* 2021 was done in Python and setup in a slightly different way.
* 2022 was in Rust
* 2023 was a different language each day.
* 2024 was done in Go
* 2025 was in Rust

Commands below apply to Rust years.

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
