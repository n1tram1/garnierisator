#![feature(generic_const_exprs)]

mod blif;

fn main() {
    blif::parser::parse("");
}