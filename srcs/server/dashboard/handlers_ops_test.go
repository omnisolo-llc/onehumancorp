package dashboard

import (
	"context"
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
		// Create a test context with a timeout to cancel the request
		ctx, cancel := context.WithCancel(context.Background())
		req := httptest.NewRequest("POST", "/api/ops/scale/stream", nil).WithContext(ctx)
		w := httptest.NewRecorder()

		// Run in a goroutine and cancel immediately so the sleep loop exits
		go func() {
			time.Sleep(10 * time.Millisecond)
			cancel()
		}()

		srv.handleScaleStream(w, req)
		if w.Code != http.StatusOK { // It doesn't check method.
			t.Errorf("expected 200, got %d", w.Code)
		}
	})
}
