use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

use rand::Rng;

fn rand() -> f32 {
    rand::thread_rng().gen::<f32>()
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
pub type Color = Vec3;

// ADDITION
impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

// MULTIPLICATION
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

// DIVISION
impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        let reciprocal = 1.0 / other;

        self * reciprocal
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self {
        Self {
            x: self.x * (1.0 / other.x),
            y: self.y * (1.0 / other.y),
            z: self.z * (1.0 / other.z),
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        let reciprocal = 1.0 / other;

        *self *= reciprocal;
    }
}

// SUBTRACTION / NEGATION
impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<i32> for Vec3 {
    type Output = f32;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Index out of bounds");
            }
        }
    }
}

impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => {
                panic!("Index out of bounds");
            }
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Vec3 { x, y, z };
    }

    pub fn sqrt(&self) -> Self {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn new_fromi32(x: i32, y: i32, z: i32) -> Self {
        return Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        };
    }

    // The square root of the sum of the squares of the vectors values.
    // Otherwise known as the pythagorean theorem lol.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn unit(self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn random() -> Self {
        Vec3 {
            x: rand(),
            y: rand(),
            z: rand(),
        }
    }

    pub fn rand_range(min: f32, max: f32) -> Self {
        let diff = max - min;

        Vec3::random() * diff + min
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::rand_range(-1.0, 1.0);

        while p.length_squared() > 1.0 {
            p = Vec3::rand_range(-1.0, 1.0);
        }

        p
    }

    pub fn random_unit_vec() -> Vec3 {
        Vec3::rand_in_unit_sphere().unit()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self[0].abs() < s && self[1].abs() < s && self[2].abs() < s;
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        v - 2.0 * v.dot(&n) * n
    }

    // vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
    //     auto cos_theta = fmin(dot(-uv, n), 1.0);
    //     vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    //     vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n;
    //     return r_out_perp + r_out_parallel;
    // }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-uv).dot(&n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_paralell = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
        r_out_perp + r_out_paralell
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rand_vec = Vec3::new(rand(), rand(), 0.0);

        while rand_vec.length_squared() >= 1.0 {
            rand_vec = Vec3::new(rand(), rand(), 0.0);
        }

        rand_vec
    }
}

fn clamp(n: f32, min: f32, max: f32) -> f32 {
    if n < min {
        return min;
    }
    if n > max {
        return max;
    }
    n
}

impl Color {
    pub fn as_color_triplet(&self) -> String {
        format!("{} {} {}", self.x * 255.0, self.y * 255.0, self.z * 255.0)
    }

    pub fn clamped(&self) -> Self {
        Color {
            x: clamp(self.x, 0.0, 255.0),
            y: clamp(self.y, 0.0, 255.0),
            z: clamp(self.z, 0.0, 255.0),
        }
    }
}
