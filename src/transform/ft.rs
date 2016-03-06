extern crate num;
use self::num::Complex;

use std::f64::consts::PI;
use std::iter::Iterator;

/// A Discrete Fourier Transfom using a slow algorithm
/// * `x` - Sample
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

/// Returns the DFT sample frequencies
/// * `n` - Length of sample
/// * `t` - Sample spacing
pub fn ftfreq(n: usize, d: f64) -> Vec<f64> {
    let window = n as isize;
    let val = 1.0 / (window as f64 * d);
    let (plus, neg) = match window % 2 {
        0 => (window / 2 - 1, -window / 2),
        1 => ((window - 1) / 2, -(window - 1) / 2),
        _ => panic!("Error"),
    };
    let mut result: Vec<f64> = Vec::with_capacity(n);

    for x in 0..(plus + 1) {
        result.push(x as f64 * val);
    }
    for x in neg..0 {
        result.push(x as f64 * val);
    }
    result
}