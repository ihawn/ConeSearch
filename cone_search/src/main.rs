mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;


fn main()
{
    cone_search::solve((-5.0, 5.0), (-5.0, 5.0), 6.5, 125, 5000);
}
