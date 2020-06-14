# newick-rs

[![Build Status](https://travis-ci.com/kaorun343/newick.svg?branch=master)](https://travis-ci.com/kaorun343/newick)

## Usage

```rust
extern crate newick_rs;

fn main() {
    let input_text = "(A,B)";
    let input_tree = newick_rs::from_newick(text);

    // some logics

    let output_text = newick_rs::to_newick(&output_tree);
}

```
