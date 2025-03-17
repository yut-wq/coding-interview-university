//! 動的可変配列の実装、テスト。
//! rustのvectorを利用する。
//! vectorのドキュメント
//! <https://doc.rust-lang.org/std/vec/struct.Vec.html>

#[allow(dead_code)]
/// インデックス0にアイテムを挿入する。
trait Prepend {
    #[allow(unused_variables)]
    type Item;
    #[allow(unused_variables)]
    fn prepend(&mut self, value: Self::Item) {}
}

impl<T> Prepend for Vec<T> {
    type Item = T;

    fn prepend(&mut self, value: Self::Item) {
        self.insert(0, value);
    }
}

/// インデックスにある項目を削除し、末尾の要素をすべて左にシフトする。
#[allow(dead_code)]
trait Delete {
    #[allow(unused_variables)]
    fn delete(&mut self, index: usize) {}
}

impl<T> Delete for Vec<T> {
    fn delete(&mut self, index: usize) {
        self.remove(index);
    }
}

#[allow(dead_code)]
/// 値を検索し、それを保持するインデックスを削除します (複数の場所にある場合でも)
trait MyRemove {
    #[allow(unused_variables)]
    type Item;
    #[allow(unused_variables)]
    fn my_remove(&mut self, value: Self::Item);
}

impl<T: Eq + PartialEq> MyRemove for Vec<T> {
    type Item = T;

    fn my_remove(&mut self, value: Self::Item) {
        let mut delete_index = vec![];
        for (index, item) in self.iter().enumerate() {
            if *item == value {
                // deleteした際にindexが変動するので、考慮したインデックスを保存する。
                let index = index - delete_index.len();
                delete_index.push(index);
            }
        }

        for index in delete_index {
            self.delete(index);
        }
    }
}

#[allow(dead_code)]
/// 値を検索し、その値を持つ最初のインデックスを返します。見つからない場合は -1 を返します。
trait Find {
    #[allow(unused_variables)]
    type Item;
    #[allow(unused_variables)]
    fn find(&self, value: Self::Item) -> i64;
}

impl<T: PartialEq + Eq> Find for Vec<T> {
    type Item = T;

    fn find(&self, value: Self::Item) -> i64 {
        for (index, item) in self.iter().enumerate() {
            if *item == value {
                return i64::try_from(index).unwrap();
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn prepend() {
        let mut vec = vec![1, 2, 3];
        vec.prepend(0);

        assert_eq!(vec[0], 0);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[2], 2);
        assert_eq!(vec[3], 3);
    }

    #[test]
    fn pop() {
        let mut vec = vec![1, 2, 3];

        assert_eq!(vec.pop().unwrap(), 3);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
    }

    #[test]
    fn delete() {
        // インデックスにある項目を削除し、末尾の要素をすべて左にシフトする
        let mut vec = vec![1, 2, 3];
        vec.delete(1);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 3);
    }

    #[test]
    fn my_remove() {
        // 値を検索し、それを保持するインデックスを削除します (複数の場所にある場合でも)
        let mut vec = vec![1, 2, 1];
        vec.my_remove(1);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 2);
    }

    #[test]
    fn find() {
        // 値を検索し、その値を持つ最初のインデックスを返します。見つからない場合は -1 を返します。
        let vec = vec![1, 2, 1];
        assert_eq!(vec.find(1), 0);
        assert_eq!(vec.find(2), 1);
        assert_eq!(vec.find(3), -1);
    }
}
