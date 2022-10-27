#![feature(test)]

extern crate test;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
struct Matrix<T> {
    h: usize,
    w: usize,
    data: Vec<T>,
}

/// xxx   xx     xx
/// xxx x xx ==> xx  left(h=2,w=3) x right(h=3,w=2) = result(h=2,w=2)
///       xx
#[allow(dead_code)]
impl<T> Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Debug,
{
    fn empty(h: usize, w: usize) -> Matrix<T> {
        Matrix {
            h,
            w,
            data: Vec::with_capacity(h * w),
        }
    }

    pub fn new(h: usize, w: usize, data: Vec<T>) -> Matrix<T> {
        Matrix { h, w, data }
    }

    // index count from zero
    fn get_index(&self, h: usize, w: usize) -> usize {
        self.w * h + w
    }

    // index count from zero
    fn get_value(&self, h: usize, w: usize) -> T {
        let index = self.get_index(h, w);
        self.data[index]
    }

    // index count from zero
    fn add_value(&mut self, h: usize, w: usize, value: T) {
        let index = self.get_index(h, w);

        if let Some(elem) = self.data.get_mut(index) {
            *elem = *elem + value;
        } else {
            self.data.insert(index, value);
        }
    }

    // most simplest impl
    pub fn multiply_v1(&self, right: &Matrix<T>) -> Matrix<T> {
        if self.w != right.h {
            panic!("w should eq h when matrix multiply");
        }

        let res_h = self.h;
        let res_w = right.w;
        let common = self.w;
        let mut result = Matrix::empty(res_h, res_w);

        for i in 0..res_h {
            for j in 0..res_w {
                for k in 0..common {
                    let value = self.get_value(i, k) * right.get_value(k, j);
                    result.add_value(i, j, value);
                }
            }
        }

        result
    }

    // cache friendly optimization
    pub fn multiply_v2(&self, right: &Matrix<T>) -> Matrix<T> {
        if self.w != right.h {
            panic!("w should eq h when matrix multiply");
        }

        let res_h = self.h;
        let res_w = right.w;
        let common = self.w;
        let mut result = Matrix::empty(res_h, res_w);

        for i in 0..res_h {
            for k in 0..common {
                for j in 0..res_w {
                    let value = self.get_value(i, k) * right.get_value(k, j);
                    result.add_value(i, j, value);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_left() -> Matrix<i32> {
        Matrix::new(2, 3, vec![1, 2, 1, 2, 6, 9])
    }

    fn get_right() -> Matrix<i32> {
        Matrix::new(3, 4, vec![0, 7, 9, 1, 4, 9, 2, 5, 8, 1, 2, 1])
    }

    fn get_result() -> Matrix<i32> {
        Matrix::new(2, 4, vec![16, 26, 15, 12, 96, 77, 48, 41])
    }

    #[bench]
    fn bench_multiply_v1(b: &mut Bencher) {
        let left = get_left();
        let right = get_right();
        b.iter(|| left.multiply_v1(&right))
    }

    #[bench]
    fn bench_multiply_v2(b: &mut Bencher) {
        let left = get_left();
        let right = get_right();
        b.iter(|| left.multiply_v2(&right))
    }

    #[test]
    fn multiply_v1() {
        let v1_result = get_left().multiply_v1(&get_right());
        assert_eq!(get_result(), v1_result);
    }

    #[test]
    fn multiply_v2() {
        let v2_result = get_left().multiply_v2(&get_right());
        assert_eq!(get_result(), v2_result);
    }
}

fn main() {
    //
}
