package dashboard

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"os"

	"database/sql"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/auth"
	_ "modernc.org/sqlite"
)

func TestHandleHybridHealthCheck(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
	defer os.Unsetenv("OHC_STANDALONE")

	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	provider := db.NewSqliteProvider(sqliteDB)
	defer provider.Close()

	ctx := context.Background()

	// Execute raw schema
	_, err = provider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	hub := orchestration.NewHub()

	// Create a real SIPDB so the mock method gets bypassed or implement the mock setup correctly. Wait, we can construct one using orchestration.NewSIPDBWithProvider.
	realSipDB, err := orchestration.NewSIPDBWithProvider(provider, "test-org")
	if err != nil {
		t.Fatalf("Failed to create SIPDB: %v", err)
	}

	hub.SetSIPDB(realSipDB)
	tracker := billing.NewTracker(billing.DefaultCatalog)
	org := domain.Organization{ID: "test-org"}
	handler := NewServer(org, hub, tracker)

	req := httptest.NewRequest(http.MethodGet, "/api/health/hybrid", nil)

	// Add an empty claims context so auth middleware doesn't block the request
	claims := &auth.Claims{
		Subject:        "test-user",
		OrganizationID: "test-org",
		Roles:          []string{"admin"},
	}

	ctx = context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	w := httptest.NewRecorder()

	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %d", w.Code)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &resp); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	if resp["status"] != "healthy" && resp["status"] != "ok" {
		t.Errorf("Expected status to be healthy or ok, got %s", resp["status"])
	}

	checklist, ok := resp["checklist"].([]interface{})
	if !ok {
		t.Fatalf("Expected checklist in response, got %v", resp)
	}

	if len(checklist) != 2 {
		t.Errorf("Expected 2 items in checklist, got %d", len(checklist))
	}
}