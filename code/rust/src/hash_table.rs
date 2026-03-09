//! ハッシュテーブルの実装

use std::hash::{DefaultHasher, Hasher};

/// キーからハッシュを計算する。
#[allow(dead_code)]
fn hash(key: &str, table_size: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(key.as_bytes());
    let hash = hasher.finish();
    hash % table_size
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ハッシュ値がテーブルサイズより小さいこと() {
        let table_size = 3;
        let value = hash("test", table_size);

        assert!(value < table_size);
    }
}
