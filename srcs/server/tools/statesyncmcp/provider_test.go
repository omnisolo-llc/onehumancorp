package statesyncmcp

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"onehumancorp/srcs/server/orchestration"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockLocalDB struct {
	tasks      []*orchestration.SharedTask
	getErr     error
	getByIDErr error
	createErr  error
	updateErr  error
}

func (m *mockLocalDB) PollDelegatedTasks(ctx context.Context, limit int) ([]*orchestration.SharedTask, error) {
	return nil, nil
}

func (m *mockLocalDB) ClaimTask(ctx context.Context, organizationID string, agentID string) (*orchestration.SharedTask, error) {
	return nil, nil
}

func (m *mockLocalDB) CreateTask(ctx context.Context, task *orchestration.SharedTask) error {
	return m.createErr
}

func (m *mockLocalDB) GetTask(ctx context.Context, id string) (*orchestration.SharedTask, error) {
	return nil, m.getByIDErr
}

func (m *mockLocalDB) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	return m.updateErr
}

func (m *mockLocalDB) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*orchestration.SharedTask, error) {
	if m.getErr != nil {
		return nil, m.getErr
	}
	return m.tasks, nil
}

func TestDefaultProvider_SyncUp(t *testing.T) {
	t.Run("empty local db", func(t *testing.T) {
		db := &mockLocalDB{
			tasks: []*orchestration.SharedTask{},
		}
		provider := NewDefaultProvider(db, "http://localhost")

		count, err := provider.SyncUp(context.Background(), "org1")
		assert.NoError(t, err)
		assert.Equal(t, 0, count)
	})

	t.Run("local db error", func(t *testing.T) {
		db := &mockLocalDB{
			getErr: errors.New("db error"),
		}
		provider := NewDefaultProvider(db, "http://localhost")

		count, err := provider.SyncUp(context.Background(), "org1")
		assert.Error(t, err)
		assert.Equal(t, 0, count)
		assert.Contains(t, err.Error(), "db error")
	})

	t.Run("success", func(t *testing.T) {
		tasks := []*orchestration.SharedTask{
			{ID: "task1", OrganizationID: "org1", Status: "PENDING"},
			{ID: "task2", OrganizationID: "org1", Status: "COMPLETED"},
		}
		db := &mockLocalDB{
			tasks: tasks,
		}

		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			assert.Equal(t, "/api/v1/sync/up/bulk", r.URL.Path)
			assert.Equal(t, "org1", r.Header.Get("X-Organization-ID"))
			assert.Equal(t, "application/json", r.Header.Get("Content-Type"))

			var received []*orchestration.SharedTask
			err := json.NewDecoder(r.Body).Decode(&received)
			require.NoError(t, err)
			assert.Len(t, received, 2)
			assert.Equal(t, "task1", received[0].ID)

			w.WriteHeader(http.StatusOK)
		}))
		defer server.Close()

		provider := NewDefaultProvider(db, server.URL)
		count, err := provider.SyncUp(context.Background(), "org1")
		assert.NoError(t, err)
		assert.Equal(t, 2, count)
	})

	t.Run("server error", func(t *testing.T) {
		db := &mockLocalDB{
			tasks: []*orchestration.SharedTask{
				{ID: "task1", OrganizationID: "org1"},
			},
		}

		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusInternalServerError)
		}))
		defer server.Close()

		provider := NewDefaultProvider(db, server.URL)
		count, err := provider.SyncUp(context.Background(), "org1")
		assert.Error(t, err)
		assert.Equal(t, 0, count)
		assert.Contains(t, err.Error(), "bulk sync up failed with status: 500")
	})
}

