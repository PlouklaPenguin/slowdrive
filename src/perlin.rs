use bevy::math::{
    vec2,
    Vec2,
};
use core::ops::{Add, Mul, Sub};
use rand::{
    distr::{Distribution, StandardUniform},
    seq::SliceRandom,
    Rng, SeedableRng,
};
use rand_xorshift::XorShiftRng;

const TABLE_SIZE: usize = 256;

trait Quintic {
    fn map_quintic(&self) -> Self;
}

impl Quintic for f32 {
    fn map_quintic(&self) -> Self {
        let x = self.clamp(0.0, 1.0);

        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }
}

trait NoiseHasher: Send + Sync {
    fn hash(&self, to_hash: &[isize]) -> usize;
}

#[inline(always)]
pub fn linear<T>(a: T, b: T, alpha: f32) -> T
where
    T: Mul<f32, Output = T> + Add<Output = T>,
{
    b * alpha + a * (1.0 - alpha)
}


#[inline(always)]
pub fn perlin_2d<NH>(point: Vec2, hasher: &NH) -> f32
where
    NH: NoiseHasher + ?Sized,
{
    const SCALE_FACTOR: f32 = 2.0 / std::f32::consts::SQRT_2;

    let corner = point.floor();
    let distance = point - corner;

    macro_rules! call_gradient(
        ($x:expr, $y:expr) => {
            {
            let offset = vec2($x, $y);
            let point = distance - offset;

            match hasher.hash({
                &[(corner + offset).x as isize, (corner + offset).y as isize]
            }) & 0b11 {
                0 => point.x + point.y,  // ( 1,  1)
                1 => -point.x + point.y, // (-1,  1)
                2 => point.x - point.y,  // ( 1, -1)
                3 => -point.x - point.y, // (-1, -1)
                _ => unreachable!(),
            }
        }
    }
    );

    
    let g00 = call_gradient!(0., 0.);
    let g10 = call_gradient!(1., 0.);
    let g01 = call_gradient!(0., 1.);
    let g11 = call_gradient!(1., 1.);

    let curve = Vec2 {
        x: distance.x.map_quintic(),
        y: distance.y.map_quintic()
    };


    let result = linear(
        linear(g00, g01, curve.y),
        linear(g10, g11, curve.y),
        curve.x,
    ) * SCALE_FACTOR;

    result.clamp(-1.0, 1.0)
}

#[derive(Clone, Copy)]
struct PermutationTable {
    values: [u8; TABLE_SIZE],
}

impl Distribution<PermutationTable> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        let mut perm_table = PermutationTable {
            values: core::array::from_fn(|x| x as u8),
        };
        perm_table.values.shuffle(rng);

        perm_table
    }
}

impl PermutationTable {
    fn new(seed: u32) -> Self {
        let mut real = [0; 16];
        real[0] = 1;
        for i in 1..4 {
            real[i * 4] = seed as u8;
            real[i * 4 + 1] = (seed >> 8) as u8;
            real[i * 4 + 2] = (seed >> 16) as u8;
            real[i * 4 + 3] = (seed >> 24) as u8;
        }
        let mut rng: XorShiftRng = SeedableRng::from_seed(real);
        rng.random()
    }
}

impl NoiseHasher for PermutationTable {
    fn hash(&self, to_hash: &[isize]) -> usize {
        let index = to_hash
            .iter()
            .map(|&a| (a & 0xff) as usize)
            .reduce(|a, b| self.values[a] as usize ^ b)
            .unwrap();
        self.values[index] as usize
    }
}
