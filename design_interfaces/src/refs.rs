use std::collections::hash_map::HashMap;
use std::ops::Deref;

pub type Name = String;
pub type PhoneNumber = usize;

pub struct PhoneBook(HashMap<Name, PhoneNumber>);

impl PhoneBook {
    pub fn add(&mut self, name: Name, number: PhoneNumber) {
        self.0.insert(name, number);
    }
}

impl Default for PhoneBook {
    fn default() -> Self {
        Self { 0: HashMap::new() }
    }
}

impl Deref for PhoneBook {
    type Target = HashMap<Name, PhoneNumber>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_book() {
        let mut book = PhoneBook::default();
        book.add(String::from("Alice"), 123456789);
        assert_eq!(book.len(), 1);
    }
}
