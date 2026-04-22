mod list_utils;
#[macro_use]
mod macros;
#[macro_use]
mod tree_utils;

use crate::Solution;

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
