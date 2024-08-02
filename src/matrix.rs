use crate::vector::Vector;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul};

pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

// pretend this is a heavy operation
pub fn dot_product<T>(mxa: Vector<T>, mxb: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if mxa.len() != mxb.len() {
        return Err(anyhow!(
            "第一个矩阵的列数（column）和第二个矩阵的行数（row）不相同！"
        ));
    }

    let mut sum = T::default();
    for i in 0..mxa.len() {
        sum += mxa[i] * mxb[i];
    }

    Ok(sum)
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
            let row = Vector::new(&mxa.data[i * mxa.col..(i + 1) * mxa.col]);
            let col_data = mxb.data[j..]
                .iter()
                .step_by(mxb.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            data[i * mxb.col + j] += dot_product(row, col)?;
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
