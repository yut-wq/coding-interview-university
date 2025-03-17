#[cfg(test)]
mod tests {
    #[test]
    fn get_size_success() {
        let vec = vec![1, 2, 4, 5];
        assert_eq!(vec.len(), 4);
    }
}
