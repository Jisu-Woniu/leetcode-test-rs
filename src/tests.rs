mod list_utils;
#[macro_use]
mod macros;
#[macro_use]
mod tree_utils;

use crate::Solution;

impl Solution {
    /// Dummy function that always return `input` as is.
    #[allow(dead_code)]
    fn dummy_fn<T>(input: T) -> T {
        input
    }
}

test_eq! {
    /// Multiple cases in one invocation.
    tests:
    test1 => Solution::dummy_fn(1), 1;
    test2 => Solution::dummy_fn(2), 2;
}

test_eq!(
    /// Test with custom attributes
    failing_tests:
    #[ignore = "This test will fail"]
    failing_test => Solution::dummy_fn(1), -1;
);
