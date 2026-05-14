package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
	_ "github.com/mattn/go-sqlite3"
)

type Pool struct {
	*sqlx.DB
	DriverName string
}

func (p *Pool) IsSQLite() bool {
	return p.DriverName == "sqlite3"
}

type Task struct {
	ID              string    `db:"id"`
	Status          string    `db:"status"`
	Dependencies    string    `db:"dependencies"` // JSON array of task IDs
	AssignedAgentID *string   `db:"assigned_agent_id"`
	UpdatedAt       time.Time `db:"updated_at"`
	Payload         *string   `db:"payload"`
	Title           *string   `db:"title"`
	Description     *string   `db:"description"`
	Priority        *string   `db:"priority"`
}

type Scheduler struct {
	pool *Pool
}

func NewScheduler(db *sqlx.DB, driver string) *Scheduler {
	return &Scheduler{
		pool: &Pool{DB: db, DriverName: driver},
	}
}

// ClaimTask claims a single unblocked task for the specified agent.
func (s *Scheduler) ClaimTask(ctx context.Context, agentID string) (*Task, error) {
	return s.ClaimTaskFromTable(ctx, agentID, "shared_tasks")
}

func (s *Scheduler) ClaimTaskFromTable(ctx context.Context, agentID string, tableName string) (*Task, error) {
	if tableName != "shared_tasks" && tableName != "swarm_tasks" {
		return nil, fmt.Errorf("invalid table name: %s", tableName)
	}
	if s.pool.IsSQLite() {
		return s.claimTaskSQLite(ctx, agentID, tableName)
	}
	return s.claimTaskPostgres(ctx, agentID, tableName)
}

func (s *Scheduler) claimTaskPostgres(ctx context.Context, agentID string, tableName string) (*Task, error) {
	tx, err := s.pool.BeginTxx(ctx, nil)
	if err != nil {
		return nil, fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback()

	// Find unblocked PENDING tasks using FOR UPDATE SKIP LOCKED.
	// We use a CTE to filter out tasks that have incomplete dependencies.
	query := fmt.Sprintf(`
		SELECT t.id, t.dependencies
		FROM %s t
		WHERE t.status = 'PENDING'
		AND NOT EXISTS (
			SELECT 1
			FROM json_array_elements_text(t.dependencies::json) AS parent_id
			JOIN %s p ON p.id = parent_id
			WHERE p.status != 'COMPLETED'
		)
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`, tableName, tableName)

	rows, err := tx.QueryxContext(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch tasks: %w", err)
	}

	var candidate *Task
	for rows.Next() {
		var t Task
		if err := rows.StructScan(&t); err != nil {
			rows.Close()
			return nil, err
		}

		unblocked, err := s.checkDependenciesPostgres(ctx, tx, t.Dependencies, tableName)
		if err != nil {
			rows.Close()
			return nil, err
		}

		if unblocked {
			candidate = &t
			break
		}
	}
	rows.Close()

	if candidate == nil {
		_ = tx.Commit()
		return nil, nil
	}
	t := *candidate

	// Claim it
	now := time.Now().UTC()
	updateQuery := fmt.Sprintf(`
		UPDATE %s
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = $2
		WHERE id = $3
		RETURNING *
	`, tableName)
	var updated Task
	err = tx.QueryRowxContext(ctx, updateQuery, agentID, now, t.ID).StructScan(&updated)
	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	return &updated, nil
}

func (s *Scheduler) claimTaskSQLite(ctx context.Context, agentID string, tableName string) (*Task, error) {
	// SQLite fallback logic without FOR UPDATE SKIP LOCKED.
	tx, err := s.pool.Beginx()
	if err != nil {
		return nil, fmt.Errorf("failed to begin sqlite tx: %w", err)
	}
	defer tx.Rollback()

	// SQLite does not have json_array_elements_text natively in all builds,
	// so we still fetch PENDING tasks and check in memory, but we scan until we find one without LIMIT 10.
	// For a small SQLite DB (fallback mode), full table scan is acceptable.
	query := fmt.Sprintf(`
		SELECT t.id, t.dependencies
		FROM %s t
		WHERE t.status = 'PENDING'
	`, tableName)

	rows, err := tx.QueryxContext(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch tasks: %w", err)
	}

	var candidate *Task
	for rows.Next() {
		var t Task
		if err := rows.StructScan(&t); err != nil {
			rows.Close()
			return nil, err
		}

		unblocked, err := s.checkDependenciesSQLite(ctx, tx, t.Dependencies, tableName)
		if err != nil {
			rows.Close()
			return nil, err
		}

		if unblocked {
			candidate = &t
			break
		}
	}
	rows.Close()

	if candidate == nil {
		return nil, nil
	}

	now := time.Now().UTC()
	updateQuery := fmt.Sprintf(`
		UPDATE %s
		SET status = 'IN_PROGRESS', assigned_agent_id = ?, updated_at = ?
		WHERE id = ? AND status = 'PENDING'
		RETURNING *
	`, tableName)
	var updated Task
	err = tx.QueryRowxContext(ctx, updateQuery, agentID, now, candidate.ID).StructScan(&updated)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // Someone else claimed it
		}
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit: %w", err)
	}

	return &updated, nil
}

func (s *Scheduler) checkDependenciesPostgres(ctx context.Context, tx *sqlx.Tx, depsJSON string, tableName string) (bool, error) {
	var deps []string
	if err := json.Unmarshal([]byte(depsJSON), &deps); err != nil {
		return false, fmt.Errorf("invalid dependencies json: %w", err)
	}

	if len(deps) == 0 {
		return true, nil
	}

	query, args, err := sqlx.In(fmt.Sprintf("SELECT count(*) FROM %s WHERE id IN (?) AND status = 'COMPLETED'", tableName), deps)
	if err != nil {
		return false, fmt.Errorf("in query err: %w", err)
	}
	query = tx.Rebind(query)

	var count int
	err = tx.QueryRowContext(ctx, query, args...).Scan(&count)
	if err != nil {
		return false, fmt.Errorf("count err: %w", err)
	}

	return count == len(deps), nil
}

func (s *Scheduler) checkDependenciesSQLite(ctx context.Context, tx *sqlx.Tx, depsJSON string, tableName string) (bool, error) {
	var deps []string
	if err := json.Unmarshal([]byte(depsJSON), &deps); err != nil {
		return false, fmt.Errorf("invalid dependencies json: %w", err)
	}

	if len(deps) == 0 {
		return true, nil
	}

	query, args, err := sqlx.In(fmt.Sprintf("SELECT count(*) FROM %s WHERE id IN (?) AND status = 'COMPLETED'", tableName), deps)
	if err != nil {
		return false, fmt.Errorf("in query err: %w", err)
	}
	query = tx.Rebind(query)

	var count int
	err = tx.QueryRowContext(ctx, query, args...).Scan(&count)
	if err != nil {
		return false, fmt.Errorf("count err: %w", err)
	}

	return count == len(deps), nil
}
