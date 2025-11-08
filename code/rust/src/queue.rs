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
}
