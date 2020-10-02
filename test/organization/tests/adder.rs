fn private_add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::private_add;
    #[test]
    fn test_private() {
        assert_eq!(private_add(5, 5), 10);
    }
}
