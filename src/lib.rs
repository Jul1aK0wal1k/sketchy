mod hash;
mod matrix;
mod sketch;
mod utils;
use matrix::Matrix;
use rand::Rng;
use sketch::Sketch;

use std::{f64::consts::E as EULER, iter::repeat_with, iter::zip, marker::PhantomData};

// pub struct CountMinSketch<HashStrategy, MatrixElement>
// where
//     MatrixElement: Sized + Copy + Default,
// {
//     rows: usize,
//     columns: usize,
//     arr: Matrix<MatrixElement>,
//     total: u64,
//     seeds: Vec<u64>,
//     _hash: PhantomData<HashStrategy>,
// }

// impl<HashStrategy, MatrixElement> CountMinSketch<HashStrategy, MatrixElement>
// where
//     HashStrategy: hash::Hash,
//     MatrixElement: Sized + Copy + Default,
// {
//     pub fn new(rows: usize, columns: usize) -> Self {
//         let arr = Matrix::zeros(rows, columns);
//         let mut rand = rand::thread_rng();
//         let mut seeds = Vec::with_capacity(rows as usize);
//         seeds.extend(repeat_with(|| rand.gen::<u64>()).take(rows as usize));
//         CountMinSketch {
//             rows,
//             columns,
//             arr,
//             total: 0,
//             seeds,
//             _hash: PhantomData,
//         }
//     }

//     // fn key_hash<Key: AsRef<[u8]> + ?Sized>(&self, key: &Key, seed_index: usize) -> usize {
//     //     (hash::Hash::hash_with_seed(key, self.seeds[seed_index]) % self.columns as u32) as usize
//     // }

//     pub fn error_value(&self) -> f64 {
//         self.total as f64 * (EULER / self.rows as f64)
//     }

//     pub fn error_probability(&self) -> f64 {
//         1f64 - (1f64 / EULER.powf(self.columns as f64))
//     }

//     pub fn error(&self) -> (f64, f64) {
//         (self.error_value(), self.error_probability())
//     }
// }

// impl<Hash, Key> Sketch<Hash, Key> for CountMinSketch<Hash, u64>
// where
//     Hash: hash::Hash + Default,
//     Key: AsRef<[u8]>,
// {
//     fn add(&mut self, key: &Key, times: u64) {
//         todo!()
//         // let columns = (0..(self.rows as usize))
//         //     .map(|i| self.key_hash(key, i as usize))
//         //     .collect::<Vec<_>>();
//         // let indices = zip(0..(self.rows as usize), columns).collect::<Vec<_>>();

//         // for (r, c) in indices {
//         //     self.arr[(r as usize, c as usize)] += times;
//         // }
//         // self.total += times as u64;
//     }

//     fn remove(&mut self, key: &Key, times: u64) {
//         todo!()
//         // let columns = (0..(self.rows as usize))
//         //     .map(|i| self.key_hash(key, i as usize))
//         //     .collect::<Vec<_>>();
//         // let indices = zip(0..(self.rows as usize), columns).collect::<Vec<_>>();

//         // for (r, c) in indices {
//         //     self.arr[(r as usize, c as usize)] -= times;
//         // }
//         // self.total -= 1;
//     }

//     fn estimate(&self, key: &Key) -> u64 {
//         todo!()
//         // let mut min_est = u32::MAX;
//         // let columns = (0..(self.rows as usize))
//         //     .map(|i| self.key_hash(key, i as usize))
//         //     .collect::<Vec<_>>();

//         // for (r, c) in zip(0..(self.rows as usize), columns) {
//         //     if self.arr[(r, c)] < min_est {
//         //         min_est = self.arr[(r, c)];
//         //     }
//         // }
//         // min_est
//     }
// }

// #[cfg(test)]
// mod tests {

//     // use ndarray::Array2;

//     // #[test]
//     // fn test_array_slice() {
//     //     let array = Array2::<u32>::zeros((2, 2));
//     //     let indices1 = vec![0u32, 1u32];
//     //     let indices2 = vec![1u32, 0u32];
//     // }
// }
