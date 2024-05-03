use imageproc::point::Point;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

use crate::random_point::RandomPoint;

mod random_point;

pub fn run() {
    let mut rng = thread_rng();
    let uniform = Uniform::new(
        RandomPoint::new(Point { x: 0, y: 0 }),
        RandomPoint::new(Point { x: 1, y: 2 }),
    );
    println!("{:?}", uniform.sample(&mut rng).into_inner());
}
