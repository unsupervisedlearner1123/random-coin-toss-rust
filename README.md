## Simple Rust tool to return a random coin toss outcome

This is a simple Rust program that simulates a coin toss and returns either "Heads" or "Tails". It was created as part of a programming exercise for a Rust course.

### Usage
Add the following line to your Cargo.toml file to include this library as a dependency:

```
rand = "0.8.4"
```

Use the following code to return a random coin toss outcome:

```
extern crate week4_rust;
use week4_rust::coin_toss;

fn main() {
    println!("Coin toss outcome: {}", coin_toss());
}
```

### Contribute
If you would like to contribute to this repository, please fork the repository, make your changes, and create a pull request.
