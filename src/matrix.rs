use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};
use std::process::Output;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Matrix<T: Clone> {
    pub rows: usize,
    pub cols: usize,
    values: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, v: Vec<T>) -> Option<Matrix<T>> {
        if v.len() != rows * cols {
            None
        } else {
            Some(Matrix {
                rows,
                cols,
                values: v,
            })
        }
    }
}

impl<S: Clone, T: Clone + Add<Output = S>> Add for Matrix<T> {
    type Output = Option<Matrix<S>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            None
        } else {
            let mut v = Vec::new();
            for (a, b) in self.values.iter().zip(rhs.values.iter()) {
                v.push(a.clone() + b.clone());
            }
            Some(Matrix {
                rows: self.rows,
                cols: self.cols,
                values: v,
            })
        }
    }
}

impl<S: Clone, T: Clone + Sub<Output = S>> Sub for Matrix<T> {
    type Output = Option<Matrix<S>>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            None
        } else {
            let mut v = Vec::new();
            for (a, b) in self.values.iter().zip(rhs.values.iter()) {
                v.push(a.clone() - b.clone());
            }
            Some(Matrix {
                rows: self.rows,
                cols: self.cols,
                values: v,
            })
        }
    }
}

impl<T: Clone + Neg> Neg for Matrix<T>
where
    Vec<T>: FromIterator<<T as Neg>::Output>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            values: self.values.iter().map(|x| -x.clone()).collect::<Vec<T>>(),
        }
    }
}

impl<S: Clone + Add<Output = S>, T: Clone + Mul<Output = S>> Mul for Matrix<T> {
    type Output = Option<Matrix<S>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            None
        } else {
            let mut v = Vec::new();

            for lrow in 0..self.rows {
                for rcol in 0..rhs.cols {
                    let mut sum = self.values[lrow * self.cols].clone() * rhs.values[rcol].clone();
                    for lcol in 1..self.cols {
                        sum = sum
                            + self.values[lrow * self.cols + lcol].clone()
                                * rhs.values[lcol * rhs.cols + rcol].clone();
                    }
                    v.push(sum);
                }
            }
            Some(Matrix {
                rows: self.rows,
                cols: rhs.cols,
                values: v,
            })
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Clone + ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in 0..self.rows {
            s += &self.values[(row * self.cols)..(row * self.cols + self.cols)]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\t");
            s += "\n";
        }
        s.pop().unwrap();
        write!(f, "{}", s)
    }
}
