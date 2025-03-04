# `leetcode-test` Crate Template

> General purpose code test template for LeetCode solutions in Rust

This solution is adapted from the binary project template of Cargo. To use it:

1. Create a _hard link_ `solution.rs` to the file where `impl Solution` is.

   On UNIX-like systems:

   ```sh
   ln -f /path/to/your/leetcode-solution.rs src/solution.rs
   ```

   On Windows:

   ```powershell
   New-Item -ItemType HardLink -Path src/solution.rs -Value /path/to/your/leetcode-solution.rs
   ```

   If a hard link doesn't work, you can just copy your solution file and replace the original one.

2. Add a `use crate::Solution;` declaration to your code file, before submit region if you use a plugin.

   If you use [LeetCode for VS Code](https://marketplace.visualstudio.com/items?itemName=LeetCode.vscode-leetcode), you can add it like:

   ```rust
   use crate::Solution;
   // @lc code=start
   ```

   If you use [LeetCode Editor for JetBrains IDE](https://plugins.jetbrains.com/plugin/12132-leetcode-editor), you can add it like:

   ```rust
   use crate::Solution;
   //leetcode submit region begin(Prohibit modification and deletion)
   ```

3. Copy or write the test cases in `src/tests/mod.rs`. A `LinkedList` and a `TreeNode`, with their respective deserialize functions, are provided for convenience.
4. Run or debug your tests as you like.
