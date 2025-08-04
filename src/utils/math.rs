// ! 4.7 Mathematical functions

use std::ops::{Shl, Shr};





pub fn floor_log2<T: From<u8> + PartialEq + Shr<u8, Output = T>>(mut n: T) -> u8 {
    let mut s = 0;
    let zero = 0.into();
    while n != zero {
        n = n >> 1;
        s += 1;
    }
    s - 1
}


pub fn ceil_log2<T: From<u8> + PartialOrd + Shl<u8, Output = T>>(x: T) -> u8 {
    let two = 2.into();
    if x < two {
        return 0;
    }
    let mut i = 1;
    let mut p = two;
    while p < x {
        i += 1;
        p = p << 1;
    }
    i
}

pub fn clip3<T: PartialOrd>(x: T, y: T, z: T) -> T {
    if z < x {
        x
    } else if z > y {
        y
    } else {
        z
    }
}
