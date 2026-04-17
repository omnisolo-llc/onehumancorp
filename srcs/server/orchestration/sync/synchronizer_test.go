package sync

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

type mockRepository struct {
	pendingMissions []LocalMission
	activeMissions  []LocalMission
	syncedID        string
	syncError       string
	updatedStatus   string
}

func (m *mockRepository) GetPendingSync(ctx context.Context, limit int) ([]LocalMission, error) {
	return m.pendingMissions, nil
}
func (m *mockRepository) MarkSynced(ctx context.Context, localID string, cloudID string) error {
	m.syncedID = cloudID
	return nil
}
func (m *mockRepository) MarkSyncError(ctx context.Context, localID string, syncError string) error {
	m.syncError = syncError
	return nil
}
func (m *mockRepository) GetActiveEscalations(ctx context.Context) ([]LocalMission, error) {
	return m.activeMissions, nil
}
func (m *mockRepository) UpdateLocalStatus(ctx context.Context, localID string, newStatus string) error {
	m.updatedStatus = newStatus
	return nil
}

func TestCloudSynchronizer_PushPendingMissions(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/missions/escalate" {
			w.WriteHeader(http.StatusAccepted)
			json.NewEncoder(w).Encode(EscalateResponse{CloudID: "c-1", Status: "ACCEPTED"})
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer ts.Close()

	repo := &mockRepository{
		pendingMissions: []LocalMission{
			{ID: "m-1", Status: "BURSTING", Payload: MissionPayload{Role: "test", Task: "task"}},
			{ID: "m-2", Status: "COMPLETED", Payload: MissionPayload{}}, // Should be skipped
		},
	}

	sync := NewCloudSynchronizer(repo, ts.URL)
	err := sync.PushPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if repo.syncedID != "c-1" {
		t.Errorf("expected synced ID c-1, got %s", repo.syncedID)
	}
}

func TestCloudSynchronizer_PushPendingMissions_ErrorHandling(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	repo := &mockRepository{
		pendingMissions: []LocalMission{
			{ID: "m-1", Status: "BURSTING", Payload: MissionPayload{Role: "test", Task: "task"}},
		},
	}

	sync := NewCloudSynchronizer(repo, ts.URL)
	err := sync.PushPendingMissions(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if repo.syncError == "" {
		t.Errorf("expected sync error, got empty")
	}
}

func TestCloudSynchronizer_PullMissionUpdates(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/missions/c-1/status" {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(StatusResponse{CloudID: "c-1", Status: "DONE"})
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer ts.Close()

	repo := &mockRepository{
		activeMissions: []LocalMission{
			{ID: "m-1", CloudMissionID: "c-1", Status: "BURSTING"},
		},
	}

	sync := NewCloudSynchronizer(repo, ts.URL)
	err := sync.PullMissionUpdates(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if repo.updatedStatus != "DONE" {
		t.Errorf("expected status DONE, got %s", repo.updatedStatus)
	}
}
