package orchestration

import (
	"context"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDreamListener struct {
	db db.Provider
}

func NewAutoDreamListener(provider db.Provider) *AutoDreamListener {
	return &AutoDreamListener{db: provider}
}

func (a *AutoDreamListener) BatchCompletedTasks(ctx context.Context) error {
	tx, err := a.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var query string
	if os.Getenv("OHC_STANDALONE") == "true" {
		query = "SELECT id, organization_id, payload FROM shared_tasks_v2 WHERE status = 'DONE' LIMIT 50"
	} else {
		query = "SELECT id, organization_id, payload FROM shared_tasks_v2 WHERE status = 'DONE' LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return err
	}
	defer rows.Close()

	type taskRecord struct {
		id      string
		orgID   string
		payload string
	}
	var records []taskRecord

	for rows.Next() {
		var r taskRecord
		if err := rows.Scan(&r.id, &r.orgID, &r.payload); err != nil {
			return err
		}
		records = append(records, r)
	}

	for _, r := range records {
		var embedQuery string
		if os.Getenv("OHC_STANDALONE") == "true" {
			embedQuery = "INSERT INTO ohc_memory_embeddings (id, tenant_id, memory_type, content, source_task_id) VALUES (?, ?, 'TASK_COMPLETED', ?, ?)"
			_, err = tx.Exec(ctx, embedQuery, r.id+"_mem", r.orgID, r.payload, r.id)
		} else {
			embedQuery = "INSERT INTO ohc_memory_embeddings (id, tenant_id, memory_type, content, source_task_id) VALUES ($1, $2, 'TASK_COMPLETED', $3, $4)"
			_, err = tx.Exec(ctx, embedQuery, r.id+"_mem", r.orgID, r.payload, r.id)
		}
		if err != nil {
			continue
		}

		updateQuery := "UPDATE shared_tasks_v2 SET status = 'ARCHIVED' WHERE id = $1"
		if os.Getenv("OHC_STANDALONE") == "true" {
			updateQuery = "UPDATE shared_tasks_v2 SET status = 'ARCHIVED' WHERE id = ?"
		}
		_, _ = tx.Exec(ctx, updateQuery, r.id)
	}

	return tx.Commit(ctx)
}
