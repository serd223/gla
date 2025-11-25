use gla::*;
pub fn main() {
    // Users only need to implement three methods to implement
    // `Matrix` for their type:
    //   * `fn zeroed() -> Self;`
    //   * `fn set_entry(&mut self, i: usize, j: usize, n: f32);`
    //   * `fn get_entry(&self, i: usize, j: usize) -> f32;`
    // All other methods of `Matrix` have default implementations that
    // only make use of the three methods mentioned above, your type
    // doesn't even have to be `Sized` as long as you can specify a
    // `Sized` return type for the arithmetical operations.

    // `Matrix3x3` is a very simple struct that implements the
    // trait `SquareMatrix<3>` which is a special case of the
    // trait `Matrix<3, 3>`. The `identity` constructor used here
    // actually comes from the trait `SquareMatrix`.
    let m1 = Matrix3x3::identity();
    println!("m1: \n{m1:?}\n");

    let m2 = Matrix3x3 {
        #[rustfmt::skip]
        entries: [
            [0., 0., 2.],
            [0., 1., 0.],
            [0., 3., 0.]
        ],
    };
    println!("m2: \n{m2:?}\n");

    // Mind the turbofish syntax, this means that any type
    // that implements Matrix<3, 3> could be used as a
    // return type here.
    let m1_plus_m2 = m1.add::<Matrix3x3>(&m2);
    println!("m1 + m2: \n{:?}\n", m1_plus_m2);
    println!("(m1 + m2)^T: \n{:?}", m1_plus_m2.transpose());
}
