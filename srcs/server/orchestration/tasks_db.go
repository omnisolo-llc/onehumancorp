package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"
)

type SharedTask struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organization_id"`
	Title          string    `json:"title"`
	Status         string    `json:"status"`
	Dependencies   []string  `json:"dependencies"`
	CreatedAt      time.Time `json:"created_at"`
}

type TasksDB struct {
	db     *sql.DB
	isPg   bool
	sqliteMutex chan struct{}
}

func NewTasksDB(driverName, dataSourceName string) (*TasksDB, error) {
	db, err := sql.Open(driverName, dataSourceName)
	if err != nil {
		return nil, err
	}

	isPg := driverName == "postgres"
	var sqliteMutex chan struct{}
	if !isPg {
		sqliteMutex = make(chan struct{}, 1)
		sqliteMutex <- struct{}{}
	}

	return &TasksDB{
		db:          db,
		isPg:        isPg,
		sqliteMutex: sqliteMutex,
	}, nil
}

func (tdb *TasksDB) CreateSharedTask(ctx context.Context, task *SharedTask) error {
	depsJSON, err := json.Marshal(task.Dependencies)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO shared_tasks (id, organization_id, title, status, dependencies, created_at)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	if !tdb.isPg {
		query = `
			INSERT INTO shared_tasks (id, organization_id, title, status, dependencies, created_at)
			VALUES (?, ?, ?, ?, ?, ?)
		`
	}

	_, err = tdb.db.ExecContext(ctx, query, task.ID, task.OrganizationID, task.Title, task.Status, string(depsJSON), task.CreatedAt)
	return err
}

func (tdb *TasksDB) ClaimTask(ctx context.Context, organizationID string) (*SharedTask, error) {
	if !tdb.isPg {
		<-tdb.sqliteMutex
		defer func() { tdb.sqliteMutex <- struct{}{} }()
	}

	tx, err := tdb.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	query := `
		SELECT id, title, status, dependencies, created_at
		FROM shared_tasks
		WHERE organization_id = $1 AND status = 'PENDING'
		FOR UPDATE SKIP LOCKED
		LIMIT 1
	`
	if !tdb.isPg {
		query = `
			SELECT id, title, status, dependencies, created_at
			FROM shared_tasks
			WHERE organization_id = ? AND status = 'PENDING'
			LIMIT 1
		`
	}

	var task SharedTask
	task.OrganizationID = organizationID
	var depsJSON string

	err = tx.QueryRowContext(ctx, query, organizationID).Scan(&task.ID, &task.Title, &task.Status, &depsJSON, &task.CreatedAt)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No task available
		}
		return nil, err
	}

	json.Unmarshal([]byte(depsJSON), &task.Dependencies)

	updateQuery := `UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = $1`
	if !tdb.isPg {
		updateQuery = `UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = ?`
	}

	_, err = tx.ExecContext(ctx, updateQuery, task.ID)
	if err != nil {
		return nil, err
	}

	err = tx.Commit()
	if err != nil {
		return nil, err
	}

	task.Status = "IN_PROGRESS"
	return &task, nil
}
