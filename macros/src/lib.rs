use std::any::type_name;
use std::fmt::Debug;

fn test_inner<T: Debug>(init: T, flag: bool) {
    println!("type={:?}, init={:?}, flag={:?}", type_name::<T>(), init, flag);
}

fn test_clone<T: Clone>() {}

macro_rules! clone_from_copy {
    ($($t:ty),*) => {
        $(impl Clone for $t {
            fn clone(&self) -> Self { *self }
        })*
    }
}

macro_rules! test_battery {
  ($($t:ty as $name:ident),*) => {
$(
#[cfg(test)]
mod $name {
    use super::test_inner;
    #[test]
    fn frobnified() { test_inner::<$t>(1, true) }
    #[test]
    fn unfrobnified() { test_inner::<$t>(1, false) }
} )*
} }

macro_rules! test_trait {
  ($($t:ty as $name:ident),*) => {
$(
#[cfg(test)]
mod $name {
    use super::*;
    #[test]
    fn implements_clone() { test_clone::<$t>() }
} )*
} }

test_battery! (
    u8 as u8_test,
    u16 as u16_test,
    u32 as u32_test,
    u64 as u64_test,
    u128 as u128_test
);

#[derive(Copy)]
struct Integer(i32);

#[derive(Copy)]
struct Float(f32);

#[derive(Copy)]
struct Flag(bool);

clone_from_copy![Integer, Float, Flag];

test_trait! (
    Integer as integer_clone,
    Float as float_clone,
    Flag as flag_clone
);

macro_rules! dict {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $val);
        )*
        map
    }}
}

#[cfg(test)]
mod test_dict {
    use std::collections::HashMap;

    #[test]
    fn test_dict_literal() {
        let hashmap = dict![
            "Alice" => 1,
            "Bob" => 2,
            "Chris" => 3
        ];

        let mut result: Vec<(&str, i32)> = hashmap.into_iter().collect();
        result.sort();

        assert_eq!(result, vec![("Alice", 1), ("Bob", 2), ("Chris", 3)]);
    }
}