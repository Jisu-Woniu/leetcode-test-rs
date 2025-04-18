mod list_utils;
mod tree_utils;

use crate::Solution;

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

// Multiple cases in one invocation.
test_eq! {
    tests:
    test1 => Solution::dummy_fn(1), 1;
    test2 => Solution::dummy_fn(2), 2;
}

// Test with custom attributes
test_eq!(
    #[cfg(any())]
    failing_tests:
    #[ignore = "This test will fail"]
    failing_test => Solution::dummy_fn(1), -1;
);
