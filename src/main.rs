mod transform;
use transform::ft::*;

extern crate num;
use num::Complex;

extern crate gnuplot;
use gnuplot::{Figure, Caption, Color};

use std::iter::Iterator;

fn main() {
    let mut sam = Vec::new();
    for x in 0..200 {
        sam.push((x as f64 / 2.0).sin() + (x as f64 / 5.0).sin())
    }
    let f = ftfreq(sam.len(), 0.067);
    let a = dft(sam);
    let mut amps = Vec::new();
    for x in a {
        amps.push(x.norm());
    }

    let mut fg = Figure::new();
    fg.axes2d()
      .lines(&f, &amps, &[Caption("A line"), Color("black")]);
    fg.show();
}
