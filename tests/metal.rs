extern crate assert;
extern crate blas;

use blas::metal;

#[test]
fn dgemv() {
    let (m, n) = (2, 3);

    let a = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let x = vec![1.0, 2.0, 3.0];
    let mut y = vec![6.0, 8.0];

    metal::dgemv(metal::Trans::N, m, n, 1.0, &a, m, &x, 1, 1.0, &mut y, 1);

    let expected_y = vec![20.0, 40.0];
    assert::equal(&y, &expected_y);
}

#[test]
fn dgemm() {
    let (m, n, k) = (2, 4, 3);

    let a = vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0];
    let b = vec![1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 4.0, 8.0, 12.0];
    let mut c = vec![2.0, 7.0, 6.0, 2.0, 0.0, 7.0, 4.0, 2.0];

    metal::dgemm(metal::Trans::N, metal::Trans::N, m, n, k, 1.0, &a, m, &b, k, 1.0, &mut c, m);

    let expected_c = vec![40.0, 90.0, 50.0, 100.0, 50.0, 120.0, 60.0, 130.0];
    assert::equal(&c, &expected_c);
}