use crate::utils::{debug_bounds_check, real_index};
use std::ops;

fn foo() {
    ()
}

pub(crate) struct Matrix<Element, Idx>
where
    Element: Sized + Default + Copy,
{
    data: Vec<Element>,
    rows: Idx,
    columns: Idx,
}

impl<Element, Idx> Matrix<Element, Idx>
where
    Element: Sized + Default + Copy,
    Idx: num_traits::Unsigned,
{
    pub(crate) fn zeros(rows: Idx, columns: Idx) -> Self {
        let cap = rows * columns;
        let data = std::vec::from_elem(Default::default(), cap);

        Matrix {
            data,
            rows,
            columns,
        }
    }

    pub(crate) const fn shape(&self) -> (Idx, Idx) {
        (self.rows, self.columns)
    }

    #[inline(always)]
    pub(crate) unsafe fn get_unchecked(&self, index: (Idx, Idx)) -> &Element {
        let (i, j) = index;
        let real_idx = real_index(i, j, self.columns);

        debug_bounds_check(real_idx, &self.data);
        unsafe { self.data.get_unchecked(real_idx) }
    }

    #[inline(always)]
    pub(crate) unsafe fn get_unchecked_mut(&mut self, index: Idx) -> &mut Element {
        let (i, j) = index;
        let real_idx = real_index(i, j, self.columns);

        debug_bounds_check(real_idx, &self.data);
        unsafe { self.data.get_unchecked_mut(real_idx) }
    }
}

impl<Element, Idx> ops::Index<Idx> for Matrix<Element, Idx>
where
    Element: Sized + Default + Copy,
    Idx: num_traits::Unsigned,
{
    type Output = Element;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        unsafe { self.get_unchecked(index) }
    }
}

impl<Element, Idx> ops::IndexMut<Idx> for Matrix<Element, Idx>
where
    Element: Sized + Default + Copy,
    Idx: num_traits::Unsigned,
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
