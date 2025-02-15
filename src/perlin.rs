use crate::{
    utilities::{random_rng_int, random_vec_rng},
    vec3d::Point3d,
    vec3d::Vec3d,
};

const POINT_COUNT: usize = 256;

/// Perlin noise
#[derive(Clone)]
pub struct Perlin {
    random_vec3d: Vec<Vec3d>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(
        random_vec3d: Vec<Vec3d>,
        perm_x: Vec<usize>,
        perm_y: Vec<usize>,
        perm_z: Vec<usize>,
    ) -> Self {
        Self {
            random_vec3d,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn build_random() -> Perlin {
        let mut random_vec3d = Vec::<Vec3d>::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            random_vec3d.push(random_vec_rng(-1.0, 1.0));
        }
        Perlin {
            random_vec3d,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, p: Point3d) -> f64 {
        let (u, i) = Self::float_to_index(p.x);
        let (v, j) = Self::float_to_index(p.y);
        let (w, k) = Self::float_to_index(p.z);

        let mut c = [[[Vec3d::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vec3d[self.perm_x[(i + di) & 255]
                        ^ self.perm_y[(j + dj) & 255]
                        ^ self.perm_z[(k + dk) & 255]];
                }
            }
        }

        Self::perlin_interpolate(&c, u, v, w)
    }

    pub fn turbulence(&self, p: Point3d, depth: Option<usize>) -> f64 {
        let actual_depth = depth.unwrap_or(7); // Set default depth
        let mut accumulator = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..actual_depth {
            accumulator += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accumulator.abs()
    }

    fn generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = (0..POINT_COUNT).collect();
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            let target = random_rng_int(0, i + 1);
            p.swap(i, target);
        }
    }

    fn float_to_index(x: f64) -> (f64, usize) {
        let x_pos = if x < 0.0 { -1.0 * x } else { x };
        let u = x_pos - x_pos.floor();
        (u * u * (3.0 - (2.0 * u)), x_pos.floor() as usize)
    }

    fn perlin_interpolate(c: &[[[Vec3d; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - (2.0 * u));
        let vv = v * v * (3.0 - (2.0 * v));
        let ww = w * w * (3.0 - (2.0 * w));

        let mut accumulator = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3d::new(u - (i as f64), v - (j as f64), w - (k as f64));
                    accumulator += (((i as f64) * uu) + ((1 - i) as f64) * (1.0 - uu))
                        * (((j as f64) * vv) + ((1 - j) as f64) * (1.0 - vv))
                        * (((k as f64) * ww) + ((1 - k) as f64) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accumulator
    }
}

#[cfg(test)]
mod perlin_tests {
    use super::*;

    #[test]
    fn build_random_will_run() {
        let _ = Perlin::build_random();
    }

    #[test]
    fn noise_will_run_for_a_range_of_values() {
        let perlin = Perlin::build_random();

        perlin.noise(Point3d::new(1.0, 0.0, 0.0));
        perlin.noise(Point3d::new(0.0, 1.0, 0.0));
        perlin.noise(Point3d::new(0.0, 0.0, 1.0));
        perlin.noise(Point3d::new(-1.0, 0.0, 0.0));
        perlin.noise(Point3d::new(0.0, -1.0, 0.0));
        perlin.noise(Point3d::new(0.0, 0.0, -1.0));
        perlin.noise(Point3d::new(0.5, 0.5, 0.5));
        perlin.noise(Point3d::new(0.0, 0.0, 0.25));
    }
}
