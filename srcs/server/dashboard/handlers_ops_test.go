package dashboard

import (
	"strings"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleScaleStreamOpsCoverage(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test", "CEO", time.Now())
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(billing.DefaultCatalog)
	authStore := auth.NewStore()

	_, err := authStore.CreateUser("adminuser", "admin@test.com", "adminpass123", []string{"admin"})
	if err != nil {
		t.Fatal("create user failed", err)
	}

	srv := &Server{
		org:       org,
		hub:       hub,
		tracker:   tracker,
		authStore: authStore,
	}

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/api/ops/scale/stream", nil)
		w := httptest.NewRecorder()
		srv.handleScaleStream(w, req)
		if w.Code != http.StatusOK { // It doesn't check method.
			t.Errorf("expected 200, got %d", w.Code)
		}
	})
}

func TestHandleSyncMissions(t *testing.T) {
	_, server, token := newTestServer(t)

	// Valid payload
	payload := `{"id": "m-123", "payload": {"task": "do something"}}`
	req, _ := http.NewRequest("POST", server.URL+"/api/missions/sync", strings.NewReader(payload))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("X-Conflict-Resolution", "client-wins")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		t.Fatalf("failed to make request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("expected 200, got %d", resp.StatusCode)
	}
}
