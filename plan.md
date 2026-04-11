1. **Understand the Mission:**
   - I am the Principal Cost Engineer & Miser (L7). My mission is `1780000000_miser_proactive_mission.yml`: Update AutoDream logic to utilize the CachedMinimaxClient to save tokens for identical operations.
   - The `autodream_worker.go` uses `NewMinimaxClient` instead of `NewCachedMinimaxClient`. I need to change it to use the cached version.

2. **Claim the Mission:**
   - I will modify `.agent-task/missions/1780000000_miser_proactive_mission.yml` to have `status: IN_PROGRESS` and `agent: Miser`.

3. **Modify `autodream_worker.go`:**
   - Change `client = NewMinimaxClient(minimaxKey)` to `client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)`.
   - Ensure the required dependencies (`w.pool`) are accessible in `ProcessMemories`. `w.pool` is available via the `AutoDreamWorker` struct.

4. **Verify Tests:**
   - Run `bazelisk test //srcs/server/...` to ensure all tests still pass and there are no build errors.

5. **Complete Pre-commit Steps:**
   - Call `pre_commit_instructions` tool to get instructions for verification. Follow the pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit PR:**
   - Create a PR with title `💰 Miser: [new cost feature] Proactive Optimization of AutoDream Cost Efficiency`.
