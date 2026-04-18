1. **Fix `ProcessMemories` memory directory default:**
   - I will use `run_in_bash_session` to apply a sed replacement in `srcs/server/orchestration/autodream_worker.go` to default `memoryDir` to `.agent-task/memory` if `OHC_MEMORY_DIR` is empty. The problem statement explicitly requires data from `.agent-task/memory/` to be processed.
   - Command:
     ```bash
     sed -i 's/if memoryDir == "" {/if memoryDir == "" {\n\t\tmemoryDir = ".agent-task\/memory"\n\t}\n\tif memoryDir == "" {/' srcs/server/orchestration/autodream_worker.go
     ```
   - Actually, I will just use `replace_with_git_merge_diff` for precision.
2. **Verify modifications:**
   - I will use `run_in_bash_session` to `cat srcs/server/orchestration/autodream_worker.go` and verify the exact changes were applied.
3. **Check Code Coverage:**
   - I will run `bazelisk coverage //srcs/server/orchestration:orchestration_test --test_filter="TestAutoDreamWorker"` and then `cat bazel-out/_coverage/_coverage_report.dat | grep autodream_worker.go` to ensure >90% coverage.
4. **Complete pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done. I will use `pre_commit_instructions` tool to execute all necessary checks.
5. **Submit:**
   - Run `submit` with the branch name and a descriptive commit message. I will include the text `issue_id: 4339` in the commit body and my final message.
