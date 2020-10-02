#[cfg(test)]
mod tests {
    #[test]
    fn string_inclues() {
        let test_word = "Hello";
        let test_sentence = "Hello World";
        assert!(test_sentence.contains(&test_word));
    }
}
