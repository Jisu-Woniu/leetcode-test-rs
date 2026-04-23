#![allow(unused_macros)]

macro_rules! test_eq {
    ($(#[$group_meta: meta])* $group_name:ident : $($(#[$fn_meta: meta])* $fn_name:ident => $actual:expr, $expect:expr);+ $(;)?) => {
        #[cfg(test)]
        $(#[$group_meta])*
        mod $group_name {
            use super::*;

            $(
                #[test]
                $(#[$fn_meta])*
                fn $fn_name() {
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
    ($([$($inner:tt)*]),+ $(,)?) => {
        vec![$(vec_nested![$($inner)*]),+]
    };
    ([$($inner:tt)*]; $n:expr) => {
        vec![vec_nested![$($inner)*]; $n]
    };
    // Base case: forward to `vec!` call.
    ($($elem:tt)*) => {
        vec![$($elem)*]
    };
}

#[cfg(test)]
mod tests {
    use std::{convert::Infallible, fmt::Debug};

    /// Helper function for adding type annotations.
    fn assert_equal<T: PartialEq + Debug>(a: T, b: T) {
        assert_eq!(a, b);
    }

    #[test]
    fn test_vec_nested() {
        assert_equal::<Vec<Infallible>>(vec![], vec_nested![]);
        assert_eq!(vec![1, 2], vec_nested![1, 2]);
        assert_eq!(vec![1, 2], vec_nested![1, 2,]);
        assert_eq!(vec![vec![1, 2]], vec_nested![[1, 2]]);
        assert_eq!(vec![vec![1, 2], vec![3, 4]], vec_nested![[1, 2], [3, 4]]);
        assert_eq!(vec![vec![1, 2], vec![3, 4]], vec_nested![[1, 2], [3, 4],]);
        assert_eq!(vec![vec![vec![1]]], vec_nested![[[1]]]);
        assert_eq!(vec![vec![1]; 5], vec_nested![[1]; 5]);
    }
}
