package tiers

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"onehumancorp/srcs/server/onboarding"

	_ "github.com/mattn/go-sqlite3"
)

func TestAPIHandler(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	// Create tables for testing
	_, err = db.Exec(`
		CREATE TABLE tenants (
			id TEXT PRIMARY KEY,
			tier TEXT NOT NULL
		);
		CREATE TABLE tier_usage (
			tenant_id TEXT PRIMARY KEY,
			product_count INT DEFAULT 0,
			ai_actions_month INT DEFAULT 0,
			storage_bytes BIGINT DEFAULT 0,
			last_reset_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	tenantID := "test-tenant-1"
	_, err = db.Exec("INSERT INTO tenants (id, tier) VALUES (?, ?)", tenantID, "free")
	if err != nil {
		t.Fatalf("failed to insert tenant: %v", err)
	}

	svc := NewTierService(db)
	handler := NewAPIHandler(svc)

	// Valid Request
	req, err := http.NewRequest("GET", "/api/tiers/check?metric=products", nil)
	if err != nil {
		t.Fatal(err)
	}
	ctx := context.WithValue(req.Context(), onboarding.TenantContextKey, tenantID)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	handler.HandleCheckLimit(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]interface{}
	err = json.Unmarshal(rr.Body.Bytes(), &resp)
	if err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}

	if allowed, ok := resp["allowed"].(bool); !ok || !allowed {
		t.Errorf("expected allowed to be true")
	}

	// Test failure cases: exceeding limits
	err = svc.UpdateUsage(context.Background(), tenantID, "products", 10)
	if err != nil {
		t.Fatalf("failed to update usage")
	}

	req2, err := http.NewRequest("GET", "/api/tiers/check?metric=products", nil)
	if err != nil {
		t.Fatal(err)
	}
	ctx2 := context.WithValue(req2.Context(), onboarding.TenantContextKey, tenantID)
	req2 = req2.WithContext(ctx2)
	rr2 := httptest.NewRecorder()
	handler.HandleCheckLimit(rr2, req2)

	if status := rr2.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp2 map[string]interface{}
	err = json.Unmarshal(rr2.Body.Bytes(), &resp2)
	if err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}

	if allowed, ok := resp2["allowed"].(bool); !ok || allowed {
		t.Errorf("expected allowed to be false")
	}

	// Test missing parameter tenant_id in context
	req3, _ := http.NewRequest("GET", "/api/tiers/check?metric=products", nil)
	rr3 := httptest.NewRecorder()
	handler.HandleCheckLimit(rr3, req3)
	if status := rr3.Code; status != http.StatusUnauthorized {
		t.Errorf("handler returned wrong status code for missing tenant ctx: got %v want %v", status, http.StatusUnauthorized)
	}

	// Test missing parameter metric
	req4, _ := http.NewRequest("GET", "/api/tiers/check", nil)
	ctx4 := context.WithValue(req4.Context(), onboarding.TenantContextKey, tenantID)
	req4 = req4.WithContext(ctx4)
	rr4 := httptest.NewRecorder()
	handler.HandleCheckLimit(rr4, req4)
	if status := rr4.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code for missing param: got %v want %v", status, http.StatusBadRequest)
	}

	// Test invalid method
	req5, _ := http.NewRequest("POST", "/api/tiers/check?metric=products", nil)
	ctx5 := context.WithValue(req5.Context(), onboarding.TenantContextKey, tenantID)
	req5 = req5.WithContext(ctx5)
	rr5 := httptest.NewRecorder()
	handler.HandleCheckLimit(rr5, req5)
	if status := rr5.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("handler returned wrong status code for invalid method: got %v want %v", status, http.StatusMethodNotAllowed)
	}

	// Test service error (simulate db close)
	db.Close()
	req6, _ := http.NewRequest("GET", "/api/tiers/check?metric=products", nil)
	ctx6 := context.WithValue(req6.Context(), onboarding.TenantContextKey, tenantID)
	req6 = req6.WithContext(ctx6)
	rr6 := httptest.NewRecorder()
	handler.HandleCheckLimit(rr6, req6)
	if status := rr6.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code for internal error: got %v want %v", status, http.StatusInternalServerError)
	}
}
