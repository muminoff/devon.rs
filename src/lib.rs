pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::grammar::Word;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_word() {
        let word = Word::new("test");
        assert_eq!(word.text, "test");
    }
}
