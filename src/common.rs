pub fn alphabet_index_from_char(c: char) -> usize {
    (c as usize) - ('a' as usize)
}

pub fn char_from_alphabet_index(index: usize) -> char {
    (index as u8 + 'a' as u8) as char
}
