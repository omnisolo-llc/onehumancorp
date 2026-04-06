import re

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

upsert_code = """// UpsertMission inserts or updates a mission in the agent_missions table.
func (s *SIPDB) UpsertMission(ctx context.Context, missionID, status, payload string, forceLocal bool) error {
	return withSipRetry(ctx, func() error {
		tx, err := s.db.Begin(ctx)
		if err != nil {
			return err
		}
		defer tx.Rollback(ctx)

		if s.db.IsSQLite() {
			// SQLite simple UPSERT
			upsertQuery := `
				INSERT INTO agent_missions (id, status, payload, created_at, organization_id)
				VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4)
				ON CONFLICT(id) DO NOTHING
			`
			if forceLocal {
				upsertQuery = `
					INSERT INTO agent_missions (id, status, payload, created_at, organization_id)
					VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4)
					ON CONFLICT(id) DO UPDATE SET
						status=EXCLUDED.status,
						payload=EXCLUDED.payload
				`
			}
			_, err = tx.Exec(ctx, upsertQuery, missionID, status, payload, s.orgID)
			if err != nil {
				return err
			}
		} else {
			// Postgres with FOR UPDATE SKIP LOCKED
			// Try to select existing row for update
			var existingID string
			err := tx.QueryRow(ctx, "SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2 FOR UPDATE SKIP LOCKED", missionID, s.orgID).Scan(&existingID)

			if err != nil {
				// Record doesn't exist or is locked by someone else.
				// Since we skip locked, if it's locked we might just skip the insert/update to avoid contention,
				// or we try inserting. For standard upsert logic without waiting:
				insertQuery := `
					INSERT INTO agent_missions (id, status, payload, created_at, organization_id)
					VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4)
					ON CONFLICT(id) DO NOTHING
				`
				_, errInsert := tx.Exec(ctx, insertQuery, missionID, status, payload, s.orgID)
				if errInsert != nil {
					return errInsert
				}
				// If forceLocal, we still want to update it if it was inserted successfully but not updated.
				// However ON CONFLICT DO NOTHING will skip if it exists and wasn't locked.
				// Actually, a simpler standard approach for Postgres with FOR UPDATE SKIP LOCKED is:
			}

			if forceLocal && existingID != "" {
				updateQuery := `
					UPDATE agent_missions
					SET status = $1, payload = $2
					WHERE id = $3 AND organization_id = $4
				`
				_, errUpdate := tx.Exec(ctx, updateQuery, status, payload, missionID, s.orgID)
				if errUpdate != nil {
					return errUpdate
				}
			} else if existingID == "" {
				// We tried to select and it failed (either not found or locked).
				// We will try an insert ON CONFLICT.
				insertQuery := `
					INSERT INTO agent_missions (id, status, payload, created_at, organization_id)
					VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4)
					ON CONFLICT(id) DO NOTHING
				`
				if forceLocal {
					insertQuery = `
						INSERT INTO agent_missions (id, status, payload, created_at, organization_id)
						VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4)
						ON CONFLICT(id) DO UPDATE SET
							status=EXCLUDED.status,
							payload=EXCLUDED.payload
					`
				}
				_, errInsert := tx.Exec(ctx, insertQuery, missionID, status, payload, s.orgID)
				if errInsert != nil {
					return errInsert
				}
			}
		}

		return tx.Commit(ctx)
	})
}"""

old_pattern = re.compile(r"// UpsertMission inserts or updates a mission in the agent_missions table\.\nfunc \(s \*SIPDB\) UpsertMission\(ctx context\.Context, missionID, status, payload string, forceLocal bool\) error \{.*?\n\s+return err\n\t\}\)\n\}", re.DOTALL)

if not old_pattern.search(content):
    print("Could not find pattern to replace")
else:
    new_content = old_pattern.sub(upsert_code, content)
    with open("srcs/server/orchestration/sip.go", "w") as f:
        f.write(new_content)
    print("Replaced UpsertMission")
