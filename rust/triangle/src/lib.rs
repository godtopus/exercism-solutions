extern crate num;
use num::{Num, NumCast, Zero};

use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Type {
    Equilateral,
    Isosceles,
    Scalene
}

#[derive(Debug)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
    kind: Type,
}

impl<T: Num + NumCast + Copy + PartialOrd> Triangle<T> {
    pub fn build(mut sides: [T; 3]) -> Result<Self, String> {
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
        let sides = (sides[0], sides[1], sides[2]);

        match sides {
            (a, b, c) if Zero::is_zero(&a) || Zero::is_zero(&b) || Zero::is_zero(&c) => Err("One or more of the sides were zero.".into()),
            (a, b, c) if a + b < c => Err("Triangle was degenerate.".into()),
            (a, b, c) if a == b && b == c => Ok(Triangle::construct(sides, Type::Equilateral)),
            (a, b, c) if a == b || b == c || a == c => Ok(Triangle::construct(sides, Type::Isosceles)),
            sides => Ok(Triangle::construct(sides, Type::Scalene))
        }
    }

    fn heron((a, b, c): (T, T, T)) -> f64 {
        let s = (a + b + c) / NumCast::from(2usize).unwrap();
        let a_squared: f64 = NumCast::from(s * (s - a) * (s - b) * (s - c)).unwrap();
        a_squared.sqrt()
    }

    pub fn is_equilateral(&self) -> bool {
        self.kind == Type::Equilateral
    }

    pub fn is_isosceles(&self) -> bool {
        self.kind == Type::Isosceles
    }

    pub fn is_scalene(&self) -> bool {
        self.kind == Type::Scalene
    }

    fn construct(sides: (T, T, T), kind: Type) -> Self {
        Triangle {
            a: sides.0,
            b: sides.1,
            c: sides.2,
            kind: kind,
        }
    }
}