#![allow(dead_code)]
use std::fmt;
use std::fmt::{Formatter, Display};
use std::f32;

use numeric::calculus::Calculus;
use numeric::function::{Eval, Domain};

/// A struct for representing a single indeterminate polynomial. According to normal conventions, the order must be >= 0
#[derive(Debug, Clone)]
pub struct Polynomial {
    /// Coefficients in the format vec[0] + vec[1]*x + vec[2]*x^2 ... = y
    pub coeffs: Vec<f32>,
    /// Range that function is valid for in the form (min, max)
    pub domain: (f32, f32),
}

impl Polynomial {
    /// Returns the order of the polynomial - Currently will malfunction if the coeffs set to 0
    pub fn order(&self) -> usize {
        self.coeffs.len() - 1
    }
    pub fn new(coeffs: Vec<f32>, domain: (f32, f32)) -> Polynomial {
        Polynomial { coeffs: coeffs, domain: domain }
    }
}

impl Eval for Polynomial {
    fn value(&self, x: f32) -> f32 {
        if self.in_domain(x) {
            // let mut tot = 0f32;
            // for (pow, coeff) in self.coeffs.iter().enumerate() {
            //     tot += coeff * x.powi(pow as i32);
            // }
            // return tot;
            return self.coeffs.iter().enumerate()
                .map(|(pow, c)| c * x.powi(pow as i32))
                .fold(0f32, |sum, i| sum + i);
        }
        f32::NAN
    }
}

impl Domain for Polynomial {
    fn in_domain(&self, x: f32) -> bool {
        let (r1, r2) = self.domain;
        let max = r1.max(r2);
        let min = r1.min(r2);
        if x <= max && x >= min {
            return true;
        }
        false
    }
}

impl Calculus for Polynomial {
    fn integral(&mut self, c: f32) {
        let mut updated = self.coeffs.clone();
        for (pow, val) in self.coeffs.iter().enumerate() {
            updated[pow] = self.coeffs[pow] / (pow as f32 + 1f32);
        }
        updated.insert(0, c);
        self.coeffs = updated;


    }
    fn differential(&mut self) {
        let mut updated = self.coeffs.clone();
        for (pow, val) in self.coeffs.iter().enumerate() {
            updated[pow] = self.coeffs[pow] * pow as f32;
        }
        updated.remove(0);
        self.coeffs = updated;
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (pow, coeff) in self.coeffs.iter().enumerate() {
            write!(f, "+ {}x^{} ", coeff, pow);
        }
        write!(f, "= y")
    }
}

impl Default for Polynomial {
    fn default() -> Polynomial {
        Polynomial{ coeffs: vec![], domain: (f32::NEG_INFINITY, f32::INFINITY)}
    }
}
