use ndarray::*;
use crate::structs::{Hyperplane, Pyramid};
use crate::lu::lu_solve;
use crate::cone_search::f;

//Functions to generate hyperplanes and pyramids

pub fn generate_hyperplane(x: Array2<f64>, par_id: usize, direction: u8) -> Hyperplane
{
    let a: Array2<f64> = array![
        [x[(0,0)], x[(0,1)], x[(0,2)], 1.0],
        [x[(1,0)], x[(1,1)], x[(1,2)], 1.0],
        [x[(2,0)], x[(2,1)], x[(2,2)], 1.0],
        [1.0, 1.0, 1.0, 1.0]
    ];

    let mut b = Array1::zeros(4);
    b[3] = 1.0;
    let c = lu_solve(a, b, 4);
    let coeff: [f64; 4] = [c[0], c[1], c[2], c[3]];

    Hyperplane{ parent_id: par_id, direction: direction, coeff: coeff }
}

pub fn generate_pyramid(mut peak: [f64; 3], ell: f64, id: usize) -> Pyramid
{
    peak[2] = f([peak[0], peak[1]]);

    let x1: Array2<f64> = array![
        [peak[0]+1.0, peak[1]+1.0, peak[2] - ell],
        [peak[0]-1.0, peak[1]+1.0, peak[2] - ell],
        [peak[0], peak[1], peak[2]]
    ];
    let x2: Array2<f64> = array![
        [peak[0]-1.0, peak[1]+1.0, peak[2] - ell],
        [peak[0]-1.0, peak[1]-1.0, peak[2] - ell],
        [peak[0], peak[1], peak[2]]
    ];
    let x3: Array2<f64> = array![
        [peak[0]-1.0, peak[1]-1.0, peak[2] - ell],
        [peak[0]+1.0, peak[1]-1.0, peak[2] - ell],
        [peak[0], peak[1], peak[2]]
    ];
    let x4: Array2<f64> = array![
        [peak[0]+1.0, peak[1]-1.0, peak[2] - ell],
        [peak[0]+1.0, peak[1]+1.0, peak[2] - ell],
        [peak[0], peak[1], peak[2]]
    ];

    let hyps: [Hyperplane; 4] = [
        generate_hyperplane(x1, id, 0),
        generate_hyperplane(x2, id, 1),
        generate_hyperplane(x3, id, 2),
        generate_hyperplane(x4, id, 3)
    ];

    Pyramid{ id: id, peak: peak, ell: ell, dist: 0.0, hyperplanes: hyps }
}