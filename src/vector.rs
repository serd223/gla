use super::Matrix;

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// TODO: Create special derive macro that does this for any Matrix (since a blanket impl isn't possible due to the Orphan Rule)
impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Matrix::add(&self, &rhs)
    }
}

// TODO: Create special derive macro that does this for any Matrix (since a blanket impl isn't possible due to the Orphan Rule)
impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::sub(&self, &rhs)
    }
}

impl std::ops::Mul for Vector3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn norm_square(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(self) -> f32 {
        self.norm_square().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.norm()
    }

    pub fn distance(self, rhs: Self) -> f32 {
        ((self.x - rhs.x) * (self.x - rhs.x)
            + (self.y - rhs.y) * (self.y - rhs.y)
            + (self.z - rhs.z) * (self.z - rhs.z))
            .sqrt()
    }
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3::new(x, y, z)
}

impl Matrix<3, 1> for Vector3 {
    fn zeroed() -> Vector3 {
        vec3(0., 0., 0.)
    }

    fn get_entry(&self, i: usize, j: usize) -> f32 {
        match j {
            0 => match i {
                0 => self.x,
                1 => self.y,
                2 => self.z,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn set_entry(&mut self, i: usize, j: usize, n: f32) {
        match j {
            0 => match i {
                0 => self.x = n,
                1 => self.y = n,
                2 => self.z = n,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const V1: Vector3 = vec3(10., 10., 10.);
    const V2: Vector3 = vec3(50., -10., 20.);
    const V3: Vector3 = vec3(20., 20., 20.);

    #[test]
    fn vector3_dot() {
        let result = V1 * V2;
        assert_eq!(result, 600.);
    }

    #[test]
    fn vector3_cross() {
        let result = V1.cross(V3);
        assert_eq!(result.x, 0.);
        assert_eq!(result.y, 0.);
        assert_eq!(result.z, 0.);
    }

    #[test]
    fn vector3_add() {
        let result = V1 + V2;
        assert_eq!(result.x, 60.);
        assert_eq!(result.y, 0.);
        assert_eq!(result.z, 30.);
    }

    #[test]
    fn vector3_sub() {
        let result = V1 - V2;
        assert_eq!(result.x, -40.);
        assert_eq!(result.y, 20.);
        assert_eq!(result.z, -10.);
    }
}
