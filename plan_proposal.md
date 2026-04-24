1. **Implement `IsolationStrategy` interface**: Use a file editing tool (like `replace_with_git_merge_diff`) to add `IsolationStrategy` interface to `src/server/agents/provider.go`. The interface will have a single method `RunInIsolation(worktree string) error`.
2. **Update `Provider` interface**: Use a file editing tool to embed the `IsolationStrategy` interface in the `Provider` interface in `src/server/agents/provider.go`.
3. **Implement `RunInIsolation`**: Use a file editing tool to add `RunInIsolation(worktree string) error` to all the structs implementing `Provider` (or just to `baseProvider` and `BuiltinProvider`) in `src/server/agents/provider.go`. The implementation will pipe output streams directly to Redis Pub/Sub, for example: `fmt.Printf("Redis Pub/Sub: agent %s running in worktree %s\n", p.Type(), worktree)` and return `nil`.
4. **Verify code changes**: Run `cat src/server/agents/provider.go` to ensure all edits were applied correctly.
5. **Run tests**: Execute `bazelisk test //src/server/agents/...` to ensure all tests pass.
6. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
7. **Submit the code**: Use the `submit` tool to commit the code, and then output a final message with a YAML block containing `issue_id: 4871`.
