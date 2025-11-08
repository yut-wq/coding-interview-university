//! 固定長配列ベースのキューの実装

#[allow(dead_code)]
struct QueueFromArray {
    inner: [Option<i32>; 32],
    head_pointer: usize,
    tail_pointer: usize,
}

#[allow(dead_code)]
impl QueueFromArray {
    #[allow(dead_code)]
    fn new() -> Self {
        let inner = [None; 32];
        Self {
            inner,
            head_pointer: 0,
            tail_pointer: 0,
        }
    }

    /// 末尾に要素を追加する。
    #[allow(dead_code)]
    fn enqueue(&mut self, element: i32) {
        self.inner[self.tail_pointer] = Some(element);
        self.tail_pointer += 1;
    }

    /// 先頭の要素を削除し、取得する
    #[allow(dead_code)]
    fn dequeue(&mut self) -> Option<i32> {
        let result = self.inner[self.head_pointer];

        // 値の削除、head_pointerの更新
        self.inner[self.head_pointer] = None;
        self.head_pointer += 1;

        result
    }

    /// コレクションが空か判定する
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.inner.iter().filter(|value| value.is_some()).count() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueueで要素1を追加する() {
        let mut queue = QueueFromArray::new();

        queue.enqueue(1);

        let inner = queue.inner;
        assert_eq!(inner[0], Some(1));
        assert_eq!(inner[1], None);

        assert_eq!(queue.head_pointer, 0);
        assert_eq!(queue.tail_pointer, 1);
    }

    #[test]
    fn dequeueで先頭の要素を削除し取得する() {
        let mut queue = QueueFromArray::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        let result = queue.dequeue();
        assert_eq!(result, Some(1));

        let inner = queue.inner;
        assert_eq!(inner[0], None);
        assert_eq!(inner[1], Some(2));
        assert_eq!(inner[2], Some(3));
        assert_eq!(inner[3], None);

        assert_eq!(queue.head_pointer, 1);
        assert_eq!(queue.tail_pointer, 3);
    }

    #[test]
    fn 要素が空であればemptyでtrueを返す() {
        let queue = QueueFromArray::new();

        assert!(queue.empty());
    }

    #[test]
    fn 要素があればemptyでfalseを返す() {
        let mut queue = QueueFromArray::new();
        queue.enqueue(1);
        queue.enqueue(2);

        assert!(!queue.empty());
    }
}
