use std::fmt::Display;

pub trait Print: sealed::Sealed {
    fn print(&self);
    fn println(&self);
}

mod sealed {
    use super::Display;
    pub trait Sealed {}
    impl<T> Sealed for T where T: Display {}
}

impl<T> Print for T
where
    T: Display,
{
    fn print(&self) {
        print!("{self}");
    }

    fn println(&self) {
        println!("{self}")
    }
}
