use structs::Hyperplane;
use ndarray::*;

use crate::pyramid_handler::generate_hyperplane;

mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;


fn main()
{
    let x: Array2<f64> = array![
        [2.0, -0.5, 0.0],
        [1.0, -1.0, 4.6],
        [-3.7, 4.2, 1.2]
    ];

    let plane = generate_hyperplane(x, 0, 0);

    println!("{:?}", plane.coeff);
}
