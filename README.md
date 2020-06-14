# newick-rs

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
