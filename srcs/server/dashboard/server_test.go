package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
	"onehumancorp/srcs/server/onboarding"
)

func TestHandleStream(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/v1/stream", nil)
	if err != nil {
		t.Fatal(err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	ctx = context.WithValue(ctx, onboarding.TenantContextKey, "test-tenant")
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()

	go func() {
		HandleStream(rr, req)
	}()

	time.Sleep(100 * time.Millisecond)

	GlobalBroker.messages <- "test event"

	time.Sleep(100 * time.Millisecond)
	cancel()
	time.Sleep(100 * time.Millisecond)

	if rr.Code != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			rr.Code, http.StatusOK)
	}

	expectedPrefix := "data: test event\n\n"
	if len(rr.Body.String()) < len(expectedPrefix) || rr.Body.String()[:len(expectedPrefix)] != expectedPrefix {
		t.Errorf("handler returned unexpected body: got %v want prefix %v",
			rr.Body.String(), expectedPrefix)
	}

	// Test middleware rejection for unauthorized request
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/stream", onboarding.TenantAuthMiddleware(HandleStream))
	reqMiddleware, _ := http.NewRequest("GET", "/api/v1/stream", nil)
	rrMiddleware := httptest.NewRecorder()
	mux.ServeHTTP(rrMiddleware, reqMiddleware)
	if status := rrMiddleware.Code; status != http.StatusUnauthorized {
		t.Errorf("middleware returned wrong status code: got %v want %v", status, http.StatusUnauthorized)
	}
}

func TestHandleAutoDreamSync(t *testing.T) {
	req, err := http.NewRequest("POST", "/api/v1/autodream/sync", nil)
	if err != nil {
		t.Fatal(err)
	}
	ctx := context.WithValue(req.Context(), onboarding.TenantContextKey, "test-tenant")
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleAutoDreamSync)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	// Test middleware rejection for unauthorized request
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/sync", onboarding.TenantAuthMiddleware(HandleAutoDreamSync))
	reqMiddleware, _ := http.NewRequest("POST", "/api/v1/autodream/sync", nil)
	rrMiddleware := httptest.NewRecorder()
	mux.ServeHTTP(rrMiddleware, reqMiddleware)
	if status := rrMiddleware.Code; status != http.StatusUnauthorized {
		t.Errorf("middleware returned wrong status code: got %v want %v", status, http.StatusUnauthorized)
	}
}

func TestHandleAutoDreamQuery(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/v1/autodream/query", nil)
	if err != nil {
		t.Fatal(err)
	}
	ctx := context.WithValue(req.Context(), onboarding.TenantContextKey, "test-tenant")
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleAutoDreamQuery)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	// Test middleware rejection for unauthorized request
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/query", onboarding.TenantAuthMiddleware(HandleAutoDreamQuery))
	reqMiddleware, _ := http.NewRequest("GET", "/api/v1/autodream/query", nil)
	rrMiddleware := httptest.NewRecorder()
	mux.ServeHTTP(rrMiddleware, reqMiddleware)
	if status := rrMiddleware.Code; status != http.StatusUnauthorized {
		t.Errorf("middleware returned wrong status code: got %v want %v", status, http.StatusUnauthorized)
	}
}

func TestHandleMeshBroadcast(t *testing.T) {
	req, err := http.NewRequest("POST", "/api/mesh/broadcast", nil)
	if err != nil {
		t.Fatal(err)
	}
	ctx := context.WithValue(req.Context(), onboarding.TenantContextKey, "test-tenant")
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleMeshBroadcast)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	// Test middleware rejection for unauthorized request
	mux := http.NewServeMux()
	mux.HandleFunc("/api/mesh/broadcast", onboarding.TenantAuthMiddleware(HandleMeshBroadcast))
	reqMiddleware, _ := http.NewRequest("POST", "/api/mesh/broadcast", nil)
	rrMiddleware := httptest.NewRecorder()
	mux.ServeHTTP(rrMiddleware, reqMiddleware)
	if status := rrMiddleware.Code; status != http.StatusUnauthorized {
		t.Errorf("middleware returned wrong status code: got %v want %v", status, http.StatusUnauthorized)
	}
}
