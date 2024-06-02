#![cfg(feature = "run_if")]

use either::Either::Right;
use scopefn::prelude::*;

#[test]
fn test_run_if() {
    let x = 42;

    let y = x.run_if(x % 2 == 0, |x| x + 1);

    assert_eq!(y, Right(43));
}

#[test]
fn test_run_if_usage() {
    let x = vec![1, 2, 3];
    let need_filter = true;

    let res = x
        .into_iter()
        .run_if(need_filter, |x| x.filter(|x| *x % 2 == 0))
        .collect::<Vec<_>>();

    assert_eq!(res, vec![2]);
}
