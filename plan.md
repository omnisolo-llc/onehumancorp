1. **Generate Mission File: Phase 1 (Shared Task List)**
   - Use `cat << 'EOF' > .agent-task/missions/$(date -u +"%Y-%m-%dT%H-%M-%SZ")_phase1_shared_task_list.md` to create the mission file. The content must include: Title, Problem Statement, Research Report, Design Doc (with database schema and sequence diagrams), Implementation Prompt, Priority, and Estimated Scope.

2. **Generate Mission File: Phase 2 (Teammate Mesh APIs)**
   - Use `cat << 'EOF' > .agent-task/missions/$(date -u +"%Y-%m-%dT%H-%M-%SZ")_phase2_mesh_apis.md` to create the mission file containing the required sections and designing the Realtime Teammate Mesh APIs.

3. **Generate Mission File: Phase 3 (AutoDream Pipelines)**
   - Use `cat << 'EOF' > .agent-task/missions/$(date -u +"%Y-%m-%dT%H-%M-%SZ")_phase3_autodream_pipelines.md` to create the mission file detailing the AutoDream data pipelines for OHC's long-term memory consolidation.

4. **Generate Mission File: Phase 4 (Master Design Doc)**
   - Use `cat << 'EOF' > .agent-task/missions/$(date -u +"%Y-%m-%dT%H-%M-%SZ")_phase4_master_design_doc.md` to create the premium Master Design Doc via PR detailing how OHC will implement these AI OS features.

5. **Verify Mission Files**
   - Use `ls -la .agent-task/missions/` to verify the files were created.
   - Use `cat $(ls -t .agent-task/missions/*.md | head -n 4)` to ensure their contents match expectations.

6. **Teammate Mesh Coordination via Memory Protocol**
   - Use `echo "message: KAIROS orchestration completed. Phases 1-4 mission files have been generated and dispatched to the swarm." > .agent-task/memory/mesh_mock.log && git add -f .agent-task/memory/mesh_mock.log` to record the coordination message.

7. **Record Observability Heartbeat**
   - Use `echo -e "status: healthy\ncomponent: kairos_orchestrator\nmessage: Master orchestration planning complete." > .agent-task/status/$(date -u +"%Y-%m-%dT%H-%M-%SZ").yml` to record system health metrics.

8. **Verify Coordination and Heartbeat Files**
   - Use `cat .agent-task/memory/mesh_mock.log` to verify the mock coordination message.
   - Use `cat $(ls -t .agent-task/status/*.yml | head -n 1)` to verify the status file creation.

9. **Run Tests**
   - Use `export GOPROXY=direct && bazelisk test //srcs/server/... && bazelisk test //srcs/app/...` to ensure no regressions were introduced while avoiding timeouts.

10. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

11. **Submit**
    - Use `submit` to submit the PR with branch name `kairos-orchestrator-planning`.
