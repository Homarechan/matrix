use std::fmt;
use std::ops::{Add, Deref, Mul, Neg, Sub};

#[derive(Clone)]
struct String_(String);

impl Deref for String_ {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String_> for String {
    fn into(self) -> String_ {
        String_(self)
    }
}

impl Add for String_ {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        String_ { 0: self.0 + &rhs.0 }
    }
}

impl fmt::Display for String_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Matrix<T: Clone> {
    pub rows: usize,
    pub cols: usize,
    values: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    fn new(rows: usize, cols: usize, v: Vec<T>) -> Option<Matrix<T>> {
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

impl<T: Clone + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.rows != other.rows || self.cols != other.cols {
            None
        } else {
            let mut v = Vec::new();
            for (a, b) in self.values.iter().zip(other.values.iter()) {
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

impl<T: Clone + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Self>;

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

impl<T: Clone + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Self>;

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

fn main() {
    let m1 = Matrix::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    let m2 = Matrix::new(3, 3, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]).unwrap();

    let m3 = m1.clone() + m2.clone();
    println!("{}\n+\n{}\n=\n{}", m1, m2, m3.unwrap());

    let m4 = m1.clone() - m2.clone();
    println!("{}\n-\n{}\n=\n{}", m1, m2, m4.unwrap());

    println!("{}", -m1.clone());

    let m5 = m1.clone() * m2.clone();
    println!("{}\n*\n{}\n=\n{}", m1, m2, m5.unwrap());

    let m6: Matrix<String_> = Matrix::new(
        2,
        2,
        vec![
            "abc".to_string().into(),
            "def".to_string().into(),
            "ghi".to_string().into(),
            "jkl".to_string().into(),
        ],
    )
    .unwrap();
    let m7: Matrix<String_> = Matrix::new(
        2,
        2,
        vec![
            "mno".to_string().into(),
            "pqr".to_string().into(),
            "stu".to_string().into(),
            "vwx".to_string().into(),
        ],
    )
    .unwrap();
    let m8 = m6.clone() + m7.clone();
    println!("{}\n+\n{}\n=\n{}", m6, m7, m8.unwrap());
}
