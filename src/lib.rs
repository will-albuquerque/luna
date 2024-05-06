use bivariate::Bivariate;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

mod bivariate;
mod triangle;

pub fn run() {
    let mut rng = thread_rng();
    let dist = Bivariate::new(Uniform::new_inclusive(0, 10), Uniform::new_inclusive(0, 10));
    let t = dist.sample(&mut rng);
    println!("{:?}", t);
}
