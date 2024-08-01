use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul};
use std::process::Output;

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

pub fn matrix_multiply<T>(mxa: &Matrix<T>, mxb: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Default,
{
    if mxa.col != mxb.row {
        return Err(anyhow!(
            "第一个矩阵的列数（column）和第二个矩阵的行数（row）不相同！"
        ));
    }

    let mut data = vec![T::default(); mxa.row * mxb.col];
    for i in 0..mxa.row {
        for j in 0..mxb.col {
            for k in 0..mxa.col {
                data[i * mxb.col + j] += mxb.data[i * mxa.col + k] * mxb.data[k * mxb.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: mxa.row,
        col: mxb.col,
    })
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T: Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let mxa = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let mxb = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let mxc = matrix_multiply(&mxa, &mxb)?;
        assert_eq!(mxc.col, 2);
        assert_eq!(mxc.row, 2);
        assert_eq!(format!("{:?}", mxc), "Matrix(row=2, col=2, {22 28, 49 64})");
        Ok(())
    }
}
