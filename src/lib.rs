use imageproc::point::Point;
use rand::{
    distributions::{Distribution, Uniform},
    thread_rng, Rng,
};

#[derive(Debug)]
struct Polygon<T, const N: usize> {
    vertices: [Point<T>; N],
}

struct Bivariate<X, Y> {
    x: X,
    y: Y,
}

impl<T, const N: usize, X, Y> Distribution<Polygon<T, N>> for Bivariate<X, Y>
where
    X: Distribution<T>,
    Y: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Polygon<T, N> {
        Polygon {
            vertices: [(); N].map(|_| Point {
                x: self.x.sample(rng),
                y: self.y.sample(rng),
            }),
        }
    }
}

pub fn run() {
    let mut rng = thread_rng();
    let dist = Bivariate {
        x: Uniform::new(0, 2),
        y: Uniform::new(2, 4),
    };
    let shape: Polygon<i32, 5> = dist.sample(&mut rng);
    println!("{:?}", shape);
}
