//! This is a very simple example to demonstrate memory zerozing.

extern crate zeroize;

use std::mem;
use zeroize::Zeroize;

#[derive(Zeroize, Debug)]
#[zeroize(drop)]
struct Data1([u64; 4]);

#[derive(Debug)]
struct Data2([u64; 4]);

fn main() {
    let tmp = [1u64, 2, 3, 4];

    let data1 = Data1(tmp);
    println!("original data: {:?}\n", data1.0);

    // with zeroize, the memory will be cleaned after use
    // we are expecting 0s from the output
    let t = foo1(data1);
    unsafe {
        println!("should be zeroized: {:?}\n", (*t).0);
    }

    // without zeroize, the memory is not freed
    // we will see the data is leftover
    let data2 = Data2(tmp);
    let t = foo2(data2);
    unsafe {
        println!("the data is left in memory: {:?}\n", (*t).0);
    }

    // although we have derived zeroize, the memory will be not cleaned after use
    // in foo3 we gave up the onwership of data3, therefore, no destructor is called.
    let data3 = Data1(tmp);
    let t = foo3(data3);
    unsafe {
        println!("data is also left in memory: {:?}\n", (*t).0);
    }
}

fn foo1(d: Data1) -> *const Data1 {
    &d as *const Data1
}

fn foo2(d: Data2) -> *const Data2 {
    &d as *const Data2
}

fn foo3(d: Data1) -> *const Data1 {
    let t: *const Data1 = &d;
    mem::forget(d);
    t
}
