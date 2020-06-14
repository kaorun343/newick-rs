# newick-rs

[![Build Status](https://travis-ci.com/kaorun343/newick.svg?branch=master)](https://travis-ci.com/kaorun343/newick)
[![newick-rs at crates.io](https://img.shields.io/crates/v/newick-rs.svg)](https://crates.io/crates/newick-rs)
[![newick-rs at docs.rs](https://docs.rs/newick-rs/badge.svg)](https://docs.rs/newick-rs)

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
