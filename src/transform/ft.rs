extern crate num;
use self::num::Complex;

use std::f64::consts::PI;
use std::iter::Iterator;

pub fn dft(x: Vec<f64>) -> Vec<Complex<f64>> {
    let len = x.len();
    let mut ft: Vec<Complex<f64>> = Vec::with_capacity(len);
    for k in 0..len {
        ft.push(x.iter()
                 .enumerate()
                 .map(|(n, z): (usize, &f64)| {
                     (Complex::i() * PI * -2f64 * k as f64 * n as f64 / len as f64).exp() *
                     Complex::new(*z, 0f64)
                 })
                 .fold(Complex::new(0f64, 0f64), |sum, t| sum + t))
    }
    ft
}