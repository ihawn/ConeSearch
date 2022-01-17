mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;
use std::time::Instant;

fn main()
{
    let before = Instant::now();
    cone_search::solve((-10.0, 10.0), (-10.0, 10.0), 20.0, 200, 5000);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
