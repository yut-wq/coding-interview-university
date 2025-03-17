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
}
