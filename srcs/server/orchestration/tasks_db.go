package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"
)

type DBRow interface {
	Scan(dest ...interface{}) error
}

type DBResult interface {
	LastInsertId() (int64, error)
	RowsAffected() (int64, error)
}

type DBTx interface {
	QueryRowContext(ctx context.Context, query string, args ...interface{}) DBRow
	ExecContext(ctx context.Context, query string, args ...interface{}) (DBResult, error)
	Commit() error
	Rollback() error
}

type DBProvider interface {
	BeginTx(ctx context.Context, opts *sql.TxOptions) (DBTx, error)
}

type DBWrapper struct {
	Provider DBProvider
}

type SharedTask struct {
	ID              string
	OrganizationID  string
	ParentPlanID    *string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Dependencies    json.RawMessage
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// sqlTxWrapper wraps *sql.Tx to implement DBTx
type sqlTxWrapper struct {
	*sql.Tx
}

func (w *sqlTxWrapper) QueryRowContext(ctx context.Context, query string, args ...interface{}) DBRow {
	return w.Tx.QueryRowContext(ctx, query, args...)
}

func (w *sqlTxWrapper) ExecContext(ctx context.Context, query string, args ...interface{}) (DBResult, error) {
	return w.Tx.ExecContext(ctx, query, args...)
}

func (dbw *DBWrapper) ClaimTask(ctx context.Context, organizationID, agentID string) (*SharedTask, error) {
	tx, err := dbw.Provider.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var task SharedTask
	var parentPlanID sql.NullString
	var description sql.NullString
	var assignedAgentID sql.NullString
	var dependencies []byte

	selectQuery := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND organization_id = $1
		LIMIT 1
	`
	selectQueryAgnostic := `
		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
		FROM shared_tasks
		WHERE status = 'PENDING' AND organization_id = ?
		LIMIT 1
	`

	err = tx.QueryRowContext(ctx, selectQueryAgnostic, organizationID).Scan(
		&task.ID,
		&task.OrganizationID,
		&parentPlanID,
		&task.Title,
		&description,
		&task.Status,
		&assignedAgentID,
		&dependencies,
		&task.CreatedAt,
		&task.UpdatedAt,
	)

	if err != nil {
		if err != sql.ErrNoRows {
			// Fallback to postgres specific
			err = tx.QueryRowContext(ctx, selectQuery, organizationID).Scan(
				&task.ID,
				&task.OrganizationID,
				&parentPlanID,
				&task.Title,
				&description,
				&task.Status,
				&assignedAgentID,
				&dependencies,
				&task.CreatedAt,
				&task.UpdatedAt,
			)
			if err != nil {
				if err == sql.ErrNoRows {
					return nil, nil // No task found
				}
				return nil, err
			}
		} else {
			return nil, nil // No task found
		}
	}

	updateQuery := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`
	updateQueryAgnostic := `
		UPDATE shared_tasks
		SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'PENDING'
	`

	res, err := tx.ExecContext(ctx, updateQueryAgnostic, agentID, task.ID)
	if err != nil {
		res, err = tx.ExecContext(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil {
		return nil, err
	}

	if rowsAffected == 0 {
		return nil, nil
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	if parentPlanID.Valid {
		task.ParentPlanID = &parentPlanID.String
	}
	if description.Valid {
		task.Description = &description.String
	}
	task.Status = "ASSIGNED"
	task.AssignedAgentID = &agentID
	if len(dependencies) > 0 {
		task.Dependencies = json.RawMessage(dependencies)
	}

	return &task, nil
}
