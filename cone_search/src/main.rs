use structs::Hyperplane;
use ndarray::*;

mod structs;
mod lu;
mod intersections;
mod cone_search;


fn main()
{
    //cone_search::solve();
    let a: Array2<f64> = array![
        [1.0, 1.0, 1.0],
        [2.0, 3.0, 3.0],
        [3.0, 3.0, 5.0]];

    let b: Array1<f64> = array![
        4.0,
        11.0,
        16.0
    ];

    let x = lu::lu(a, b,3);
    println!("{:?}", x);
}
