#![allow(dead_code)]
use numeric::function::{Eval, Domain};
use std::f32;

/// Struct represent a Piecewise function. Which is a function that is defined by different functions in subdomains.
#[derive(Default, Debug)]
pub struct Piecewise<T> {
    subdomains: Vec<T>,
}

impl<T> Piecewise<T> {
    pub fn new(v: Vec<T>) -> Piecewise<T> {
        Piecewise{ subdomains: v }
    }
    pub fn add_sub(&mut self, func: T) {
        self.subdomains.push(func);
    }
}

impl<T: Eval + Domain + Clone> Eval for Piecewise<T> {
    fn value(&self, x: f32) -> f32 {
        for func in self.subdomains.clone() { // See if can remove clone...
            if func.in_domain(x) {
                return func.value(x);
            }
        }
        f32::NAN
    }
}

impl<T: Domain + Clone> Domain for Piecewise<T> {
    fn in_domain(&self, i: f32) -> bool {
        for x in self.subdomains.clone() { // See if can remove clone...
            if x.in_domain(i) {
                return true;
            }
        }
        false
    }
}
