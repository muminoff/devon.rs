pub struct Word {
    pub text: &'static str,
    pub vowels: Vec<char>,
    pub consonants: Vec<char>,
}

impl Word {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            vowels: Vec::new(),
            consonants: Vec::new(),
        }
    }
}
