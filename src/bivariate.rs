use imageproc::point::Point;
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::triangle::Triangle;

pub struct Bivariate {
    x: Uniform<i32>,
    y: Uniform<i32>,
}

impl Bivariate {
    pub fn new(x: Uniform<i32>, y: Uniform<i32>) -> Self {
        Self { x, y }
    }
}

impl Distribution<Triangle> for Bivariate {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triangle {
        Triangle::new([(); 3].map(|_| Point {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
        }))
    }
}
