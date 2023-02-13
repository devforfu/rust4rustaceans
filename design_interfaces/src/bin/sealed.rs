use interfaces::sealed::Print;
/// An example of a "sealed" trait that cannot be implemented outside crate.
///
/// The following code fails, as the trait is already implemented.
///
/// ```compile_fail!
/// struct Bin<T> {
///     vec: Vec<T>
/// }
///
/// impl<T> Display for Bin<T> {
///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(fmt, "Bin")
///     }
/// }
///
/// impl<T> Print for Bin<T> {
///     fn print(&self) {}
///     fn println(&self) {}
/// }
/// ```
use std::fmt::{self, Display};

struct Bag<T> {
    vec: Vec<T>,
}

impl<T> Bag<T> {
    fn put(&mut self, item: T) {
        self.vec.push(item);
    }
}

impl<T> Default for Bag<T> {
    fn default() -> Self {
        Self { vec: Vec::new() }
    }
}

impl<T: Display> Display for Bag<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.vec.is_empty() {
            write!(fmt, "Empty bag")
        } else if self.vec.len() == 1 {
            write!(fmt, "Bag of one item")
        } else {
            write!(fmt, "Bag of {} items", self.vec.len())
        }
    }
}

fn main() {
    let mut bag = Bag::default();
    bag.put(1u8);
    bag.println();
    bag.put(2u8);
    let mut another = Bag::default();
    another.put(3u8);
    bag.print();
    print!(" and ");
    another.println();
}