func TestDefaultProvider_SyncDown(t *testing.T) {
	t.Run("server error", func(t *testing.T) {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusInternalServerError)
		}))
		defer server.Close()

		provider := NewDefaultProvider(&mockLocalDB{}, server.URL)
		count, err := provider.SyncDown(context.Background(), "org1")
		assert.Error(t, err)
		assert.Equal(t, 0, count)
		assert.Contains(t, err.Error(), "status: 500")
	})

	t.Run("success create and update", func(t *testing.T) {
		t1 := time.Now()
		t2 := t1.Add(1 * time.Hour)

		cloudTasks := []*orchestration.SharedTask{
			{ID: "task_new", Status: "PENDING", UpdatedAt: t1},
			{ID: "task_update", Status: "COMPLETED", UpdatedAt: t2},
			{ID: "task_ignore", Status: "PENDING", UpdatedAt: t1}, // Local is newer
		}

		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			assert.Equal(t, "/api/v1/sync/down", r.URL.Path)
			assert.Equal(t, "org1", r.Header.Get("X-Organization-ID"))
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(cloudTasks)
		}))
		defer server.Close()

		// For simplicity, we just use a custom db here.
		customDB := &customMockDB{
			tasks: map[string]*orchestration.SharedTask{
				"task_update": {ID: "task_update", Status: "PENDING", UpdatedAt: t1}, // Older
				"task_ignore": {ID: "task_ignore", Status: "PENDING", UpdatedAt: t2}, // Newer
			},
		}

		provider := NewDefaultProvider(customDB, server.URL)
		count, err := provider.SyncDown(context.Background(), "org1")
		assert.NoError(t, err)
		assert.Equal(t, 2, count) // 1 created, 1 updated

		assert.True(t, customDB.created["task_new"])
		assert.True(t, customDB.updated["task_update"])
		assert.False(t, customDB.updated["task_ignore"])
	})
}

func TestDefaultProvider_GetStatus(t *testing.T) {
	t.Run("success", func(t *testing.T) {
		expected := SyncStatus{
			LastSyncTime: "2023-10-27T10:00:00Z",
			PendingTasks: 5,
			Status:       "ok",
		}

		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			assert.Equal(t, "/api/v1/sync/status", r.URL.Path)
			assert.Equal(t, "org1", r.Header.Get("X-Organization-ID"))
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(expected)
		}))
		defer server.Close()

		provider := NewDefaultProvider(&mockLocalDB{}, server.URL)
		status, err := provider.GetStatus(context.Background(), "org1")
		assert.NoError(t, err)
		assert.Equal(t, expected.PendingTasks, status.PendingTasks)
		assert.Equal(t, expected.Status, status.Status)
	})

	t.Run("server error", func(t *testing.T) {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.WriteHeader(http.StatusNotFound)
		}))
		defer server.Close()

		provider := NewDefaultProvider(&mockLocalDB{}, server.URL)
		status, err := provider.GetStatus(context.Background(), "org1")
		assert.Error(t, err)
		assert.Nil(t, status)
		assert.Contains(t, err.Error(), "status: 404")
	})
}

// customMockDB for more complex SyncDown testing
type customMockDB struct {
	tasks   map[string]*orchestration.SharedTask
	created map[string]bool
	updated map[string]bool
}

func (m *customMockDB) GetTask(ctx context.Context, id string) (*orchestration.SharedTask, error) {
	if t, ok := m.tasks[id]; ok {
		return t, nil
	}
	return nil, errors.New("not found")
}

func (m *customMockDB) CreateTask(ctx context.Context, task *orchestration.SharedTask) error {
	if m.created == nil {
		m.created = make(map[string]bool)
	}
	m.created[task.ID] = true
	return nil
}

func (m *customMockDB) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	if m.updated == nil {
		m.updated = make(map[string]bool)
	}
	m.updated[id] = true
	return nil
}

func (m *customMockDB) PollDelegatedTasks(ctx context.Context, limit int) ([]*orchestration.SharedTask, error) {
	return nil, nil
}

func (m *customMockDB) ClaimTask(ctx context.Context, organizationID string, agentID string) (*orchestration.SharedTask, error) {
	return nil, nil
}

func (m *customMockDB) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*orchestration.SharedTask, error) {
	return nil, nil
}

func TestNoOpProvider(t *testing.T) {
	provider := NewNoOpProvider()

	count, err := provider.SyncUp(context.Background(), "org1")
	assert.NoError(t, err)
	assert.Equal(t, 0, count)

	count, err = provider.SyncDown(context.Background(), "org1")
	assert.NoError(t, err)
	assert.Equal(t, 0, count)

	status, err := provider.GetStatus(context.Background(), "org1")
	assert.NoError(t, err)
	assert.NotNil(t, status)
	assert.Equal(t, "cloud_native", status.Status)
}
