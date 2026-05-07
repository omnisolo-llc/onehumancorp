package statemachine

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "github.com/mattn/go-sqlite3"

)

func setupTestDB(t *testing.T) *sql.DB {
	database, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = database.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			agent_id TEXT,
			updated_at DATETIME
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
	require.NoError(t, err)
	return database
}

func TestStateMachine_TransitionValid(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true") // Force sqlite behavior for provider
	database := setupTestDB(t)
	defer database.Close()

	sm := NewStateMachine(database, nil, nil)

	_, err := database.Exec("INSERT INTO shared_tasks (id, status) VALUES ('task-1', 'PENDING')")
	require.NoError(t, err)

	err = sm.Transition(context.Background(), "task-1", "ASSIGNED", "agent-1")
	require.NoError(t, err)

	var status, agent string
	err = database.QueryRow("SELECT status, agent_id FROM shared_tasks WHERE id = 'task-1'").Scan(&status, &agent)
	require.NoError(t, err)
	assert.Equal(t, "ASSIGNED", status)
	assert.Equal(t, "agent-1", agent)
}

func TestStateMachine_TransitionInvalid(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	database := setupTestDB(t)
	defer database.Close()

	sm := NewStateMachine(database, nil, nil)

	_, err := database.Exec("INSERT INTO shared_tasks (id, status) VALUES ('task-1', 'PENDING')")
	require.NoError(t, err)

	err = sm.Transition(context.Background(), "task-1", "REVIEW", "agent-1")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "invalid transition")
}

func TestStateMachine_RedisLock(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false") // Simulate Postgres mode to trigger redis lock

	s, err := miniredis.Run()
	require.NoError(t, err)
	defer s.Close()

	rdb := redis.NewClient(&redis.Options{Addr: s.Addr()})

	database := setupTestDB(t)
	defer database.Close()

	sm := NewStateMachine(database, rdb, nil)

	_, err = database.Exec("INSERT INTO shared_tasks (id, status) VALUES ('task-lock', 'PENDING')")
	require.NoError(t, err)

	// Simulate already locked by another process
	rdb.SetNX(context.Background(), "lock:statemachine:task-lock", "other-agent", 10*time.Second)

	err = sm.Transition(context.Background(), "task-lock", "ASSIGNED", "agent-1")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "could not acquire lock")
}
