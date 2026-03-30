Ah, the reviewer said "test coverage should ideally be added for the new `AuditMissions` function to catch the aforementioned SQL error before runtime."
Let me add a test `TestSIPDB_AuditMissions` in `srcs/orchestration/sip_test.go` that verifies the `updated_at` column works and `AuditMissions` doesn't crash.
