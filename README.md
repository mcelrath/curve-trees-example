# curve-trees-example
Example code using the Curve Trees accumulator

This is an example using the [Curve Trees Accumulator](https://eprint.iacr.org/2022/756) by 
Campanelli, Hall-Andersen, and Kamp implemented [here](https://github.com/simonkamp/curve-trees).

# Running

To run this code, first check out the [curve-trees repository](https://github.com/simonkamp/curve-trees), `cd curve-trees` and check out this repo there. Then edit `Cargo.toml` to add "curve-trees-example" to the `members` block, your `Cargo.toml` should look like this:
```
[workspace]
members = [
    "bulletproofs",
    "relations",
    "curve-trees-example"
]

[workspace.package]
name = "curve_trees"
version = "0.1.0"
```
Now execute:
```
  cargo run
```
