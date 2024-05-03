use rand::{
    distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler},
    Rng,
};

use imageproc::point::Point;

#[derive(Clone, Copy)]
pub struct RandomPoint(Point<i32>);

impl RandomPoint {
    pub fn new(p: Point<i32>) -> Self {
        Self(p)
    }

    pub fn into_inner(self) -> Point<i32> {
        let Self(p) = self;
        p
    }
}

pub struct UniformPoint {
    x: UniformInt<i32>,
    y: UniformInt<i32>,
}

impl UniformSampler for UniformPoint {
    type X = RandomPoint;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let Point { x: x1, y: y1 } = low.borrow().into_inner();
        let Point { x: x2, y: y2 } = high.borrow().into_inner();

        UniformPoint {
            x: UniformInt::new(x1, x2),
            y: UniformInt::new(y1, y2),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let Point { x: x1, y: y1 } = low.borrow().into_inner();
        let Point { x: x2, y: y2 } = high.borrow().into_inner();

        UniformPoint {
            x: UniformInt::new_inclusive(x1, x2),
            y: UniformInt::new_inclusive(y1, y2),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        RandomPoint::new(Point {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
        })
    }
}

impl SampleUniform for RandomPoint {
    type Sampler = UniformPoint;
}
