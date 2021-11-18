use crate::structs::{Hyperplane, Pyramid, Vector3};
use crate::pyramid_handler::{generate_pyramid, combine_pyramids};
use crate::intersections::{intersect_new_hyperplane, intersect_pyramids, prune_intersections};
use itertools::Itertools;

pub fn solve(x_bounds: (f64, f64), y_bounds: (f64, f64), ell: f64)
{
    //init
    let mut hyperplanes: Vec<Hyperplane> = vec![];
    let mut pyramids: Vec<Pyramid> = vec![];
    pyramids.push(generate_pyramid([x_bounds.0, y_bounds.0, 0.0], ell, 0));
    pyramids.push(generate_pyramid([x_bounds.0, y_bounds.1, 0.0], ell, 1));
    pyramids.push(generate_pyramid([x_bounds.1, y_bounds.0, 0.0], ell, 2));
    pyramids.push(generate_pyramid([x_bounds.1, y_bounds.1, 0.0], ell, 3));

    //Initial combinations of intersecting pyramids
    let combos = combine_pyramids(&pyramids);
    let mut intersections: Vec<Vector3> = vec![];
    for i in 0..combos.len()
    {
        intersections.append(&mut intersect_pyramids(combos[i][0], combos[i][1], combos[i][2]));
    }

    //Add hyperplanes to the list
    for i in 0..pyramids.len()
    {
        for j in 0..pyramids[i].hyperplanes.len()
        {
            hyperplanes.push(pyramids[i].hyperplanes[j]);
            intersections.append(&mut intersect_new_hyperplane(&hyperplanes, &pyramids, pyramids[i]));
            intersections = intersections.into_iter().unique().collect();
            intersections = prune_intersections(intersections.clone(), &pyramids);
        }
    }

    for i in 0..intersections.len()
    {
        println!("{:?}", intersections[i]);
    }
    println!("{}", intersections.len());
}

pub fn f(x: [f64; 2]) -> f64
{
    x[0]*x[0] + x[1]*x[1] + 2.0*f64::sin((x[0] - 2.0)*x[1])
}
