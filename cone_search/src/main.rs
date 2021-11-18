mod structs;
mod lu;
mod intersections;
mod cone_search;
mod pyramid_handler;


fn main()
{
    cone_search::solve((-0.8, 0.8), (-0.8, 0.8), 7.0, 16);
}
