#![feature(generic_const_exprs)]

mod blif;
mod simulation;

fn main() {
    blif::parser::parse("");
}
