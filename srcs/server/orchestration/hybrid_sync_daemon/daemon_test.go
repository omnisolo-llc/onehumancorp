package hybrid_sync_daemon

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
	isSQLite bool
	tasks    []db.TaskRecord
	updated  map[string]string
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.updated == nil {
		m.updated = make(map[string]string)
	}
	if len(arguments) > 0 {
		if id, ok := arguments[0].(string); ok {
			m.updated[id] = "synced"
		}
	}
	return 1, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{tasks: m.tasks, current: -1}, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil
}

func (m *mockProvider) Close() {}

func (m *mockProvider) Ping(ctx context.Context) error { return nil }

func (m *mockProvider) IsSQLite() bool { return m.isSQLite }

func (m *mockProvider) AcquireTask(ctx context.Context, agentID string) (*db.TaskRecord, error) {
	return nil, nil
}

type mockRows struct {
	tasks   []db.TaskRecord
	current int
}

func (m *mockRows) Next() bool {
	m.current++
	return m.current < len(m.tasks)
}

func (m *mockRows) Scan(dest ...any) error {
	t := m.tasks[m.current]
	*dest[0].(*string) = t.ID
	*dest[1].(**string) = t.ParentTaskID
	*dest[2].(**string) = t.AgentID
	*dest[3].(*string) = t.Status
	*dest[4].(**string) = t.Payload
	*dest[5].(*time.Time) = t.CreatedAt
	*dest[6].(*time.Time) = t.UpdatedAt
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }
func (m *mockRows) Err() error { return nil }

type mockCloud struct {
	pushed [][]db.TaskRecord
}

func (m *mockCloud) PushTasks(ctx context.Context, tasks []db.TaskRecord) error {
	m.pushed = append(m.pushed, tasks)
	return nil
}

func TestMissionSyncDaemon(t *testing.T) {
	payload := `{"data":"some data", "email":"test@example.com"}`
	tasks := []db.TaskRecord{
		{
			ID:      "task-1",
			Status:  "pending",
			Payload: &payload,
		},
		{
			ID:      "task-2",
			Status:  "pending",
		},
	}

	provider := &mockProvider{isSQLite: true, tasks: tasks}
	cloud := &mockCloud{}

	daemon := NewMissionSyncDaemon(provider, cloud, 10, time.Millisecond*100)

	err := daemon.syncTasks(context.Background())
	if err != nil {
		t.Fatalf("syncTasks failed: %v", err)
	}

	if len(cloud.pushed) != 1 {
		t.Fatalf("expected 1 push, got %d", len(cloud.pushed))
	}

	if len(cloud.pushed[0]) != 2 {
		t.Fatalf("expected 2 tasks pushed, got %d", len(cloud.pushed[0]))
	}

	// Verify PII scrubbing
	p1 := cloud.pushed[0][0].Payload
	if p1 == nil {
		t.Fatalf("expected payload, got nil")
	}
	if *p1 != `{"data":"some data"}` {
		t.Errorf("expected scrubbed payload, got %s", *p1)
	}

	// Verify status updates
	if provider.updated["task-1"] != "synced" {
		t.Errorf("expected task-1 to be updated")
	}
	if provider.updated["task-2"] != "synced" {
		t.Errorf("expected task-2 to be updated")
	}
}

func TestMissionSyncDaemon_NotSQLite(t *testing.T) {
	provider := &mockProvider{isSQLite: false}
	cloud := &mockCloud{}
	daemon := NewMissionSyncDaemon(provider, cloud, 10, time.Millisecond*100)

	err := daemon.syncTasks(context.Background())
	if err != nil {
		t.Fatalf("syncTasks failed: %v", err)
	}

	if len(cloud.pushed) != 0 {
		t.Fatalf("expected 0 pushes, got %d", len(cloud.pushed))
	}
}
