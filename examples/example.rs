use matrix::matrix::Matrix;
use matrix::string::String_;

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
