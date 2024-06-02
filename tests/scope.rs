use std::vec;

use scopefn::prelude::*;

#[test]
fn test_apply() {
    let x = 42;

    x.apply(|x| {
        assert_eq!(*x, 42);
    });
}

#[test]
fn test_apply_mut() {
    let x = vec![42];

    let y = x.apply_mut(|x| {
        x.push(43);
    });

    assert_eq!(y, vec![42, 43]);
}

#[test]
fn test_apply_useage() {
    let v = vec![1, 1, 4, 5, 1, 4].apply_mut(|v| v.sort());

    assert_eq!(v, vec![1, 1, 1, 4, 4, 5]);
}

#[test]
fn test_example_usage() {
    fn sort_with_debug(data: Vec<i32>) -> Vec<i32> {
        data.apply(|v| println!("Before sort: {:?}", v))
            .apply_mut(|v| v.sort())
            .apply(|data| println!("After sort: {:?}", data))
    }

    let data = vec![1, 1, 4, 5, 1, 4];

    let sorted_data = sort_with_debug(data);

    assert_eq!(sorted_data, vec![1, 1, 1, 4, 4, 5]);
}

#[test]
fn test_run() {
    let x = 42;

    let y = x.run(|x| x + 1);

    assert_eq!(y, 43);
}

#[test]
fn test_run_vec() {
    let x = vec![1, 2, 3];

    let y: i32 = x.run(|x| x.iter().sum());

    assert_eq!(y, 6);
}

#[test]
fn test_run_usage() {
    let x = vec![1, 2, 3];

    let y = x.run(|x| x.iter().count()).run(|x| x + 1);

    assert_eq!(y, 4);
}

#[test]
fn test_take_if() {
    let x = 42;

    let some = x.take_if(|x| x % 2 == 0);
    let none = x.take_if(|x| x % 2 != 0);

    assert_eq!(some, Some(42));
    assert_eq!(none, None);
}


#[test]
fn test_take_unless() {
    let x = 42;

    let some = x.take_unless(|x| x % 2 != 0);
    let none = x.take_unless(|x| x % 2 == 0);

    assert_eq!(some, Some(42));
    assert_eq!(none, None);
}