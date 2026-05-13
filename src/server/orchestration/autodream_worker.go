package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
)

type AutoDreamWorker struct {
	db       *sql.DB
	isSQLite bool
}

func NewAutoDreamWorker(db *sql.DB, isSQLite bool) *AutoDreamWorker {
	return &AutoDreamWorker{
		db:       db,
		isSQLite: isSQLite,
	}
}

func (w *AutoDreamWorker) ConsolidateEpoch(ctx context.Context) error {
	if err := w.processCompletedTasks(ctx); err != nil {
		return fmt.Errorf("failed to process completed tasks: %w", err)
	}
	if err := w.processFSOperations(ctx); err != nil {
		return fmt.Errorf("failed to process fs operations: %w", err)
	}
	return nil
}

func (w *AutoDreamWorker) processCompletedTasks(ctx context.Context) error {
	query := "SELECT id, payload FROM shared_tasks WHERE status = 'COMPLETED' AND auto_dreamed = FALSE LIMIT 50"
	if !w.isSQLite {
		query += " FOR UPDATE SKIP LOCKED"
	}

	rows, err := w.db.QueryContext(ctx, query)
	if err != nil {
		return err
	}
	defer rows.Close()

	type taskData struct {
		id      string
		payload string
	}
	var tasks []taskData

	for rows.Next() {
		var t taskData
		if err := rows.Scan(&t.id, &t.payload); err == nil {
			tasks = append(tasks, t)
		}
	}
	if err := rows.Err(); err != nil {
		return err
	}

	for _, t := range tasks {
		embedding := "[0.1, 0.2, 0.3]"
		memID := uuid.New().String()

		var q string
		if w.isSQLite {
			q = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id) VALUES (?, ?, ?, ?)"
		} else {
			q = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id) VALUES ($1, $2, $3::vector, $4)"
		}

		_, insertErr := w.DoDBOperation(ctx, q, memID, t.payload, embedding, t.id)
		if insertErr == nil {
			qUpdate := "UPDATE shared_tasks SET auto_dreamed = TRUE WHERE id = ?"
			if !w.isSQLite {
				qUpdate = "UPDATE shared_tasks SET auto_dreamed = TRUE WHERE id = $1"
			}
			w.DoDBOperation(ctx, qUpdate, t.id)
		}
	}
	return nil
}

func (w *AutoDreamWorker) processFSOperations(ctx context.Context) error {
	memDir := ".agent-task/memory"
	if _, err := os.Stat(memDir); os.IsNotExist(err) {
		return nil
	}

	files, err := os.ReadDir(memDir)
	if err != nil {
		return err
	}

	for _, file := range files {
		if !file.IsDir() && strings.HasSuffix(file.Name(), ".yml") {
			path := filepath.Join(memDir, file.Name())
			contentBytes, err := os.ReadFile(path)
			if err != nil {
				continue
			}
			content := string(contentBytes)

			embedding := "[0.1, 0.2, 0.3]"
			memID := uuid.New().String()

			var q string
			if w.isSQLite {
				q = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id) VALUES (?, ?, ?, ?)"
			} else {
				q = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id) VALUES ($1, $2, $3::vector, $4)"
			}

			_, insertErr := w.DoDBOperation(ctx, q, memID, content, embedding, "fs-task")
			if insertErr == nil {
				os.Remove(path)
			}
		}
	}
	return nil
}

func (w *AutoDreamWorker) DoDBOperation(ctx context.Context, query string, args ...interface{}) (int64, error) {
	result, err := w.db.ExecContext(ctx, query, args...)
	if err != nil {
		return 0, err
	}
	id, err := result.LastInsertId()
	if err != nil {
		return 0, nil
	}
	return id, nil
}
