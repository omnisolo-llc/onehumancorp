package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"onehumancorp/srcs/server/orchestration"
	"onehumancorp/srcs/server/repository"
	"github.com/stretchr/testify/assert"
)

type mockTaskStore struct {
	tasks []*orchestration.SharedTask
}

func (m *mockTaskStore) CreateTask(ctx context.Context, task *orchestration.SharedTask) error {
	m.tasks = append(m.tasks, task)
	return nil
}

func (m *mockTaskStore) ClaimTask(ctx context.Context, organizationID string, agentID string) (*orchestration.SharedTask, error) {
	return nil, nil
}
func (m *mockTaskStore) GetTask(ctx context.Context, id string, organizationID string) (*orchestration.SharedTask, error) {
	return nil, nil
}
func (m *mockTaskStore) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*orchestration.SharedTask, error) {
	return nil, nil
}
func (m *mockTaskStore) PollDelegatedTasks(ctx context.Context, limit int) ([]*orchestration.SharedTask, error) {
	return nil, nil
}
func (m *mockTaskStore) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	return nil
}
func (m *mockTaskStore) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	return nil
}

func TestHandleSyncMissions(t *testing.T) {
	store := &mockTaskStore{}
	handler := NewSyncHandler(store)

	task := orchestration.SharedTask{
		ID:             "mission-1",
		Status:         "PENDING",
		OrganizationID: "org-malicious", // Should be ignored
	}
	body, err := json.Marshal(task)
	assert.NoError(t, err)

	req, err := http.NewRequest("POST", "/api/sync/missions", bytes.NewBuffer(body))
	assert.NoError(t, err)

	// Set context to simulate authentication middleware
	ctx := context.WithValue(req.Context(), repository.OrgIDKey, "org-authenticated")
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler.HandleSyncMissions(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)
	assert.Len(t, store.tasks, 1)
	assert.Equal(t, "mission-1", store.tasks[0].ID)
	// Verify tenant isolation
	assert.Equal(t, "org-authenticated", store.tasks[0].OrganizationID)
}

func TestHandleSyncMissions_Unauthorized(t *testing.T) {
	store := &mockTaskStore{}
	handler := NewSyncHandler(store)

	task := orchestration.SharedTask{
		ID:             "mission-2",
		Status:         "PENDING",
		OrganizationID: "org-malicious",
	}
	body, err := json.Marshal(task)
	assert.NoError(t, err)

	req, err := http.NewRequest("POST", "/api/sync/missions", bytes.NewBuffer(body))
	assert.NoError(t, err)

	// No context set, should fail
	rr := httptest.NewRecorder()
	handler.HandleSyncMissions(rr, req)

	assert.Equal(t, http.StatusUnauthorized, rr.Code)
	assert.Len(t, store.tasks, 0)
}

func TestHandleSyncMissions_CrossTenantAccess(t *testing.T) {
	store := &mockTaskStore{}
	handler := NewSyncHandler(store)

	task := orchestration.SharedTask{
		ID:             "mission-cross",
		Status:         "PENDING",
		OrganizationID: "org-malicious", // Should be ignored and overwritten
	}
	body, err := json.Marshal(task)
	assert.NoError(t, err)

	req, err := http.NewRequest("POST", "/api/sync/missions", bytes.NewBuffer(body))
	assert.NoError(t, err)

	// Set context to simulate authentication middleware for a different tenant
	ctx := context.WithValue(req.Context(), repository.OrgIDKey, "org-authenticated")
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler.HandleSyncMissions(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)
	assert.Len(t, store.tasks, 1)
	assert.Equal(t, "mission-cross", store.tasks[0].ID)
	// Verify tenant isolation - the organization ID from the request MUST be ignored
	// and strictly replaced by the one from the authenticated context
	assert.NotEqual(t, "org-malicious", store.tasks[0].OrganizationID)
	assert.Equal(t, "org-authenticated", store.tasks[0].OrganizationID)
}

func TestHandleSyncMissions_UnauthorizedAccess(t *testing.T) {
	store := &mockTaskStore{}
	handler := NewSyncHandler(store)

	task := orchestration.SharedTask{
		ID:             "mission-unauth",
		Status:         "PENDING",
		OrganizationID: "org-malicious",
	}
	body, err := json.Marshal(task)
	assert.NoError(t, err)

	req, err := http.NewRequest("POST", "/api/sync/missions", bytes.NewBuffer(body))
	assert.NoError(t, err)

	// No context set, should fail authorization check
	rr := httptest.NewRecorder()
	handler.HandleSyncMissions(rr, req)

	assert.Equal(t, http.StatusUnauthorized, rr.Code)
	assert.Len(t, store.tasks, 0)
}
