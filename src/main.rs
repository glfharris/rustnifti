
extern crate time;
use time::PreciseTime;

extern crate num;
use num::iter::range;

extern crate nalgebra;
use nalgebra::{DMat, DVec};


mod signals;
mod numeric;
use std::f32;
use std::iter::Map;
use numeric::function::{Domain, Eval};
use numeric::polynomial::Polynomial;
use numeric::interpolate::Interpolate;


fn main() {
    let start = PreciseTime::now(); // Leave in place for testing & debugging
    println!("Output:\n");



    let end = PreciseTime::now(); // Leave in place for testing & debigging
    println!("\nTime Elapsed: {} seconds", start.to(end));
}
