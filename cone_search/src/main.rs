mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;
use std::time::Instant;
use std::io;

fn main()
{
    let before = Instant::now();
    cone_search::solve((-11.0, 10.0), (-12.0, 11.0), 2650.0, 125, 100000);
    println!("Elapsed time: {:.2?}", before.elapsed());
    io::stdin().read_line(&mut String::new()).unwrap();
}
