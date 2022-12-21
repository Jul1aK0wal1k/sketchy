pub trait Hash<T> {
    type Hash;
    type Seed;

    fn hash_with_seed(bytes: &T, seed: Self::Seed) -> Self::Hash;

    fn hash(bytes: &T) -> Self::Hash;
}

impl<Hasher, Value> Hash<Value> for Hasher
where
    Hasher: std::hash::Hasher + Default,
    Value: AsRef<[u8]>,
{
    type Hash = u64;
    type Seed = u64;

    #[inline(always)]
    fn hash_with_seed(value: &Value, seed: Self::Seed) -> Self::Hash {
        let mut s = Hasher::default();
        s.write(value.as_ref());
        s.write_u64(seed);
        s.finish()
    }

    #[inline(always)]
    fn hash(value: &Value) -> Self::Hash {
        let mut s = Hasher::default();
        s.write(value.as_ref());
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Hash;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_hash_empty() {
        let hash = DefaultHasher::hash(&"");
        assert!(hash > 0)
    }
    #[test]
    fn test_hash_with_seed_empty() {
        let hash = DefaultHasher::hash_with_seed(&"", 1);
        assert!(hash > 0)
    }
}
