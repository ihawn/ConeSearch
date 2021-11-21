use crate::structs::{Hyperplane, Pyramid, Vector3};
use std::collections::BinaryHeap;
use crate::pyramid_handler::{generate_pyramid, combine_pyramids};
use crate::intersections::{intersect_new_hyperplane, intersect_pyramids, prune_intersections};


pub fn solve(x_bounds: (f64, f64), y_bounds: (f64, f64), ell: f64, closeness_threshold: usize)
{
    //init
    let mut hyperplanes: Vec<Hyperplane> = vec![];
    let mut pyramids: Vec<Pyramid> = vec![];
    let mut intersections: Vec<Vector3> = vec![];  
    let mut lower_bound = -1e10_f64;
    let mut upper_bound = 1e10_f64;
    pyramids.push(generate_pyramid([x_bounds.0, y_bounds.0, 0.0], ell, 0));
    pyramids.push(generate_pyramid([x_bounds.0, y_bounds.1, 0.0], ell, 1));
    pyramids.push(generate_pyramid([x_bounds.1, y_bounds.0, 0.0], ell, 2));
    pyramids.push(generate_pyramid([x_bounds.1, y_bounds.1, 0.0], ell, 3));


    // //Initial combinations of intersecting pyramids
    // let combos = combine_pyramids(&pyramids);

    // for i in 0..combos.len()
    // {
    //     let sects = intersect_pyramids(combos[i][0], combos[i][1], combos[i][2]);
    //     for s in sects { intersections.push(s); }
    // }

    //Add hyperplanes to the list
    for i in 0..pyramids.len()
    {
        for j in 0..pyramids[i].hyperplanes.len()
        {
            hyperplanes.push(pyramids[i].hyperplanes[j]);

            let sects = intersect_new_hyperplane(&hyperplanes, &pyramids, pyramids[i]);
            for s in sects { intersections.push(s); }

            //intersections = prune_intersections(intersections.clone(), &pyramids);
        }
    }

    for i in 0..intersections.len()
    {
        println!("{:?}", intersections[i]);
    }
    println!("{}", intersections.len());



    for i in 0..0
    {
        let min_loc = min_sect(&intersections);
        let pt = intersections[min_loc];
        let fx = f([pt.x, pt.y]);
        let pyr: Pyramid = generate_pyramid([pt.x, pt.y, fx], ell, pyramids.len());

        println!("{:?}", pt);

        if pt.z > lower_bound { lower_bound = pt.z; }
        if fx < upper_bound { upper_bound = fx; }

        let ans = get_adj_hyperplanes(pyramids, pyr, closeness_threshold);
        let mut close_hyps = ans.0; pyramids = ans.1;
        let mut new_intersections = intersect_new_hyperplane(&close_hyps, &pyramids, pyr);

        for j in 0..pyr.hyperplanes.len()
        {
            hyperplanes.push(pyr.hyperplanes[j]);
        }

        new_intersections = prune_intersections(new_intersections, &pyramids);
        let mut singleton = vec![]; singleton.push(pyr);
        intersections = prune_intersections(intersections, &singleton);
        intersections.append(&mut new_intersections);

        println!("\nLower Bound: {}\nUpper Bound: {}", lower_bound, upper_bound);
    };
}

fn min_sect(lst: &Vec<Vector3>) -> usize
{
    let mut min_val = lst[0].z;
    let mut min_pos: usize = 0;

    for i in 0..lst.len()
    {
        if lst[i].z < min_val
        {
            min_val = lst[i].z;
            min_pos = i;
        }
    }

    min_pos
}

fn get_adj_hyperplanes(mut pyrs: Vec<Pyramid>, pyr: Pyramid, adj_size: usize) -> (Vec<Hyperplane>, Vec<Pyramid>)
{
    let mut dist_heap: BinaryHeap<Pyramid> = BinaryHeap::new();
    let mut adj: Vec<Hyperplane> = vec![];

    for i in 0..pyrs.len()
    {
        pyrs[i].dist = f64::max(f64::abs(pyrs[i].peak[0] - pyr.peak[0]), f64::abs(pyrs[i].peak[1] - pyr.peak[1]));
        dist_heap.push(pyrs[i]);
    }

    for i in 0..usize::min(adj_size, pyrs.len())
    {
        let current = dist_heap.pop().unwrap();
        for j in 0..4
        {
            adj.push(current.hyperplanes[j]);
        }
    }
    (adj, pyrs)
}

pub fn f(x: [f64; 2]) -> f64
{
    x[0]*x[0] + x[1]*x[1] + 2.0*f64::sin((x[0] - 2.0)*x[1])
}
