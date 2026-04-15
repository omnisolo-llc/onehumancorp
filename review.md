1. Initialize task directories and assign file paths using variables:
   - `export MISSIONS_DIR=".agent-task/missions"`
   - `export STATUS_DIR=".agent-task/status"`
   - `export MEMORY_DIR=".agent-task/memory"`
   - `export CURRENT_TIMESTAMP=$(date -u +"%Y-%m-%dT%H-%M-%SZ")`
   - `export MISSION_FILE="${MISSIONS_DIR}/${CURRENT_TIMESTAMP}.md"`
   - `export STATUS_FILE="${STATUS_DIR}/${CURRENT_TIMESTAMP}.yml"`
   - `export MEMORY_FILE="${MEMORY_DIR}/${CURRENT_TIMESTAMP}.yml"`
   - `mkdir -p $MISSIONS_DIR $STATUS_DIR $MEMORY_DIR`

2. Create the mission file for the Implementer agent outlining how to add `BufferMetricFunc` instrumentation to `telemetry_bridge.go`, `minimax_metrics.go`, and `rag_sync_metrics.go` ensuring proper PII redaction:
   - Command: `cat << 'EOF' > $MISSION_FILE` followed by the markdown payload with specific `<div>` styling, implementation prompt referencing paths, and functions explicitly checked in the trace (`RecordBridgeMessageSent`, `RecordBridgeMessageReceived`, `RecordBridgeStatus` in `telemetry_bridge.go`; `RecordMinimaxCall` in `minimax_metrics.go`; missing `RecordRAGRecordsSynced` and `RecordRAGSyncError` in `rag_sync_metrics.go`).

3. Create the status file to record the observability heartbeat:
   - Command: `cat << 'EOF' > $STATUS_FILE` followed by basic status info (e.g., `status: DONE`).

4. Create the memory file to update the swarm memory with the findings:
   - Command: `cat << 'EOF' > $MEMORY_FILE` followed by a memory snippet about the missing BufferMetricFuncs.

5. Verify the generated files:
   - `cat $MISSION_FILE`
   - `cat $STATUS_FILE`
   - `cat $MEMORY_FILE`

6. Run the test skip command since no code was directly modified:
   - `export PATH=$PATH:$HOME/go/bin && echo "No code modifications; tests skipped"`

7. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. Submit the change:
   - Branch: `researcher-telemetry-fix`
   - Title: `✨ Researcher: Implement missing telemetry buffering`
   - Description: `Creates mission for Implementer to add BufferMetricFunc support.`
   - Commit message: "Added mission for telemetry buffering implementation"
