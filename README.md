# gla
## Overview
`gla` is a **G**eneric **L**inear **A**lgebra library that aims to leverage Rust's powerful type system to express generic mathematical structures used for linear algebra (matrices, vectors, etc.).

## The aim of this project
`gla`'s main focus is exploring the capabilities and expresiveness of Rust's type system, so do not expect performance comparable to mainstream linear algebra libraries.

## Quick Start
After cloning this repository, you can run the matrix demo with:
```console
  $ cargo run --example matrix_demo
```

(Taken from [examples/matrix_demo.rs](./examples/matrix_demo.rs))
```rs
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
```
The output will look like this:
```
m1: 
┏                 ┓
|1.000 0.000 0.000|
|0.000 1.000 0.000|
|0.000 0.000 1.000|
┗                 ┛

m2: 
┏                 ┓
|0.000 0.000 2.000|
|0.000 1.000 0.000|
|0.000 3.000 0.000|
┗                 ┛

m1 + m2: 
┏                 ┓
|1.000 0.000 2.000|
|0.000 2.000 0.000|
|0.000 3.000 1.000|
┗                 ┛

(m1 + m2)^T: 
┏                 ┓
|1.000 0.000 0.000|
|0.000 2.000 3.000|
|2.000 0.000 1.000|
┗                 ┛
```
