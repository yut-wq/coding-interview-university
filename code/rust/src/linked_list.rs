//! linked listの実装、テスト。

use std::collections::LinkedList;

/// Linked Listで実装するメソッド群。
#[allow(dead_code)]
trait MyLinkedList {
    type Item;
    fn value_at(&self, index: usize) -> Self::Item;
}

impl<T: Copy> MyLinkedList for LinkedList<T> {
    type Item = T;

    fn value_at(&self, index: usize) -> Self::Item {
        for (now_index, item) in self.iter().enumerate() {
            if index == now_index {
                return *item;
            }
        }
        panic!("invalid index");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size() {
        let list = LinkedList::from([0, 1, 2]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn is_empty() {
        let list = LinkedList::from([0, 1, 2]);
        assert!(!list.is_empty());

        let list: LinkedList<i32> = LinkedList::from([]);
        assert!(list.is_empty());
    }

    #[test]
    fn value_at() {
        // indexの値を返す
        let list = LinkedList::from([0, 1, 2]);

        assert_eq!(list.value_at(0), 0);
        assert_eq!(list.value_at(1), 1);
        assert_eq!(list.value_at(2), 2);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedList::from([0, 1, 2]);
        list.push_front(999);

        assert_eq!(list.value_at(0), 999);
        assert_eq!(list.value_at(1), 0);
        assert_eq!(list.value_at(2), 1);
        assert_eq!(list.value_at(3), 2);
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedList::from([0, 1, 2]);

        assert_eq!(list.pop_front().unwrap(), 0);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedList::from([0, 1, 2]);
        list.push_back(999);

        assert_eq!(list.value_at(0), 0);
        assert_eq!(list.value_at(1), 1);
        assert_eq!(list.value_at(2), 2);
        assert_eq!(list.value_at(3), 999);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedList::from([0, 1, 2]);
        assert_eq!(list.pop_back().unwrap(), 2);
    }
}
