mod matrix;
mod utils;
mod sketch;
mod hash;
use matrix::Matrix;
use rand::Rng;

use std::{f64::consts::E as EULER, iter::repeat_with, iter::zip};

pub struct CountMinSketch {
    rows: usize,
    columns: usize,
    arr: Matrix<u32>,
    total: u64,
    seeds: Vec<u32>,
}

impl CountMinSketch {
    pub fn new(rows: usize, columns: usize) -> Self {
        let arr = Matrix::zeros(rows, columns);
        let mut rand = rand::thread_rng();
        let mut seeds = Vec::with_capacity(rows as usize);
        seeds.extend(repeat_with(|| rand.gen::<u32>()).take(rows as usize));
        CountMinSketch {
            rows,
            columns,
            arr,
            total: 0,
            seeds,
        }
    }

    pub fn add<Key: AsRef<[u8]> + ?Sized>(&mut self, key: &Key, times: u32) {
        let columns = (0..(self.rows as usize))
            .map(|i| self.key_hash(key, i as usize))
            .collect::<Vec<_>>();
        let indices = zip(0..(self.rows as usize), columns).collect::<Vec<_>>();

        for (r, c) in indices {
            self.arr[(r as usize, c as usize)] += times;
        }
        self.total += times as u64;
    }

    pub fn remove<Key: AsRef<[u8]> + ?Sized>(&mut self, key: &Key, times: u32) {
        let columns = (0..(self.rows as usize))
            .map(|i| self.key_hash(key, i as usize))
            .collect::<Vec<_>>();
        let indices = zip(0..(self.rows as usize), columns).collect::<Vec<_>>();

        for (r, c) in indices {
            self.arr[(r as usize, c as usize)] -= times;
        }
        self.total -= 1;
    }

    pub fn estimate<Key: AsRef<[u8]> + ?Sized>(&self, key: &Key) -> u32 {
        let mut min_est = u32::MAX;
        let columns = (0..(self.rows as usize))
            .map(|i| self.key_hash(key, i as usize))
            .collect::<Vec<_>>();

        for (r, c) in zip(0..(self.rows as usize), columns) {
            if self.arr[(r, c)] < min_est {
                min_est = self.arr[(r, c)];
            }
        }
        min_est
    }

    fn key_hash<Key: AsRef<[u8]> + ?Sized>(&self, key: &Key, seed_index: usize) -> usize {
        (hash32_with_seed(key, self.seeds[seed_index]) % self.columns as u32) as usize
    }

    pub fn error_value(&self) -> f64 {
        self.total as f64 * (EULER / self.rows as f64)
    }

    pub fn error_probability(&self) -> f64 {
        1f64 - (1f64 / EULER.powf(self.columns as f64))
    }

    pub fn error(&self) -> (f64, f64) {
        (self.error_value(), self.error_probability())
    }
}

#[cfg(test)]
mod tests {

    // use ndarray::Array2;

    // #[test]
    // fn test_array_slice() {
    //     let array = Array2::<u32>::zeros((2, 2));
    //     let indices1 = vec![0u32, 1u32];
    //     let indices2 = vec![1u32, 0u32];
    // }
}
