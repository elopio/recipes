#![cfg_attr(not(feature = "std"), no_std)]
use babe::SameAuthoritiesForever;

fn main() {
    let a = SameAuthoritiesForever;
}


// Compilation originally failed with
// error[E0554]: `#![feature]` may not be used on the stable release channel
// I beat that level with `rustup override set nightly` learned at
// https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
