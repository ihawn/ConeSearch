use crate::structs::{Hyperplane, Pyramid, Vector3};
use std::collections::BinaryHeap;
use std::error::Error;
use std::time::Instant;
use crate::pyramid_handler::{generate_pyramid};
use crate::intersections::{intersect_new_hyperplane, prune_intersections};
use csv;


pub fn solve(x_bounds: (f64, f64), y_bounds: (f64, f64), ell: f64, closeness_threshold: usize, max_iter: usize)
{
    //init
    let mut hyperplanes: Vec<Hyperplane> = vec![];
    let mut pyramids: Vec<Pyramid> = vec![];
    let mut intersections: Vec<Vector3> = vec![];  
    let mut lower_bound = -1e10_f64;
    let mut upper_bound = 1e10_f64;
    let mut best_loc = Vector3{x: 10.0, y: 10.0, z: 10.0};
    let mut itt_sols: Vec<[f64; 2]> = vec!();
    let mut ab_diff: Vec<f64> = vec!();
    let mut step_times: Vec<f64> = vec!();
    let mut hyp_count: Vec<usize> = vec!();
    pyramids.push(generate_pyramid([x_bounds.0, y_bounds.0, 0.0], ell, 0));
    pyramids.push(generate_pyramid([x_bounds.0, y_bounds.1, 0.0], ell, 1));
    pyramids.push(generate_pyramid([x_bounds.1, y_bounds.0, 0.0], ell, 2));
    pyramids.push(generate_pyramid([x_bounds.1, y_bounds.1, 0.0], ell, 3));


    //Add hyperplanes to the list
    for i in 0..pyramids.len()
    {
        for j in 0..pyramids[i].hyperplanes.len()
        {
            hyperplanes.push(pyramids[i].hyperplanes[j]);

            let sects = intersect_new_hyperplane(&hyperplanes, &pyramids, pyramids[i]);
            for s in sects { intersections.push(s); }

            intersections = prune_intersections(intersections, &pyramids);
        }
    }

    let mut time = Instant::now();

    for i in 0..max_iter
    {
        let min_loc = min_sect(&intersections);
        let pt = intersections[min_loc];
        let fx = f([pt.x, pt.y]);
        let pyr: Pyramid = generate_pyramid([pt.x, pt.y, fx], ell, pyramids.len());

        if pt.z > lower_bound { lower_bound = pt.z; }
        if fx < upper_bound 
        {
            upper_bound = fx;
            best_loc = pt;
            itt_sols.push([pt.x, pt.y]);
            ab_diff.push(upper_bound - lower_bound);
            step_times.push(time.elapsed().as_secs_f64());
            time = Instant::now();
            hyp_count.push(hyperplanes.len());
        }

        let ans = get_adj_hyperplanes(pyramids, pyr, closeness_threshold);
        let close_hyps = ans.0; pyramids = ans.1;
        let mut new_intersections = intersect_new_hyperplane(&close_hyps, &pyramids, pyr);

        for j in 0..pyr.hyperplanes.len()
        {
            hyperplanes.push(pyr.hyperplanes[j]);
        }

        new_intersections = prune_intersections(new_intersections, &pyramids);
        let mut singleton = vec![]; singleton.push(pyr);
        intersections = prune_intersections(intersections, &singleton);
        intersections.append(&mut new_intersections);
        pyramids.push(pyr);

        if (i+1)%10 == 0
        {
            println!("\nIteration: {}\nLower Bound: {}\nUpper Bound: {}, \nHyperplane Count: {}, \nIntersection Count: {}, \nx1: {}, x2: {}, \nf(x): {}", i+1, lower_bound, upper_bound, hyperplanes.len(), intersections.len(), best_loc.x, best_loc.y, upper_bound);
        }

        if upper_bound - lower_bound < 0.01 && i > 100
        {
            itt_sols.push([best_loc.x, best_loc.y]);
            ab_diff.push(upper_bound - lower_bound);
            hyp_count.push(hyperplanes.len());           
            step_times.push(time.elapsed().as_secs_f64());
            break;
        }
    }

    let path = "C:/Users/Isaac/Documents/Optimization/NonConvex/NonConvexOptimization/NonConvexOptimiztion/BasinHopping/csv/ConeSearchRastrigin.csv";
    if let Err(e) = write_to_csv(path, itt_sols, ab_diff, step_times, hyp_count)
    {
        eprint!("{}", e);
    }
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

    for _i in 0..usize::min(adj_size, pyrs.len())
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
    let pi = 3.14159265358979;
    //x[0]*x[0] + x[1]*x[1]
    //0.05*(x[0]*x[0] + x[1]*x[1] + 15.0*f64::powf(f64::sin((x[0] - 2.0)*x[1]), 2.0))
    //0.5*(f64::powi(x[0], 4) - 16.0*f64::powi(x[0], 2) + 5.0*x[0] + f64::powi(x[1], 4) - 16.0*f64::powi(x[1], 2) + 5.0*x[1]) //Styblinski–Tang function
    //f64::powi(f64::powi(x[0], 2) + x[1] - 11.0, 2) + f64::powi(f64::powi(x[1], 2) + x[0] - 7.0, 2)//Himmelblau
    //f64::sqrt(x[0]*x[0] + x[1]*x[1] + 5.0 * f64::powi(f64::sin((x[0]-2.0)*x[1]), 2))
    //-20.0*f64::exp(-0.2*f64::sqrt(0.5*(f64::powf(x[0],2.0) + f64::powf(x[1],2.0)))) - f64::exp(0.5*(f64::cos(2.0*pi*x[0]) + f64::cos(2.0*pi*x[1]))) + 20.0 + f64::exp(1.0) //Ackley
    //-(1.0 + f64::cos(12.0*f64::sqrt(f64::powf(x[0], 2.0) + f64::powf(x[1], 2.0))))/(0.5*(f64::powf(x[0],2.0) + f64::powf(x[1], 2.0)) + 2.0) //Drop-Wave
    //(f64::powf(x[0], 2.0) + f64::powf(x[1], 2.0))/4000.0 - f64::cos(x[0])*f64::cos(x[1]/f64::sqrt(2.0)) + 1.0 //Griewank
    //-f64::cos(x[0])*f64::cos(x[1])*f64::exp(-f64::powf(x[0] - pi, 2.0) - f64::powf(x[1] - pi, 2.0)) //Easom
    //0.5 + (f64::powf(f64::sin(f64::powf(x[0], 2.0) - f64::powf(x[1], 2.0)), 2.0) - 0.5) / f64::powf(1.0 + 0.001*(f64::powf(x[1], 2.0) + f64::powf(x[1], 2.0)), 2.0) //Schaffer N. 2
    //-(f64::sin(x[0])*f64::powf(f64::sin(f64::powf(x[0], 2.0)/pi), 2.0) + f64::sin(x[1])*f64::powf(f64::sin(2.0*f64::powf(x[1], 2.0)/pi), 2.0)) //Michalewicz
    //(f64::powf(f64::powf(x[0], 2.0) + x[1] - 11.0, 2.0) + f64::powf(x[0] + f64::powf(x[1], 2.0) - 7.0, 2.0)) / 1000.0 //Himm
    20.0 + f64::powf(x[0], 2.0) - 10.0*f64::cos(2.0*pi*x[0]) + f64::powf(x[1], 2.0) - 10.0*f64::cos(2.0*pi*x[1]) //Rastrigin
}

pub fn f2(x: [f64; 2], n: f64) -> f64
{
    n*(f64::powf(x[0], 2.0) + f64::powf(x[1], 2.0))
}

fn write_to_csv(path: &str, xy: Vec<[f64; 2]>, alpha_beta: Vec<f64>, step_time: Vec<f64>, hyp_count: Vec<usize>) -> Result<(), Box<dyn Error>>
{
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record(&["x", "y", "ab_diff", "step_time", "hyp_count"])?;

    for i in 0..xy.len()
    {
        writer.write_record(&[xy[i][0].to_string(), xy[i][1].to_string(), alpha_beta[i].to_string(), step_time[i].to_string(), hyp_count[i].to_string()])?;
    }
    writer.flush()?;

    Ok(())
}
