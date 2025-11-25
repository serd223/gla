use std::ops::MulAssign;

use super::{SquareMatrix, Vector3};

#[derive(Clone)]
pub struct Pane {
    v1: Vector3,
    v2: Vector3,
    n: Vector3,
}

impl Pane {
    pub fn new(v1: Vector3, v2: Vector3) -> Self {
        Self {
            v1,
            v2,
            n: v1.cross(v2).unit(),
        }
    }

    pub fn n(&self) -> Vector3 {
        self.n
    }

    pub fn projection(&self, p: Vector3) -> Vector3 {
        // p' = p + t.n
        // p' . n = 0
        // (p + t.n) . n = 0
        // px.nx + nx^2.t + py.ny + ny^2.t + pz.nz + nz^2.t = 0
        let t = -(p * self.n) / (self.n * self.n);
        p + self.n * t
    }
}

impl<T: SquareMatrix<3>> MulAssign<T> for Pane {
    fn mul_assign(&mut self, rhs: T) {
        self.v1 = rhs.mat_mul(&self.v1);
        self.v2 = rhs.mat_mul(&self.v2);
        self.n = rhs.mat_mul(&self.n);
        self.n = self.n.unit();
    }
}
