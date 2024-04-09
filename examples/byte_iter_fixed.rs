struct ByteIter<'remainder> {
    remainder: &'remainder [u8],
}

impl<'remainder> Iterator for ByteIter<'remainder> {
    type Item = &'remainder u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remainder.is_empty() {
            None
        } else {
            let byte = &self.remainder[0];
            self.remainder = &self.remainder[1..];
            Some(byte)
        }
    }
}

fn main() {
    let mut bytes = ByteIter {
        remainder: b"hello world",
    };
    // bytes.for_each(|byte| println!("byte {}", byte));
    assert_eq!(Some(&b'h'), bytes.next());
    let byte1 = bytes.next();
    let byte2 = bytes.next();
    if byte1 == byte2 {}
}
