//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)] // このcrateに対する警告を有効にする

#[derive(PartialEq, Debug)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim + self.delimiter.len()..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let mut letters = StrSplit::new(haystack, " ");
    assert_eq!(letters.next(), Some("a"));
    assert_eq!(letters.next(), Some("b"));
    assert_eq!(letters.next(), Some("c"));
    assert_eq!(letters.next(), Some("d"));
    assert_eq!(letters.next(), Some("e"));
    assert_eq!(letters.next(), None);
}

