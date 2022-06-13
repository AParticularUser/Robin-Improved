#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_macros)]

mod robin;

#[skyline::main(name = "robin_improved")]
pub fn main() {
    robin::install();
}