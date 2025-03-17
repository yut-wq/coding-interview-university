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

    #[test]
    fn at() {
        // indexを指定して要素を取得する。
        let vec = vec![1, 2, 4, 5];
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 4);
        assert_eq!(vec[3], 5);

        // 範囲外アクセスはパニックする。
        // assert_eq!(vec[4], 5);
    }

    #[test]
    fn push() {
        let mut vec = vec![];
        vec.push(1);
        assert_eq!(vec[0], 1);
    }

    #[test]
    fn insert() {
        let mut vec = vec![1, 2, 3];
        vec.insert(1, 99);

        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 99);
        assert_eq!(vec[2], 2);
        assert_eq!(vec[3], 3);
    }
}
