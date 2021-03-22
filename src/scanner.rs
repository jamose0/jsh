pub struct Scanner<'a> {
    source_str: &'a str,
    current_pos: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source_str: &'a str) -> Scanner {
        Scanner {
            source_str,
            current_pos: 0,
        }
    }
}
