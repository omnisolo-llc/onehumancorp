Next is:
4. **Add Worker Threads to `LegacyTeammateMesh.Publish` in `mesh.go`**:
   - `LegacyTeammateMesh.Publish` writes directly to multiple subscriber channels inside a read lock.
   - Introduce a set of worker threads ("Coordinator Mode" parallelization) that pull messages from a shared `broadcast` channel and fan them out to clients asynchronously.

Let's read `mesh.go`.
