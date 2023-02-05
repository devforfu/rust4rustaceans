use std::any::type_name;
use std::fmt::Debug;

fn test_inner<T: Debug>(init: T, flag: bool) {
    println!("type={:?}, init={:?}, flag={:?}", type_name::<T>(), init, flag);
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

test_battery! (
    u8 as u8_test,
    u16 as u16_test,
    u32 as u32_test,
    u64 as u64_test,
    u128 as u128_test
);