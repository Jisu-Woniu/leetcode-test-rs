mod list_utils;
mod tree_utils;

use crate::{
    // solution::{next_mask, MaskIter},
    Solution,
};

macro_rules! test_eq {
    ($($(#[$m: meta])* $i:ident, $actual:expr, $expect:expr);+ $(;)?) => {
        $(
            #[test]
            $(#[$m])*
            fn $i() {
                assert_eq!($actual, $expect);
            }
        )+
    };
}

// Multiple cases in one invocation.
test_eq! {
    test1, Solution::dummy_fn(1), 1;
    test2, Solution::dummy_fn(2), 2;
}

// Test with custom attributes
test_eq!(
    #[ignore = "This test will fail"]
    failing_test,
    Solution::dummy_fn(3),
    4
);
