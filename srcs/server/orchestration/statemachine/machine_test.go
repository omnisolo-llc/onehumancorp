package statemachine

import (
	"context"
	"strings"
	"testing"
	"database/sql"

	"github.com/alicebob/miniredis/v2"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
	_ "modernc.org/sqlite"
)

// Helper to create an in-memory test provider
func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	dbProvider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	tx, err := dbProvider.Begin(ctx)
	if err != nil {
		t.Fatalf("Failed to begin tx: %v", err)
	}

	_, err = tx.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			description TEXT,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority INTEGER DEFAULT 0,
			agent_id TEXT,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE state_machine_transitions (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	err = tx.Commit(ctx)
	if err != nil {
		t.Fatalf("Failed to commit tables: %v", err)
	}

	return dbProvider
}

func TestStateMachine_Transition(t *testing.T) {
	ctx := context.Background()
	dbProvider := setupTestDB(t)
	defer dbProvider.Close()

	// Insert initial test task
	taskID := generateID()
	tx, _ := dbProvider.Begin(ctx)
	_, err := tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, payload, status) VALUES ($1, 'org1', 'Test Task', '{}', 'PENDING')`, taskID)
	tx.Commit(ctx)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	var broadcastPayload map[string]interface{}
	broadcast := func(entityID string, payload map[string]interface{}) {
		broadcastPayload = payload
	}

	sm := NewStateMachine(dbProvider, broadcast, nil)

	// 1. Test Valid Transition: PENDING -> IN_PROGRESS
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateInProgress, "agent1", "Starting task")
	if err != nil {
		t.Errorf("Expected transition to succeed, got: %v", err)
	}

	// Verify broadcast
	if broadcastPayload == nil || broadcastPayload["to_state"] != StateInProgress {
		t.Errorf("Expected broadcast with to_state IN_PROGRESS, got: %v", broadcastPayload)
	}

	// Verify DB state
	var currentStatus string
	tx, _ = dbProvider.Begin(ctx)
	err = tx.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = $1", taskID).Scan(&currentStatus)
	tx.Commit(ctx)
	if err != nil || currentStatus != StateInProgress {
		t.Errorf("Expected status IN_PROGRESS, got: %s (err: %v)", currentStatus, err)
	}

	// Verify transition log
	var fromState, toState string
	tx, _ = dbProvider.Begin(ctx)
	err = tx.QueryRow(ctx, "SELECT from_state, to_state FROM state_machine_transitions WHERE entity_id = $1 ORDER BY occurred_at DESC LIMIT 1", taskID).Scan(&fromState, &toState)
	tx.Commit(ctx)
	if err != nil || fromState != StatePending || toState != StateInProgress {
		t.Errorf("Expected audit log PENDING -> IN_PROGRESS, got: %s -> %s (err: %v)", fromState, toState, err)
	}

	// 2. Test Invalid Transition: IN_PROGRESS -> ASSIGNED
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateAssigned, "agent1", "Invalid move")
	if err == nil || !strings.Contains(err.Error(), "invalid transition from IN_PROGRESS to ASSIGNED") {
		t.Errorf("Expected invalid transition error, got: %v", err)
	}

	// 3. Test Entity Not Found
	err = sm.Transition(ctx, "nonexistent", "SHARED_TASK", StateInProgress, "agent1", "Should fail")
	if err == nil {
		t.Errorf("Expected entity not found error")
	}

	// 4. Test Unsupported Entity Type
	err = sm.Transition(ctx, taskID, "UNKNOWN_TYPE", StateInProgress, "agent1", "Should fail")
	if err == nil {
		t.Errorf("Expected unsupported entity type error")
	}

	// 5. Test Same State Transition (No-op)
	broadcastPayload = nil
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateInProgress, "agent1", "Noop")
	if err != nil {
		t.Errorf("Expected no-op to succeed, got: %v", err)
	}
	if broadcastPayload != nil {
		t.Errorf("Expected no broadcast for no-op transition, got: %v", broadcastPayload)
	}
}

type mockTx struct {
	db.Tx
	queryRowFunc func(ctx context.Context, sql string, args ...any) db.Row
	execFunc     func(ctx context.Context, sql string, arguments ...any) (int64, error)
	commitFunc   func(ctx context.Context) error
	rollbackFunc func(ctx context.Context) error
}

func (m *mockTx) QueryRow(ctx context.Context, sql string, args ...any) db.Row {
	if m.queryRowFunc != nil {
		return m.queryRowFunc(ctx, sql, args...)
	}
	return m.Tx.QueryRow(ctx, sql, args...)
}
func (m *mockTx) Exec(ctx context.Context, sql string, args ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, args...)
	}
	return m.Tx.Exec(ctx, sql, args...)
}
func (m *mockTx) Commit(ctx context.Context) error {
	if m.commitFunc != nil {
		return m.commitFunc(ctx)
	}
	return m.Tx.Commit(ctx)
}
func (m *mockTx) Rollback(ctx context.Context) error {
	if m.rollbackFunc != nil {
		return m.rollbackFunc(ctx)
	}
	return m.Tx.Rollback(ctx)
}

type mockProvider struct {
	db.Provider
	isSQLite bool
	beginFunc func(ctx context.Context) (db.Tx, error)
}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}
func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	if m.beginFunc != nil {
		return m.beginFunc(ctx)
	}
	return m.Provider.Begin(ctx)
}

type mockRow struct {
	scanFunc func(dest ...any) error
}

func (m *mockRow) Scan(dest ...any) error {
	return m.scanFunc(dest...)
}

func TestStateMachine_Transition_PostgresBranch(t *testing.T) {
	ctx := context.Background()

	provider := &mockProvider{
		isSQLite: false,
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return &mockTx{
				queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
					if !strings.Contains(query, "FOR UPDATE") {
						t.Errorf("Expected FOR UPDATE in postgres query, got: %s", query)
					}
					return &mockRow{
						scanFunc: func(dest ...any) error {
							*dest[0].(*string) = StatePending
							return nil
						},
					}
				},
				execFunc: func(ctx context.Context, query string, args ...any) (int64, error) {
					return 1, nil
				},
				commitFunc: func(ctx context.Context) error {
					return nil
				},
				rollbackFunc: func(ctx context.Context) error {
					return nil
				},
			}, nil
		},
	}

	sm := NewStateMachine(provider, nil, nil)
	err := sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err != nil {
		t.Errorf("Expected success, got: %v", err)
	}
}

func TestStateMachine_Transition_RedisLock(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to create miniredis: %v", err)
	}
	defer mr.Close()

	opts, err := rueidis.ParseURL("redis://" + mr.Addr())
	if err != nil {
		t.Fatalf("Failed to parse miniredis url: %v", err)
	}
	opts.DisableCache = true
	client, err := rueidis.NewClient(opts)
	if err != nil {
		t.Fatalf("Failed to create rueidis client: %v", err)
	}
	defer client.Close()

	ctx := context.Background()
	dbProvider := setupTestDB(t)
	defer dbProvider.Close()

	taskID := generateID()
	tx, _ := dbProvider.Begin(ctx)
	_, err = tx.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, payload, status) VALUES ($1, 'org1', 'Test Task', '{}', 'PENDING')`, taskID)
	tx.Commit(ctx)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	sm := NewStateMachine(dbProvider, nil, client)

	// Hold the lock to simulate contention
	lockKey := "ohc:lock:state_machine:" + taskID
	mr.Set(lockKey, "1")

	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateInProgress, "agent1", "Starting task")
	if err == nil || !strings.Contains(err.Error(), "lock is held") {
		t.Errorf("Expected lock held error, got: %v", err)
	}

	// Release lock and try again
	mr.Del(lockKey)
	err = sm.Transition(ctx, taskID, "SHARED_TASK", StateInProgress, "agent1", "Starting task")
	if err != nil {
		t.Errorf("Expected transition to succeed, got: %v", err)
	}
}

