1. Add schema additions to SQLite migration file for Standalone Mode.
   - Create `srcs/server/db/migrations/20260417000000_agent_missions_sync.sql`
   - Use `ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT FALSE;`
   - Use `ALTER TABLE agent_missions ADD COLUMN cloud_mission_id TEXT;`
   - Use `ALTER TABLE agent_missions ADD COLUMN sync_error TEXT;`
   - Use `ALTER TABLE agent_missions ADD COLUMN last_synced_at TIMESTAMP;`
2. Implement `CloudSynchronizer` and `LocalRepository` interfaces in `srcs/server/orchestration/sync/`.
   - Create `srcs/server/orchestration/sync/types.go` for `MissionPayload` and `LocalMission` structs.
   - Create `srcs/server/orchestration/sync/interfaces.go` for `CloudSynchronizer` and `LocalRepository`.
   - Implement `localRepository` using `s.db` in `srcs/server/orchestration/sync/repository.go`.
   - Implement `cloudSynchronizer` using HTTP client in `srcs/server/orchestration/sync/synchronizer.go`.
3. Add cloud REST endpoints to `TenantRegistry` router (`srcs/server/dashboard/tenant.go`).
   - Add `POST /api/v1/missions/escalate` and `GET /api/v1/missions/{cloud_id}/status`.
   - Update `srcs/server/dashboard/server.go` to have handler methods `handleMissionEscalate` and `handleMissionStatus`.
4. Ensure 100% unit test coverage for the Synchronizer, Repository, and API endpoints.
   - Create `srcs/server/orchestration/sync/repository_test.go`.
   - Create `srcs/server/orchestration/sync/synchronizer_test.go`.
   - Create `srcs/server/dashboard/server_escalate_test.go`.
5. Pre-commit check to ensure proper testing and formatting.
6. Submit and link to #5867 and #5907.
