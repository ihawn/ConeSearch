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
        [1.0, 2.0, 3.0],
        [5.0, 6.0, 7.0],
        [9.0, 10.0, 12.0]];

    let b: Array1<f64> = array![
        1.0,
        2.0,
        3.0
    ];

    let x = lu::lu_solve(a, b,3);
    println!("{:?}", x);
}
