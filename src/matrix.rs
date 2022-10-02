use crate::utils::{debug_bounds_check, real_index};
use std::ops::{Index, IndexMut};

pub(crate) type Idx = (usize, usize);

pub(crate) struct Matrix<T: Sized + Default + Copy> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Sized + Default + Copy> Matrix<T> {
    pub(crate) fn zeros(rows: usize, columns: usize) -> Self {
        let cap = rows * columns;
        let data = vec![T::default(); cap];
        Matrix {
            data,
            rows,
            columns,
        }
    }

    pub(crate) const fn shape(&self) -> Idx {
        (self.rows, self.columns)
    }

    #[inline(always)]
    pub(crate) unsafe fn get_unchecked(&self, index: Idx) -> &T {
        let (i, j) = index;
        let real_idx = real_index(i, j, self.columns);

        debug_bounds_check(real_idx, &self.data);
        unsafe { self.data.get_unchecked(real_idx) }
    }

    #[inline(always)]
    pub(crate) unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut T {
        let (i, j) = index;
        let real_idx = real_index(i, j, self.columns);

        debug_bounds_check(real_idx, &self.data);
        unsafe { self.data.get_unchecked_mut(real_idx) }
    }
}

impl<T> Index<Idx> for Matrix<T>
where
    T: Sized + Default + Copy,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        unsafe { self.get_unchecked(index) }
    }
}

impl<T> IndexMut<Idx> for Matrix<T>
where
    T: Sized + Default + Copy,
{
    #[inline(always)]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index) }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn test_zeros_matrix() {
        let _m = Matrix::<u32>::zeros(10, 10);
    }
}
