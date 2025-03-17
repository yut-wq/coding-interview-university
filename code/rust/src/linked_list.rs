//! linked listの実装、テスト。

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

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
}
