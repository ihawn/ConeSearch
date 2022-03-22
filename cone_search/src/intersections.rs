extern crate ndarray;

use ndarray::prelude::*;
use crate::structs::{Hyperplane, Pyramid, Vector3};
use lair::equation::solve;


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
        let mut offset: i32 = 1;
        let mut k: i32 = 0;
        while k < pyr.hyperplanes.len() as i32
        {
            if k == 3 { offset = -3; }
            if lst[i].direction != pyr.hyperplanes[k as usize].direction &&
               lst[i].direction != pyr.hyperplanes[(k+offset) as usize].direction
            {
                let pt = intersect_hyperplanes(lst[i], pyr.hyperplanes[k as usize], pyr.hyperplanes[(k+offset) as usize]).to_vec();
                let v: Vector3 = Vector3 {x: pt[0], y: pt[1], z: pt[2] };
                    
                if valid_intersection(pyr_lst[lst[i].parent_id], pyr, pyr, &v) { sect_lst.push(v); }
             
            }
            k+=1;
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

    solve(&a, &b).unwrap()
}


fn valid_intersection(p1: Pyramid, p2: Pyramid, p3: Pyramid, pt: &Vector3) -> bool
{
    let t1: f64 = p1.peak[2] - p1.ell * f64::max(f64::abs(pt.x - p1.peak[0]), f64::abs(pt.y - p1.peak[1]));
    let t2: f64 = p2.peak[2] - p2.ell * f64::max(f64::abs(pt.x - p2.peak[0]), f64::abs(pt.y - p2.peak[1]));
    let t3: f64 = p3.peak[2] - p3.ell * f64::max(f64::abs(pt.x - p3.peak[0]), f64::abs(pt.y - p3.peak[1]));

    f64::abs(pt.z - t1) <= 1e-12_f64 &&
    f64::abs(pt.z - t2) <= 1e-12_f64 &&
    f64::abs(pt.z - t3) <= 1e-12_f64
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
            if t > sects[j].z + 1e-12_f64
            {
                sects.remove(j);
                s-=1;
            }

            j+=1;
        }
    }

    sects
}