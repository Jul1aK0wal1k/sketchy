pub trait Hash {
    fn hash<Value>(value : Value);
    fn hash_with_seed<Value, Seed>(value: Value, seed: Seed);
}
