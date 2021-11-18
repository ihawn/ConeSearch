extern crate ndarray;

use ndarray::prelude::*;
use crate::structs::{Hyperplane, Pyramid, Vector3};
use crate::lu::lu_solve;
use std::cmp::max;

//Function to generate all valid intersections given 3 pyramids. Should only be used in initial iteration
pub fn intersect_pyramids(p1: Pyramid, p2: Pyramid, p3: Pyramid) -> Vec<Vector3>
{
    let default_hyp = Hyperplane { parent_id: 0, direction: 0, coeff: [0.0, 0.0, 0.0, 0.0] };
    let mut hyp_arr: [Hyperplane; 12] = [default_hyp; 12];
    let mut int_points: Vec<Vector3> = vec![];


    //Build list of hyperplanes to intersect
    for i in 0..4
    {
        hyp_arr[3*i] = p1.hyperplanes[i];
        hyp_arr[3*i+1] = p2.hyperplanes[i];
        hyp_arr[3*i+2] = p3.hyperplanes[i];
    }

    for i in 0..hyp_arr.len()
    {
        for j in i+1..hyp_arr.len()
        {
            for k in j+1..hyp_arr.len()
            {
                if hyp_arr[i].direction != hyp_arr[j].direction &&
                   hyp_arr[j].direction != hyp_arr[k].direction &&
                   hyp_arr[i].direction != hyp_arr[k].direction
                {
                    let pt = intersect_hyperplanes(hyp_arr[i], hyp_arr[j], hyp_arr[k]).to_vec();
                    let v: Vector3 = Vector3 {x: pt[0], y: pt[1], z: pt[2] };
                    
                    if valid_intersection(p1, p2, p3, &v) { int_points.push(v); }
                }
            }
        }
    }

    int_points
}

pub fn intersect_new_hyperplane(lst: &Vec<Hyperplane>, pyr_lst: &Vec<Pyramid>, pyr: Pyramid) -> Vec<Vector3>
{
    let mut sect_lst: Vec<Vector3> = vec![];

    for i in 0..lst.len()
    {
        for j in i+1..lst.len()
        {
            if lst[i].direction != lst[j].direction
            {
                for k in 0..pyr.hyperplanes.len()
                {
                    if lst[j].direction != pyr.hyperplanes[k].direction &&
                       lst[i].direction != pyr.hyperplanes[k].direction
                    {
                        let pt = intersect_hyperplanes(lst[i], lst[j], pyr.hyperplanes[k]).to_vec();
                        let v: Vector3 = Vector3 {x: pt[0], y: pt[1], z: pt[2] };
                    
                        if valid_intersection(pyr_lst[lst[i].parent_id], pyr_lst[lst[j].parent_id], pyr, &v) { sect_lst.push(v); }
                    }
                }
            }
        }

        //Two hyperplanes from the same pyramid

        for k in 0..pyr.hyperplanes.len()
        {
            if lst[i].direction != pyr.hyperplanes[k].direction &&
               lst[i].direction != pyr.hyperplanes[(k+1)%4].direction
            {
                let pt = intersect_hyperplanes(lst[i], pyr.hyperplanes[(k+1)%4], pyr.hyperplanes[k]).to_vec();
                let v: Vector3 = Vector3 {x: pt[0], y: pt[1], z: pt[2] };
                    
                if valid_intersection(pyr_lst[lst[i].parent_id], pyr, pyr, &v) { sect_lst.push(v); }
             
            }
        }
    }

    sect_lst
}

fn intersect_hyperplanes(h1: Hyperplane, h2: Hyperplane, h3: Hyperplane) -> Array1<f64>
{
    let a: Array2<f64> = array![
        [h1.coeff[0], h1.coeff[1], h1.coeff[2]],
        [h2.coeff[0], h2.coeff[1], h2.coeff[2]],
        [h3.coeff[0], h3.coeff[1], h3.coeff[2]]];


    let b: Array1<f64> = array![
        -h1.coeff[3],
        -h2.coeff[3],
        -h3.coeff[3]];

   lu_solve(a, b, 3)
}

fn valid_intersection(p1: Pyramid, p2: Pyramid, p3: Pyramid, pt: &Vector3) -> bool
{
    let t1: f64 = p1.peak[2] - p1.ell * f64::max(f64::abs(pt.x - p1.peak[0]), f64::abs(pt.y - p1.peak[1]));
    let t2: f64 = p2.peak[2] - p2.ell * f64::max(f64::abs(pt.x - p2.peak[0]), f64::abs(pt.y - p2.peak[1]));
    let t3: f64 = p3.peak[2] - p3.ell * f64::max(f64::abs(pt.x - p3.peak[0]), f64::abs(pt.y - p3.peak[1]));

    f64::abs(pt.z - t1) <= 1e-8_f64 &&
    f64::abs(pt.z - t1) <= 1e-8_f64 &&
    f64::abs(pt.z - t1) <= 1e-8_f64
}

pub fn prune_intersections(mut sects: Vec<Vector3>, pyrs: &Vec<Pyramid>) -> Vec<Vector3>
{
    for i in 0..pyrs.len()
    {
        let mut s = sects.len();
        let mut j = 0;
        while j < s
        {
            let t: f64 = pyrs[i].peak[2] - pyrs[i].ell * f64::max(f64::abs(sects[j].x - pyrs[i].peak[0]), f64::abs(sects[j].y - pyrs[i].peak[1]));
            if t > sects[j].z + 1e-8_f64
            {
                sects.remove(j);
                s-=1;
            }

            j+=1;
        }
    }

    sects
}