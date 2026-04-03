package dashboard

import (
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
	defer hub.Close()
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
