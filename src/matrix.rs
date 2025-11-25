pub trait Matrix<const N: usize, const M: usize> {
    fn zeroed() -> Self;
    fn set_entry(&mut self, i: usize, j: usize, n: f32);
    fn get_entry(&self, i: usize, j: usize) -> f32;

    fn mat_mul<const R: usize, T: Matrix<N, R>>(&self, rhs: &impl Matrix<M, R>) -> T {
        let mut res: T = T::zeroed();
        for i in 0..N {
            for j in 0..R {
                let mut n = 0.;
                for k in 0..M {
                    n += self.get_entry(i, k) * rhs.get_entry(k, j);
                }
                res.set_entry(i, j, n);
            }
        }
        res
    }

    fn add<T: Matrix<N, M>>(&self, rhs: &impl Matrix<N, M>) -> T {
        let mut res = T::zeroed();
        for i in 0..N {
            for j in 0..M {
                let a = self.get_entry(i, j);
                let b = rhs.get_entry(i, j);
                res.set_entry(i, j, a + b);
            }
        }

        res
    }

    fn sub<T: Matrix<N, M>>(&self, rhs: &impl Matrix<N, M>) -> T {
        let mut res = T::zeroed();
        for i in 0..N {
            for j in 0..M {
                let a = self.get_entry(i, j);
                let b = rhs.get_entry(i, j);
                res.set_entry(i, j, a - b);
            }
        }

        res
    }
}

// TODO: implement determinant
pub trait SquareMatrix<const N: usize>: Matrix<N, N> {
    fn transpose(mut self) -> Self
    where
        Self: Sized,
    {
        self.transpose_mut();
        self
    }

    fn transpose_mut(&mut self) {
        for i in 0..N {
            // 0 1 2
            for j in 0..i {
                let n = self.get_entry(i, j);
                self.set_entry(i, j, self.get_entry(j, i));
                self.set_entry(j, i, n);
            }
        }
    }

    fn identity() -> Self
    where
        Self: Sized,
    {
        let mut result = Self::zeroed();

        for i in 0..N {
            result.set_entry(i, i, 1.);
        }

        result
    }
}

impl<const N: usize, T: Matrix<N, N>> SquareMatrix<N> for T {}

#[derive(Clone)]
pub struct Matrix3x3 {
    pub entries: [[f32; 3]; 3],
}

// TODO: Create special derive macro that does this for any Matrix (since a blanket impl isn't possible due to the Orphan Rule)
impl core::fmt::Debug for Matrix3x3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut max_l = 0;
        for i in 0..3 {
            for j in 0..3 {
                let l = format!("{:.3}", self.entries[i][j]).len();
                max_l = l.max(max_l);
            }
        }
        writeln!(f, "┏{}┓", " ".repeat(max_l * 3 + 2))?;
        for i in 0..3 {
            writeln!(
                f,
                "|{:<width$.3} {:<width$.3} {:<width$.3}|",
                self.entries[i][0],
                self.entries[i][1],
                self.entries[i][2],
                width = max_l
            )?;
        }
        write!(f, "┗{}┛", " ".repeat(max_l * 3 + 2))
    }
}

impl Matrix3x3 {
    #[rustfmt::skip]
    pub fn rotation_x(angle: f32) -> Self {
        Self {
            entries: [
                [1., 0.         , 0.          ],
                [0., angle.cos(), -angle.sin()],
                [0., angle.sin(), angle.cos() ],
            ],
        }
    }

    #[rustfmt::skip]
    pub fn rotation_y(angle: f32) -> Self {
        Self {
            entries: [
                [angle.cos() , 0., angle.sin()],
                [0.          , 1., 0.         ],
                [-angle.sin(), 0., angle.cos()],
            ],
        }
    }

    #[rustfmt::skip]
    pub fn rotation_z(angle: f32) -> Self {
        Self {
            entries: [
                [angle.cos(), -angle.sin(), 0.],
                [angle.sin(), angle.cos() , 0.],
                [0.         , 0.          , 1.],
            ],
        }
    }
}

impl Matrix<3, 3> for Matrix3x3 {
    fn zeroed() -> Self {
        Self {
            entries: [[0.; 3]; 3],
        }
    }

    fn get_entry(&self, i: usize, j: usize) -> f32 {
        self.entries[i][j]
    }

    fn set_entry(&mut self, i: usize, j: usize, n: f32) {
        self.entries[i][j] = n;
    }
}
