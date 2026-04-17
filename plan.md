1. **Create Design Doc**:
   - Use `run_in_bash_session` with `cat << 'EOF' > docs/architecture/kairos_sub_agent_queue_design.md` to create a new premium design doc for the Sub-Agent Orchestration Queue, complete with problem statement, architecture, UI visual excellence mandates, and exact DB schema.
2. **Verify Design Doc**:
   - Use `cat docs/architecture/kairos_sub_agent_queue_design.md` to ensure the file was correctly written and structured.
3. **Verify Documentation Build**:
   - Run `./bazelisk run //:docs_build` to ensure the documentation builds successfully without errors.
4. **Create GitHub Issue**:
   - Use `run_in_bash_session` to execute a `curl` command to the GitHub API (`https://api.github.com/repos/onehumancorp/mono/issues`) creating a highly detailed issue for implementers, prefixed with `[backend]`. It will include 'Parent: #5049', the Problem Statement, Research Report, Design Doc, Implementation Prompt, Priority, and Estimated Scope.
5. **Complete pre-commit steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
6. **Submit Changes**:
   - Run `git checkout -b jules/kairos-subagent-queue-design`
   - Run `git add docs/architecture/kairos_sub_agent_queue_design.md`
   - Run `git commit -m "Architect Sub-Agent Orchestration Queue for KAIROS Phase 4"`
   - Run `git config --global url."https://x-access-token:$GITHUB_TOKEN@github.com/".insteadOf "https://github.com/"`
   - Run `git push origin HEAD:jules/kairos-subagent-queue-design`
   - Use `curl` to create a Pull Request via the GitHub API detailing the architectural addition.
