//! 固定長配列ベースのキューの実装

#[allow(dead_code)]
struct QueueFromArray {
    inner: Vec<i32>,
}

#[allow(dead_code)]
impl QueueFromArray {
    #[allow(dead_code)]
    fn new(capacity: usize) -> Self {
        let inner = Vec::with_capacity(capacity);
        Self { inner }
    }

    /// 末尾に要素を追加する。
    #[allow(dead_code)]
    fn enqueue(&mut self, element: i32) {
        self.inner.push(element);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueueで要素1を追加する() {
        let mut queue = QueueFromArray::new(10);

        queue.enqueue(1);

        let mut inner = queue.inner.iter();
        assert_eq!(inner.next(), Some(&1));
        assert_eq!(inner.next(), None);
    }
}
