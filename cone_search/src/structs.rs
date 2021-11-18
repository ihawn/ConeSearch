//Structs to use throughout the program
use std::{cmp::Ordering};

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Hyperplane
{
    pub parent_id: usize,
    pub direction: u8, //0-3
    pub coeff: [f64; 4]
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Pyramid
{
    pub id: usize,
    pub peak: [f64; 3],
    pub ell: f64,
    pub dist: f64, //distance to another pyramid. Sort of a temp variable but it's more convenient to store it here
    pub hyperplanes: [Hyperplane; 4]
}

impl Eq for Pyramid {}

impl Ord for Pyramid
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.dist.partial_cmp(&other.dist).unwrap().reverse()
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Vector3 {}

impl Ord for Vector3
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.z.partial_cmp(&other.z).unwrap().reverse()
    }
}
