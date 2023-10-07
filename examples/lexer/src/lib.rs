#![feature(absolute_path)]

use std::str::Chars;

struct Cursor<'me> {
    chars: Chars<'me>,
}

impl<'me> Cursor<'me> {
    fn new(source: &'me str) -> Self {
        Self { chars: source.chars() }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn shift(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn shift_while(&mut self, f: impl Fn(char) -> bool + Copy) {
        while self.peek().is_some_and(f) {
            self.shift();
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Int,
    Unknown,
}

pub fn lex(source: &str) -> Vec<TokenKind> {
    let mut tokens = Vec::new();

    let mut cursor = Cursor::new(source);

    while let Some(first_char) = cursor.shift() {
        let token = match first_char {
            // test numbers
            // 10 100 1_000 10_000
            '0'..='9' => {
                cursor.shift_while(|ch| matches!(ch, '0'..='9' | '_'));
                TokenKind::Int
            }
            _ if first_char.is_whitespace() => {
                cursor.shift_while(|ch| ch.is_whitespace());
                continue;
            }
            _ => TokenKind::Unknown,
        };

        tokens.push(token);
    }

    tokens
}

#[cfg(test)]
#[test]
fn test() {
    let files = std::fs::read_dir("test_data").unwrap();

    for file in files {
        let file = file.unwrap().path();
        if file.extension().is_some_and(|extension| extension == "expect") {
            continue;
        }

        let source = std::fs::read_to_string(&file).unwrap();
        let actual = lex(&source)
            .into_iter()
            .map(|token| format!("{token:?}"))
            .collect::<Vec<_>>()
            .join("\n");

        let expect_file = std::path::absolute(file.with_extension("expect")).unwrap();
        expect_test::expect_file![expect_file].assert_eq(&actual);
    }
}
