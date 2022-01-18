mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;
use std::time::Instant;

fn main()
{
    let before = Instant::now();
    cone_search::solve((-5.0, 5.0), (-5.0, 5.0), 1.5, 100, 10000);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
