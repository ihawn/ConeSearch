use crate::structs::{Hyperplane, Pyramid};

pub fn solve()
{
    let h1: Hyperplane = Hyperplane{parent_id: 0, direction: 0, coeff: [1.0, 2.0, 3.0, 4.0]};
    let h2: Hyperplane = Hyperplane{parent_id: 0, direction: 0, coeff: [3.0, 2.0, 4.0, 7.0]};
    let h3: Hyperplane = Hyperplane{parent_id: 0, direction: 0, coeff: [11.0, 22.0, 13.0, 1.0]};

    //println!("{:?}", intersect_hyperplanes(h1, h2, h3));
}

pub fn f(x: [f64; 2]) -> f64
{
    x[0]*x[0] + x[1]*x[1]
}
