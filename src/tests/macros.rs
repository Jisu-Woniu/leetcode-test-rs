#![allow(unused_macros)]

macro_rules! test_eq {
    ($(#[$group_meta: meta])* $group_name:ident : $($(#[$fn_meta: meta])* $i:ident => $actual:expr, $expect:expr);+ $(;)?) => {
        #[cfg(test)]
        $(#[$group_meta])*
        mod $group_name {
            use super::*;

            $(
                #[test]
                $(#[$fn_meta])*
                fn $i() {
                    assert_eq!($actual, $expect);
                }
            )+
        }
    };
}

macro_rules! vec_string {
    () => (
        Vec::<String>::new()
    );
    ($s:literal; $n:expr) => (
        vec![String::from($s); $n]
    );
    ($($s:literal),* $(,)?) => {
        vec![$(String::from($s)),*]
    };
}

macro_rules! vec_option {
    // `@item`: process next item (`null` or an `expr`) and append it to the array.
    // Always process `null` before other `expr`s, prevent `use std::ptr::null;` interference.
    // Consume exactly one comma after the item.
    (@item [$($items:expr),*] null $(, $($rem:tt)*)?) => {
        vec_option!(@item [$($items,)* None] $($($rem)*)?)
    };
    (@item [$($items:expr),*] $e:expr $(, $($rem:tt)*)?) => {
        vec_option!(@item [$($items,)* Some($e)] $($($rem)*)?)
    };
    // No more items, call `vec!` on the result directly.
    (@item [$($items:expr),*]) => {
        vec![$($items),*]
    };
    // Repeat expression handling.
    (null; $n:expr) => {
        vec![None; $n]
    };
    ($e:expr; $n:expr) => {
        vec![Some($e); $n]
    };
    // Forward to `@item` call.
    ($($tt:tt)*) => {
        vec_option!(@item [] $($tt)*)
    }
}

macro_rules! vec_nested {
    // Empty case with type annotation
    () => {
        vec![]
    };
    // Recursive case: handle multiple nested vectors
    ($([$($inner:tt)*]),* $(,)?) => {
        vec![$(vec_nested![$($inner)*],)*]
    };
    // Base case: handle a single vector of elements
    ($($elem:tt)*) => {
        vec![$($elem)*]
    };
}
