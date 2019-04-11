use std::str;
use syn::Ident;
use proc_macro::Span;

pub(crate) fn to_snake_case<S: AsRef<str>>(ident: &S) -> String {
    let mut snake_case = String::new();

    for (i, c) in ident.as_ref().chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }

        snake_case.push(c.to_lowercase().next().unwrap());
    }

    snake_case
}

pub(crate) struct UniqueIdentifierIterator {
    buffer: Vec<u8>,
}

impl UniqueIdentifierIterator {
    pub(crate) fn new() -> Self {
        UniqueIdentifierIterator {
            buffer: vec![b'a'],
        }
    }
}

impl Iterator for UniqueIdentifierIterator {
    type Item = Ident;

    /// Generates infinite length strings from ASCII chars a-z
    fn next(&mut self) -> Option<Self::Item> {
        let ident = Ident::new(str::from_utf8(&self.buffer).unwrap(), Span::call_site().into());
        let last_char = self.buffer.len() - 1;

        if self.buffer[last_char] < b'z' {
            self.buffer[last_char] += 1;
        } else {
            self.buffer.push(b'a');
        }

        Some(ident)
    }
}