func TestStateMachine_Transition_Errors(t *testing.T) {
	ctx := context.Background()
	provider := &mockProvider{
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return nil, context.DeadlineExceeded
		},
	}

	sm := NewStateMachine(provider, nil, nil)
	err := sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err == nil || !strings.Contains(err.Error(), "failed to begin transaction") {
		t.Errorf("Expected begin transaction error, got: %v", err)
	}

	provider = &mockProvider{
		isSQLite: true,
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return &mockTx{
				queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
					return &mockRow{
						scanFunc: func(dest ...any) error {
							return sql.ErrNoRows
						},
					}
				},
				rollbackFunc: func(ctx context.Context) error { return nil },
			}, nil
		},
	}
	sm = NewStateMachine(provider, nil, nil)
	err = sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err == nil || !strings.Contains(err.Error(), "entity not found") {
		t.Errorf("Expected entity not found error, got: %v", err)
	}

	provider = &mockProvider{
		isSQLite: true,
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return &mockTx{
				queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
					return &mockRow{
						scanFunc: func(dest ...any) error {
							*dest[0].(*string) = StatePending
							return nil
						},
					}
				},
				execFunc: func(ctx context.Context, query string, args ...any) (int64, error) {
					if strings.Contains(query, "UPDATE shared_tasks") {
						return 0, context.DeadlineExceeded
					}
					return 1, nil
				},
				rollbackFunc: func(ctx context.Context) error { return nil },
			}, nil
		},
	}
	sm = NewStateMachine(provider, nil, nil)
	err = sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err == nil || !strings.Contains(err.Error(), "failed to update entity state") {
		t.Errorf("Expected update entity state error, got: %v", err)
	}

	provider = &mockProvider{
		isSQLite: true,
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return &mockTx{
				queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
					return &mockRow{
						scanFunc: func(dest ...any) error {
							*dest[0].(*string) = StatePending
							return nil
						},
					}
				},
				execFunc: func(ctx context.Context, query string, args ...any) (int64, error) {
					if strings.Contains(query, "INSERT INTO state_machine_transitions") {
						return 0, context.DeadlineExceeded
					}
					return 1, nil
				},
				rollbackFunc: func(ctx context.Context) error { return nil },
			}, nil
		},
	}
	sm = NewStateMachine(provider, nil, nil)
	err = sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err == nil || !strings.Contains(err.Error(), "failed to record transition audit log") {
		t.Errorf("Expected audit log error, got: %v", err)
	}

	provider = &mockProvider{
		isSQLite: true,
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return &mockTx{
				queryRowFunc: func(ctx context.Context, sql string, args ...any) db.Row {
					return &mockRow{
						scanFunc: func(dest ...any) error {
							*dest[0].(*string) = StatePending
							return nil
						},
					}
				},
				execFunc: func(ctx context.Context, sql string, args ...any) (int64, error) {
					return 1, nil
				},
				commitFunc: func(ctx context.Context) error {
					return context.DeadlineExceeded
				},
				rollbackFunc: func(ctx context.Context) error { return nil },
			}, nil
		},
	}
	sm = NewStateMachine(provider, nil, nil)
	err = sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err == nil || !strings.Contains(err.Error(), "failed to commit transaction") {
		t.Errorf("Expected commit error, got: %v", err)
	}

	provider = &mockProvider{
		isSQLite: true,
		beginFunc: func(ctx context.Context) (db.Tx, error) {
			return &mockTx{
				queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
					return &mockRow{
						scanFunc: func(dest ...any) error {
							return context.DeadlineExceeded
						},
					}
				},
				rollbackFunc: func(ctx context.Context) error { return nil },
			}, nil
		},
	}
	sm = NewStateMachine(provider, nil, nil)
	err = sm.Transition(ctx, "123", "SHARED_TASK", StateInProgress, "agent1", "reason")
	if err == nil || !strings.Contains(err.Error(), "failed to read current state") {
		t.Errorf("Expected read current state error, got: %v", err)
	}
}

func TestStateMachine_AllTransitions(t *testing.T) {
	allStates := []string{
		StatePending, StateAssigned, StateExecuting, StateWaitingDelegation,
		StateReview, StateSuccess, StateTerminatedError,
		StateInProgress, StateCompleted, StateFailed,
	}

	for _, from := range allStates {
		for _, to := range allStates {
			validNextStates, ok := ValidTransitions[from]

			isValid := false
			if from == to {
				isValid = true
			} else if ok {
				for _, s := range validNextStates {
					if s == to {
						isValid = true
						break
					}
				}
			}

			if got := IsValidTransition(from, to); got != isValid {
				t.Errorf("IsValidTransition(%s, %s) = %v; expected %v", from, to, got, isValid)
			}
		}
	}
}
