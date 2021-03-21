struct Scanner<'a> {
    source_str: &'a str,
}

impl<'a> Scanner<'a> {
    fn new(source_str: &'a str) -> Scanner {
        Scanner {
            source_str,
        }
    }
}
