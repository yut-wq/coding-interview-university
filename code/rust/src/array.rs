//! 動的可変配列の実装、テスト。
//! rustのvectorを利用する。
//! vectorのドキュメント
//! <https://doc.rust-lang.org/std/vec/struct.Vec.html>

#[cfg(test)]
mod tests {

    #[test]
    fn get_size_success() {
        let vec = vec![1, 2, 4, 5];
        assert_eq!(vec.len(), 4);
    }

    #[test]
    fn get_capacity_success() {
        // キャパシティーを指定してvectorを作成
        let mut vec = Vec::with_capacity(3);
        assert_eq!(vec.capacity(), 3);

        vec.push(1);
        vec.push(2);
        vec.push(3);

        // キャパシティーを最小単位として、vectorが拡張される
        // 詳しくは<https://doc.rust-lang.org/std/vec/struct.Vec.html#capacity-and-reallocation>
        vec.push(4);
        assert_eq!(vec.capacity(), 6);
    }

    #[test]
    fn is_empty() {
        let vec = vec![1, 2, 4, 5];
        assert!(!vec.is_empty());

        let vec: Vec<i32> = vec![];
        assert!(vec.is_empty());
    }
}
