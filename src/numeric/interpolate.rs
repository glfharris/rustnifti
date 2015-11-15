use numeric::function::{Domain, Eval};
use numeric::piecewise::Piecewise;
use numeric::polynomial::Polynomial;

use std::ops::{Sub, Mul, Add};
use std::fmt::Debug;

extern crate nalgebra;
use self::nalgebra::{DMat, DVec, Inv};

extern crate num;
use self::num::traits::{Zero, CheckedAdd};
use self::num::iter::range;

/// Trait providing interpolation functions
pub trait Interpolate{
    fn cubic_spline(&self) -> Piecewise<Polynomial>;
    fn linear(&self) -> Piecewise<Polynomial>;
}

impl Interpolate for Vec<f32> {
    fn cubic_spline(&self) -> Piecewise<Polynomial> {
        let (mut h, mut coeffs_mat) = gen_coeffs_matrix(range(0f32, self.len() as f32).collect()); // generates a Vec<f32> comparable to the index
        let mut B: DVec<f32> = DVec::new_zeros(self.len());
        for i in 1..self.len() -1 {
            B[i] = ((self[i+1] - self[i]) / h[i] - (self[i] - self[i-1]) / h[i-1]);
        }
        coeffs_mat[(0,0)] = 1.0;
        coeffs_mat[(self.len() -1, self.len() -1)] = 1.0;
        let inv = coeffs_mat.inv().unwrap();
        println!("{:?}", inv);
        let ydd = B * inv;

        let mut result: Piecewise<Polynomial> = Piecewise::new(vec![]);
        for x in 0..h.len() {
            let a = (ydd[x+1] - ydd[x]) / ( 6.0 * h[x] );
            let b = ydd[x] / 2.0;
            let c = (self[x+1] - self[x]) / h[x] - (ydd[x+1] * h[x]/ 6.0) - (ydd[x] * h[x] / 3.0);
            let d = self[x];
            let temp = Polynomial{ coeffs: vec![d,c,b,a], domain: (x as f32, (x+1) as f32)};
            result.add_sub(temp);
        }
        result
    }
    fn linear(&self) -> Piecewise<Polynomial> {
        let mut piece = Piecewise::default();
        for (idx, win) in self.windows(2).enumerate() {
            let idx = idx as f32;
            let grad = win[1] - win[0];
            let c = win[0] - (grad * idx);
            let poly = Polynomial::new(vec![c, grad], (idx, idx + 1.0));
            piece.add_sub(poly);
        }
        piece
    }
}

pub enum BoundaryCondition {
    Natural,
    Parabolic,
    Zero,
    FirstDev(f32),
    SecondDev(f32),
}

/// Feed in a Vec<T> of x values for data
pub fn gen_coeffs_matrix<T: Sub<T, Output = T> + Copy + Zero + Add<T, Output = T> + Mul<T, Output = T> + Debug> (v: Vec<T>) -> (Vec<T>, DMat<T>) {
    let mut mat: DMat<T> = DMat::new_zeros(v.len(), v.len());
    let mut x_diff = vec![];
    for win in v.windows(2) {
        x_diff.push(win[1] - win[0])
    }
    let mut offset = 0;
    for j in 1..(v.len() - 1) {
        mat[(j, offset + 0)] = x_diff[j];
        mat[(j, offset + 1)] = (x_diff[j -1 ] + x_diff[j]) + (x_diff[j -1] + x_diff[j]);
        mat[(j, offset + 2)] = x_diff[j];
        offset += 1;
    }
    (x_diff, mat)
}
