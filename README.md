# gla
## Overview
`gla` is a **G**eneric **L**inear **A**lgebra library that aims to leverage Rust's powerful type system to express generic mathematical structures used for linear algebra (matrices, vectors, etc.).

## The aim of this project
`gla`'s main focus is exploring the capabilities and expresiveness of Rust's type system, so do not expect performance comparable to mainstream linear algebra libraries.

## Quick Start
After cloning this repository, you can run the matrix demo with
```console
  $ cargo run --example matrix_demo
```

(Taken from [examples/matrix_demo.rs](./examples/matrix_demo.rs))
```rs
use gla::*;
pub fn main() {
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

    // Mind the turbofish syntax here, this means that any type
    // that implements Matrix<3, 3> could be used here.
    let m1_plus_m2 = m1.add::<Matrix3x3>(&m2);
    println!("m1 + m2: \n{:?}\n", m1_plus_m2);
    println!("(m1 + m2)^T: \n{:?}", m1_plus_m2.transpose());
}
```
