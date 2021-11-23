mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;


fn main()
{
    cone_search::solve((-3.5, 4.5), (-2.9, 3.2), 15.0, 45, 5000);
}
