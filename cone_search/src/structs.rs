//Structs to use throughout the program

#[derive(Copy, Clone)]
pub struct Hyperplane
{
    pub parent_id: usize,
    pub direction: u8, //0-3
    pub coeff: [f64; 4]
}

#[derive(Copy, Clone)]
pub struct Pyramid
{
    pub id: usize,
    pub peak: [f64; 3],
    pub ell: f64,
    pub dist: f64, //distance to another pyramid. Sort of a temp variable but it's more convenient to store it here
    pub hyperplanes: [Hyperplane; 4]
}

#[derive(Copy, Clone)]
pub struct Vector3
{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl PartialEq for Vector3
{
    fn eq(&self, other: &Self) -> bool { self.x == other.x && self.y == other.y && self.z == other.z }
}

impl Eq for Vector3 {}

