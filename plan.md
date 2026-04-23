1. **Create Seccomp BPF filter file (`srcs/server/harness/seccomp.go`)**
   - Use `run_in_bash_session` to create `srcs/server/harness/seccomp.go` containing `CompileSeccompBPF(path string) error`. It will use `golang.org/x/net/bpf` to compile a filter blocking `socket(AF_UNIX, ...)` by returning `EACCES` and allowing everything else.
   - Run `cat srcs/server/harness/seccomp.go` to verify the file was created correctly.

2. **Update `SandboxConfig` in `srcs/server/harness/harness.go`**
   - Use `replace_with_git_merge_diff` to add `SeccompBPFPath string` to `SandboxConfig` struct in `srcs/server/harness/harness.go`.
   - Run `grep -n "SeccompBPFPath" srcs/server/harness/harness.go` to verify the struct was updated.

3. **Update `Run` method in `srcs/server/harness/harness.go`**
   - Use `replace_with_git_merge_diff` to modify the `Run` method inside `srcs/server/harness/harness.go`.
   - Specifically, around line 186 where it checks `if h.config.EnableSeccomp`, I will replace it to check `if h.config.EnableSeccomp && h.config.SeccompBPFPath != ""` and append the file descriptor to `execCmd.ExtraFiles`.
   - Run `cat srcs/server/harness/harness.go` and `go build ./srcs/server/harness/...` to verify the syntax and logic.

4. **Add tests**
   - Use `run_in_bash_session` to create `srcs/server/harness/seccomp_test.go` with a unit test `TestCompileSeccompBPF` that ensures the filter compiles without errors.
   - Use `replace_with_git_merge_diff` to update `srcs/server/harness/harness_test.go` to set `SeccompBPFPath` in a test and verify `--seccomp` is appended or executed.
   - Run `cat srcs/server/harness/seccomp_test.go` to verify test creation.

5. **Run all tests**
   - Use `run_in_bash_session` to run `bazelisk test //srcs/server/harness/...` to ensure all modifications work and maintain 100% test coverage.

6. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
