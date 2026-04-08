1. **Create Mission Brief**
   - Create a mission file `.agent-task/missions/$(date -u +'%Y-%m-%dT%H-%M-%SZ').md` for a "Hybrid Secrets Management MCP Server".
   - **Title**: Integrate Hybrid Secrets Management MCP Server
   - **Problem Statement**: Agents need to securely access API keys and credentials across Multi-tenant Cloud and Single-user Standalone environments without leaking them into plaintext context or local unencrypted storage.
   - **Research Report**: Cloud mode requires tenant-scoped Vault/K8s secret access; Standalone requires encrypted local storage.
   - **Design Doc**: MCP server exposing `get_secret`, `put_secret`. Provider interface `mcp.SecretsProvider`.
   - **Implementation Prompt**: Detailed prompt for Implementer.
   - **Priority**: P1
   - **Estimated Scope**: Medium

2. **Verify Mission File**
   - Use `run_in_bash_session` with `ls` to confirm the directory path and `head`, `tail`, or `grep` to verify the contents of the newly created mission file.

3. **Verify No Code Breakages**
   - Run the command `bazelisk test //srcs/server/... --test_output=errors` to ensure the new mission file did not break anything.

4. **Complete Pre-commit Steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

5. **Submit PR**
   - Submit the PR with the new mission file.
