mod transform;
use transform::ft::*;

extern crate num;
use num::Complex;

fn main() {

    let v = vec![0.0, 1.0,2.0,3.0,4.0,5.0, 0.0,1.0,2.0,3.0,4.0,5.0];
    let t = dft(v);

    for x in t {
        println!("{:}", x.re);
    }
}
