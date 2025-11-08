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
        let array = [
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];
        let mut queue = QueueFromArray {
            inner: array,
            head_pointer: 0,
            tail_pointer: 3,
        };

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
}
