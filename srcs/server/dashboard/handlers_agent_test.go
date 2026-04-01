package dashboard

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleMissionsSync(t *testing.T) {
	now := time.Now().UTC()
	org := domain.NewSoftwareCompany("demo", "Demo Software Company", "Human CEO", now)
	hub := orchestration.NewHub()
	// Need a SIPDB to delegate missions correctly without panic
	sipdb, _ := orchestration.NewSIPDB(":memory:")
	hub.SetSIPDB(sipdb)

	tracker := billing.NewTracker(billing.DefaultCatalog)

	s := &Server{
		org:     org,
		hub:     hub,
		tracker: tracker,
	}

	t.Run("success", func(t *testing.T) {
		payload := []byte(`{"role":"TEST_ROLE","task":{"id":"m1","content":"Test task","type":"TASK"}}`)
		req := httptest.NewRequest(http.MethodPost, "/api/missions/sync", bytes.NewBuffer(payload))
		req.Header.Set("Content-Type", "application/json")
		rr := httptest.NewRecorder()

		s.handleMissionsSync(rr, req)

		if rr.Code != http.StatusCreated {
			t.Errorf("expected status %d, got %d", http.StatusCreated, rr.Code)
		}
	})

	t.Run("invalid method", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/missions/sync", nil)
		rr := httptest.NewRecorder()

		s.handleMissionsSync(rr, req)

		if rr.Code != http.StatusMethodNotAllowed {
			t.Errorf("expected status %d, got %d", http.StatusMethodNotAllowed, rr.Code)
		}
	})

	t.Run("invalid json", func(t *testing.T) {
		payload := []byte(`invalid json`)
		req := httptest.NewRequest(http.MethodPost, "/api/missions/sync", bytes.NewBuffer(payload))
		rr := httptest.NewRecorder()

		s.handleMissionsSync(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("expected status %d, got %d", http.StatusBadRequest, rr.Code)
		}
	})
}
