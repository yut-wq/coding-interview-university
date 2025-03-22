//! linked listの実装、テスト。

#[allow(dead_code)]
/// Linked Listをenumを用いて実装
enum MyLinkedList {
    /// ノード
    /// 値と次のノードへのポインターを持つ。
    Node(i32, Box<MyLinkedList>),
    /// 終端
    Nill,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList::Nill
    }

    fn size(&self) -> usize {
        let mut size = 0;
        let mut node = self;
        while let MyLinkedList::Node(_, next_node) = node {
            size += 1;
            node = next_node;
        }

        size
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// インデックスを指定してアイテムを取得する。
    fn value_at(&self, index: usize) -> i32 {
        let mut now_index = 0;
        let mut node = self;
        while let MyLinkedList::Node(item, next_node) = node {
            if now_index == index {
                return *item;
            }
            now_index += 1;
            node = next_node;
        }
        panic!("invalid index");
    }

    fn push_front(self, item: i32) -> Self {
        MyLinkedList::Node(item, Box::new(self))
    }

    fn pop_front(&mut self) -> i32 {
        match &self {
            MyLinkedList::Node(item, my_lined_list) => {
                self = *my_lined_list;
                *item
            }
            MyLinkedList::Nill => panic!("no item."),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn size() {
        let list = MyLinkedList::new();
        assert_eq!(list.size(), 0);

        let list = MyLinkedList::Node(3, Box::new(MyLinkedList::Nill));
        assert_eq!(list.size(), 1);
    }

    #[test]
    fn is_empty() {
        let list = MyLinkedList::Node(
            0,
            Box::new(MyLinkedList::Node(
                1,
                Box::new(MyLinkedList::Node(2, Box::new(MyLinkedList::Nill))),
            )),
        );
        assert!(!list.is_empty());

        let list = MyLinkedList::Nill;
        assert!(list.is_empty());
    }

    #[test]
    fn value_at() {
        // indexの値を返す
        let list = MyLinkedList::Node(
            0,
            Box::new(MyLinkedList::Node(
                1,
                Box::new(MyLinkedList::Node(2, Box::new(MyLinkedList::Nill))),
            )),
        );

        assert_eq!(list.value_at(0), 0);
        assert_eq!(list.value_at(1), 1);
        assert_eq!(list.value_at(2), 2);
    }

    #[test]
    fn push_front() {
        let list = MyLinkedList::Node(
            0,
            Box::new(MyLinkedList::Node(
                1,
                Box::new(MyLinkedList::Node(2, Box::new(MyLinkedList::Nill))),
            )),
        );
        let new_list = list.push_front(999);

        assert_eq!(new_list.value_at(0), 999);
        assert_eq!(new_list.value_at(1), 0);
        assert_eq!(new_list.value_at(2), 1);
        assert_eq!(new_list.value_at(3), 2);
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

        // assert_eq!(list.value_at(0), 0);
        // assert_eq!(list.value_at(1), 1);
        // assert_eq!(list.value_at(2), 2);
        // assert_eq!(list.value_at(3), 999);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedList::from([0, 1, 2]);
        assert_eq!(list.pop_back().unwrap(), 2);
    }

    #[test]
    fn front() {
        let list = LinkedList::from([0, 1, 2]);
        assert_eq!(list.front().unwrap(), &0);
    }

    #[test]
    fn back() {
        let list = LinkedList::from([0, 1, 2]);
        assert_eq!(list.back().unwrap(), &2);
    }
}
