package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"testing"
)

type MockRow struct {
	ScanFunc func(dest ...interface{}) error
}

func (m *MockRow) Scan(dest ...interface{}) error {
	return m.ScanFunc(dest...)
}

type MockResult struct {
	RowsAffectedCount int64
	RowsAffectedErr   error
}

func (m *MockResult) LastInsertId() (int64, error) {
	return 0, nil
}

func (m *MockResult) RowsAffected() (int64, error) {
	return m.RowsAffectedCount, m.RowsAffectedErr
}

type MockTx struct {
	QueryRowContextFunc func(ctx context.Context, query string, args ...interface{}) DBRow
	ExecContextFunc     func(ctx context.Context, query string, args ...interface{}) (DBResult, error)
	CommitFunc          func() error
	RollbackFunc        func() error
}

func (m *MockTx) QueryRowContext(ctx context.Context, query string, args ...interface{}) DBRow {
	return m.QueryRowContextFunc(ctx, query, args...)
}

func (m *MockTx) ExecContext(ctx context.Context, query string, args ...interface{}) (DBResult, error) {
	return m.ExecContextFunc(ctx, query, args...)
}

func (m *MockTx) Commit() error {
	if m.CommitFunc != nil {
		return m.CommitFunc()
	}
	return nil
}

func (m *MockTx) Rollback() error {
	if m.RollbackFunc != nil {
		return m.RollbackFunc()
	}
	return nil
}

type MockDBProvider struct {
	BeginTxFunc func(ctx context.Context, opts *sql.TxOptions) (DBTx, error)
}

func (m *MockDBProvider) BeginTx(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
	return m.BeginTxFunc(ctx, opts)
}

func TestClaimTask_Success(t *testing.T) {
	provider := &MockDBProvider{
		BeginTxFunc: func(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
			return &MockTx{
				QueryRowContextFunc: func(ctx context.Context, query string, args ...interface{}) DBRow {
					return &MockRow{
						ScanFunc: func(dest ...interface{}) error {
							*dest[0].(*string) = "task-1"
							*dest[1].(*string) = "org-1"
							// other fields omitted for simplicity
							return nil
						},
					}
				},
				ExecContextFunc: func(ctx context.Context, query string, args ...interface{}) (DBResult, error) {
					return &MockResult{RowsAffectedCount: 1}, nil
				},
			}, nil
		},
	}

	dbw := &DBWrapper{Provider: provider}
	type authKey string
	var ClaimsContextKeyForTest authKey = "claims"
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "claims")

	task, err := dbw.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.ID != "task-1" {
		t.Errorf("expected task-1, got %s", task.ID)
	}
	if task.Status != "ASSIGNED" {
		t.Errorf("expected ASSIGNED, got %s", task.Status)
	}
}

func TestClaimTask_NoTask(t *testing.T) {
	provider := &MockDBProvider{
		BeginTxFunc: func(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
			return &MockTx{
				QueryRowContextFunc: func(ctx context.Context, query string, args ...interface{}) DBRow {
					return &MockRow{
						ScanFunc: func(dest ...interface{}) error {
							return sql.ErrNoRows
						},
					}
				},
			}, nil
		},
	}

	dbw := &DBWrapper{Provider: provider}
	type authKey string
	var ClaimsContextKeyForTest authKey = "claims"
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "claims")

	task, err := dbw.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task, got %v", task)
	}
}

func TestClaimTask_AlreadyAssigned(t *testing.T) {
	provider := &MockDBProvider{
		BeginTxFunc: func(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
			return &MockTx{
				QueryRowContextFunc: func(ctx context.Context, query string, args ...interface{}) DBRow {
					return &MockRow{
						ScanFunc: func(dest ...interface{}) error {
							*dest[0].(*string) = "task-1"
							return nil
						},
					}
				},
				ExecContextFunc: func(ctx context.Context, query string, args ...interface{}) (DBResult, error) {
					return &MockResult{RowsAffectedCount: 0}, nil
				},
			}, nil
		},
	}

	dbw := &DBWrapper{Provider: provider}
	type authKey string
	var ClaimsContextKeyForTest authKey = "claims"
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "claims")

	task, err := dbw.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task != nil {
		t.Fatalf("expected nil task, got %v", task)
	}
}

func TestClaimTask_BeginTxError(t *testing.T) {
	provider := &MockDBProvider{
		BeginTxFunc: func(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
			return nil, sql.ErrTxDone
		},
	}

	dbw := &DBWrapper{Provider: provider}
	type authKey string
	var ClaimsContextKeyForTest authKey = "claims"
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "claims")

	_, err := dbw.ClaimTask(ctx, "org-1", "agent-1")
	if err != sql.ErrTxDone {
		t.Errorf("expected ErrTxDone, got %v", err)
	}
}

func TestClaimTask_FallbackPostgres(t *testing.T) {
	provider := &MockDBProvider{
		BeginTxFunc: func(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
			return &MockTx{
				QueryRowContextFunc: func(ctx context.Context, query string, args ...interface{}) DBRow {
					return &MockRow{
						ScanFunc: func(dest ...interface{}) error {
							if query == "		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at\n		FROM shared_tasks\n		WHERE status = 'PENDING' AND organization_id = ?\n		LIMIT 1\n	" {
								return errors.New("pq: syntax error at or near \"?\"")
							}
							*dest[0].(*string) = "task-1"
							return nil
						},
					}
				},
				ExecContextFunc: func(ctx context.Context, query string, args ...interface{}) (DBResult, error) {
					if query == "		UPDATE shared_tasks\n		SET status = 'ASSIGNED', assigned_agent_id = ?, updated_at = CURRENT_TIMESTAMP\n		WHERE id = ? AND status = 'PENDING'\n	" {
						return nil, errors.New("pq: syntax error at or near \"?\"")
					}
					return &MockResult{RowsAffectedCount: 1}, nil
				},
			}, nil
		},
	}

	dbw := &DBWrapper{Provider: provider}
	type authKey string
	var ClaimsContextKeyForTest authKey = "claims"
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "claims")

	task, err := dbw.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.ID != "task-1" {
		t.Errorf("expected task-1, got %s", task.ID)
	}
}

func TestClaimTask_QueryError(t *testing.T) {
	provider := &MockDBProvider{
		BeginTxFunc: func(ctx context.Context, opts *sql.TxOptions) (DBTx, error) {
			return &MockTx{
				QueryRowContextFunc: func(ctx context.Context, query string, args ...interface{}) DBRow {
					return &MockRow{
						ScanFunc: func(dest ...interface{}) error {
							if query == "		SELECT id, organization_id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at\n		FROM shared_tasks\n		WHERE status = 'PENDING' AND organization_id = ?\n		LIMIT 1\n	" {
								return errors.New("some other error")
							}
							return errors.New("some other error")
						},
					}
				},
			}, nil
		},
	}

	dbw := &DBWrapper{Provider: provider}
	type authKey string
	var ClaimsContextKeyForTest authKey = "claims"
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "claims")

	_, err := dbw.ClaimTask(ctx, "org-1", "agent-1")
	if err == nil || err.Error() != "some other error" {
		t.Errorf("expected some other error, got %v", err)
	}
}
