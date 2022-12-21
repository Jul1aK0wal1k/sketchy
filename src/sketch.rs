use crate::{hash::Hash, matrix::Matrix};
use rand::Rng;
use std::{
    iter::{repeat_with, zip},
    marker::PhantomData,
};

pub trait Sketch<Count, Key> {
    type Hash: crate::hash::Hash<Key>;

    fn add(&mut self, key: &Key, times: Count);
    fn remove(&mut self, key: &Key, times: Count);
    fn estimate(&self, key: &Key) -> Count;
}

pub struct CountMinSketch<HashStrategy, Count>
where
    Count: Sized + Copy + Default,
{
    rows: usize,
    columns: usize,
    arr: Matrix<Count, usize>,
    total: u64,
    seeds: Vec<u64>,
    _hash: PhantomData<HashStrategy>,
}

impl<HashStrategy, Count> CountMinSketch<HashStrategy, Count>
where
    Count: Sized + Copy + Default,
{
    pub fn new(rows: usize, columns: usize, seeds: Option<Vec<u64>>) -> Self {
        let _seeds = seeds
            .or_else(|| {
                let mut rand = rand::thread_rng();
                Some(
                    repeat_with(|| rand.gen::<u64>())
                        .take(rows as usize)
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap();

        CountMinSketch {
            rows,
            columns,
            arr: Matrix::zeros(rows, columns),
            total: Default::default(),
            seeds: _seeds,
            _hash: PhantomData,
        }
    }
}

impl<HashStrategy, Count, Key> Sketch<Count, Key> for CountMinSketch<HashStrategy, Count>
where
    HashStrategy: Hash<Key>,
    Count: Copy + Default,
    Key: Copy + Default,
    HashStrategy::Hash: num_traits::PrimInt,
{
    type Hash = HashStrategy;

    fn add(&mut self, key: &Key, times: Count) {
        let columns = (0..(self.rows as usize))
            .map(|i| HashStrategy::hash_with_seed(key, i as usize))
            .collect::<Vec<_>>();
        let indices = zip(0..(self.rows as usize), columns).collect::<Vec<_>>();

        for (r, c) in indices {
            self.arr[(r as usize, c as usize)] += times;
        }
        self.total += times as u64;
    }

    fn remove(&mut self, key: &Key, times: Count) {
        todo!()
    }

    fn estimate(&self, key: &Key) -> Count {
        todo!()
    }
}
