//! キューの実装を行う。
//!
//! i32だけサポートを行う簡単なキュー

use std::collections::LinkedList;

#[allow(dead_code)]
struct QueueFromLinkedList {
    inner: LinkedList<i32>,
}

#[allow(dead_code)]
impl QueueFromLinkedList {
    #[allow(dead_code)]
    fn new() -> Self {
        let inner = LinkedList::new();
        Self { inner }
    }

    /// 末尾に要素を追加する。
    #[allow(dead_code)]
    fn enqueue(&mut self, element: i32) {
        self.inner.push_back(element);
    }

    /// 先頭の要素を削除し、取得する
    #[allow(dead_code)]
    fn dequeue(&mut self) -> Option<i32> {
        self.inner.pop_front()
    }

    /// コレクションが空か判定する
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}

// enqueueができること
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueueで要素1を追加する() {
        let mut queue = QueueFromLinkedList::new();

        queue.enqueue(1);

        let mut inner = queue.inner.iter();
        assert_eq!(inner.next(), Some(&1));
        assert_eq!(inner.next(), None);
    }

    #[test]
    fn dequeueで先頭の要素を削除し取得する() {
        let list = LinkedList::from([1, 2, 3]);
        let mut queue = QueueFromLinkedList { inner: list };

        let result = queue.dequeue();
        assert_eq!(result, Some(1));

        let mut inner = queue.inner.iter();
        assert_eq!(inner.next(), Some(&2));
        assert_eq!(inner.next(), Some(&3));
        assert_eq!(inner.next(), None);
    }

    #[test]
    fn dequeueで要素がないときにnoneを返すこと() {
        let list = LinkedList::from([]);
        let mut queue = QueueFromLinkedList { inner: list };

        let result = queue.dequeue();
        assert_eq!(result, None);

        let mut inner = queue.inner.iter();
        assert_eq!(inner.next(), None);
    }

    #[test]
    fn 要素が空であればemptyでtrueを返す() {
        let list = LinkedList::from([]);
        let queue = QueueFromLinkedList { inner: list };

        assert!(queue.empty());
    }

    #[test]
    fn 要素があればemptyでfalseを返す() {
        let list = LinkedList::from([1, 2, 3]);
        let queue = QueueFromLinkedList { inner: list };

        assert!(!queue.empty());
    }
}
