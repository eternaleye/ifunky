Usage:

Cargo.toml:
```toml
[dependencies]
ifunky = "0.1"
```

Your code:
```rust
#[macro_use]
extern crate ifunky;

ifunky! {
    // Declare your function signature
    pub fn foo(x: u32) -> u32 {
        // And write a dispatcher that will be
        // invoked the first time the function is called
        if rand::random::<bool>() {
            foo_big as fn(u32) -> u32
        } else {
            foo_bigger as fn(u32) -> u32
        }
    }

    // That's it!

    pub fn bar(x: u32) -> u32 {
        if rand::random::<bool>() {
            bar_small as fn(u32) -> u32
        } else {
            bar_smaller as fn(u32) -> u32
        }
    }
}

fn main() {
    foo(3);
    bar(7);
}

fn foo_big(x: u32) -> u32 {
    x + 1
}

fn foo_bigger(x: u32) -> u32 {
    (x + 1) * 2
}

fn bar_small(x: u32) -> u32 {
    x - 1
}

fn bar_smaller(x: u32) -> u32 {
    (x - 1) / 2
}
```
