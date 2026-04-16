package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// sharedAuthStore is used by all tenants in tests so that a single token
// issued by it is accepted by every per-tenant auth middleware.
// NewStore() already seeds a default "admin" user.
var sharedAuthStore = auth.NewStore()

// adminToken returns a valid JWT for the pre-seeded admin user.
func adminToken(t *testing.T) string {
	t.Helper()
	u, err := sharedAuthStore.Authenticate("admin", "admin")
	if err != nil {
		t.Fatalf("authenticate admin: %v", err)
	}
	tok, err := sharedAuthStore.IssueToken(u)
	if err != nil {
		t.Fatalf("issue token: %v", err)
	}
	return tok
}

// newTestRegistry creates a TenantRegistry suitable for unit tests.
func newTestRegistry() *TenantRegistry {
	factory := func(org domain.Organization) http.Handler {
		hub := orchestration.NewHub()
		defer hub.Close()
		tracker := billing.NewTracker(billing.DefaultCatalog)
		return NewServer(org, hub, tracker, sharedAuthStore)
	}
	reg := NewTenantRegistry(sharedAuthStore, factory)

	// Pre-provision two tenants using full software-company orgs so that
	// role profiles are available (required by /api/agents/hire).
	orgA := domain.NewSoftwareCompany("org-a", "Acme Corp", "Alice CEO", time.Now().UTC())
	orgB := domain.NewSoftwareCompany("org-b", "Blorp Inc", "Bob CEO", time.Now().UTC())
	reg.Register("org-a", factory(orgA))
	reg.Register("org-b", factory(orgB))
	return reg
}

// claimsCtx builds a request context carrying auth claims for the given org.
// Used only for testing code-paths that read claims from context directly.
func claimsCtx(orgID string) context.Context {
	return context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "admin-1",
		OrganizationID: orgID,
		Roles:          []string{auth.RoleAdmin},
	})
}

func TestTenantRegistry_RoutesByOrg(t *testing.T) {
	reg := newTestRegistry()
	tok := adminToken(t)

	req := httptest.NewRequest(http.MethodGet, "/healthz", nil).WithContext(claimsCtx("org-a"))
	req.Header.Set("Authorization", "Bearer "+tok)
	rr := httptest.NewRecorder()
	reg.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("org-a /healthz: want 200, got %d", rr.Code)
	}
}

func TestTenantRegistry_UnknownOrgIsLazyProvisioned(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := NewTenantRegistry(sharedAuthStore, nil)
	tok := adminToken(t)

	req := httptest.NewRequest(http.MethodGet, "/healthz", nil).WithContext(claimsCtx("org-unknown"))
	req.Header.Set("Authorization", "Bearer "+tok)
	rr := httptest.NewRecorder()
	reg.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("unknown org should be provisioned on demand: want 200, got %d", rr.Code)
	}
	if h := reg.handler("org-unknown"); h == nil {
		t.Fatal("expected org-unknown to be registered after first authenticated request")
	}
}

func TestTenantRegistry_TenantsAreIsolated(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := newTestRegistry()
	tok := adminToken(t)

	// Hiring an agent in org-a must not appear in org-b.
	reqA := httptest.NewRequest(http.MethodPost, "/api/agents/hire",
		strings.NewReader(`{"name":"Alice","role":"SOFTWARE_ENGINEER"}`)).
		WithContext(claimsCtx("org-a"))
	reqA.Header.Set("Authorization", "Bearer "+tok)
	reqA.Header.Set("Content-Type", "application/json")
	rrA := httptest.NewRecorder()
	reg.ServeHTTP(rrA, reqA)
	if rrA.Code != http.StatusOK {
		t.Fatalf("hire in org-a: want 200, got %d (body=%s)", rrA.Code, rrA.Body.String())
	}

	// org-b dashboard should not include Alice.
	reqB := httptest.NewRequest(http.MethodGet, "/api/dashboard", nil).WithContext(claimsCtx("org-b"))
	reqB.Header.Set("Authorization", "Bearer "+tok)
	rrB := httptest.NewRecorder()
	reg.ServeHTTP(rrB, reqB)
	if rrB.Code != http.StatusOK {
		t.Fatalf("org-b dashboard: want 200, got %d", rrB.Code)
	}
	body := rrB.Body.String()
	if strings.Contains(body, "Alice") {
		t.Errorf("org-b should not see org-a's agent Alice, but body contains it: %s", body)
	}
}

func TestTenantRegistry_HandleOrgRegister(t *testing.T) {
	reg := NewTenantRegistry(sharedAuthStore, nil)

	body := `{"id":"org-new","name":"New Corp","domain":"new.io"}`
	req := httptest.NewRequest(http.MethodPost, "/api/orgs/register", strings.NewReader(body)).
		WithContext(claimsCtx("sys"))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusCreated {
		t.Fatalf("register org: want 201, got %d (body=%s)", rr.Code, rr.Body.String())
	}
	if h := reg.handler("org-new"); h == nil {
		t.Error("org-new should be provisioned after registration")
	}
}

func TestTenantRegistry_AuthenticatedWithoutOrgGetsForbidden(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := newTestRegistry()
	tok := adminToken(t)

	// A request with a valid JWT but an empty org ID must get 403 — not
	// fall through to a random tenant.
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "admin-1",
		OrganizationID: "", // intentionally blank
		Roles:          []string{auth.RoleAdmin},
	})
	req := httptest.NewRequest(http.MethodGet, "/api/dashboard", nil).WithContext(ctx)
	req.Header.Set("Authorization", "Bearer "+tok)
	rr := httptest.NewRecorder()
	reg.ServeHTTP(rr, req)
	if rr.Code != http.StatusForbidden {
		t.Fatalf("authenticated but no org: want 403, got %d (body=%s)", rr.Code, rr.Body.String())
	}
}

func TestTenantRegistry_ServeHTTP_Fallback(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	// Test that unauthenticated requests hit a fresh public handler, not a random tenant
	reg := newTestRegistry()
	// "/api/auth/login" is a valid public route, so we expect 405 Method Not Allowed or 400 Bad Request
	req := httptest.NewRequest(http.MethodGet, "/api/auth/login", nil)
	rr := httptest.NewRecorder()
	reg.ServeHTTP(rr, req)
	// Fallback hits a fresh handler, the tenant auth middleware intercepts if it's not a public route.
	// Since "/api/auth/login" is public, the tenant's router will handle it.
	// Since we send a GET request to a POST endpoint, we expect 405 Method Not Allowed.
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405 Method Not Allowed for public route, got %d", rr.Code)
	}

	// Test fallback when no tenants are registered
	regEmpty := NewTenantRegistry(sharedAuthStore, nil)
	reqEmpty := httptest.NewRequest(http.MethodGet, "/api/auth/login", nil)
	rrEmpty := httptest.NewRecorder()
	regEmpty.ServeHTTP(rrEmpty, reqEmpty)
	if rrEmpty.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405 Method Not Allowed, got %d", rrEmpty.Code)
	}
}

func TestTenantRegistry_HandleOrgRegister_InvalidMethod(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := NewTenantRegistry(sharedAuthStore, nil)
	req := httptest.NewRequest(http.MethodGet, "/api/orgs/register", nil)
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405 Method Not Allowed, got %d", rr.Code)
	}
}

func TestTenantRegistry_HandleOrgRegister_NoAdminRole(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := NewTenantRegistry(sharedAuthStore, nil)
	// Create context with claims but NO admin role
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"user"},
	})
	req := httptest.NewRequest(http.MethodPost, "/api/orgs/register", strings.NewReader("{}")).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden, got %d", rr.Code)
	}
}

func TestTenantRegistry_HandleOrgRegister_TenantAdminForbidden(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := NewTenantRegistry(sharedAuthStore, nil)
	// Create context with claims WITH admin role, but a specific tenant OrganizationID
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "tenant-admin",
		OrganizationID: "org-1",
		Roles:          []string{auth.RoleAdmin},
	})
	req := httptest.NewRequest(http.MethodPost, "/api/orgs/register", strings.NewReader("{}")).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden for tenant admin, got %d", rr.Code)
	}
}

func TestTenantRegistry_HandleOrgRegister_InvalidJSON(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := NewTenantRegistry(sharedAuthStore, nil)
	ctx := claimsCtx("sys") // has admin role
	req := httptest.NewRequest(http.MethodPost, "/api/orgs/register", strings.NewReader("invalid-json")).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request, got %d", rr.Code)
	}
}

func TestTenantRegistry_HandleOrgRegister_MissingFields(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := NewTenantRegistry(sharedAuthStore, nil)
	ctx := claimsCtx("sys") // has admin role
	req := httptest.NewRequest(http.MethodPost, "/api/orgs/register", strings.NewReader(`{"id": "test"}`)).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request for missing name, got %d", rr.Code)
	}

	req2 := httptest.NewRequest(http.MethodPost, "/api/orgs/register", strings.NewReader(`{"name": "test"}`)).WithContext(ctx)
	rr2 := httptest.NewRecorder()
	reg.HandleOrgRegister(rr2, req2)
	if rr2.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request for missing id, got %d", rr2.Code)
	}
}

func TestTenantRegistry_HandleOrgList(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := newTestRegistry()
	ctx := claimsCtx("sys") // has admin role
	req := httptest.NewRequest(http.MethodGet, "/api/orgs", nil).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgList(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected 200 OK, got %d", rr.Code)
	}
	body := rr.Body.String()
	if !strings.Contains(body, "org-a") || !strings.Contains(body, "org-b") {
		t.Errorf("expected response to contain org-a and org-b, got: %s", body)
	}
}

func TestTenantRegistry_HandleOrgList_NoAdminRole(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := newTestRegistry()
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"user"},
	})
	req := httptest.NewRequest(http.MethodGet, "/api/orgs", nil).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgList(rr, req)
	if rr.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden, got %d", rr.Code)
	}
}

func TestTenantRegistry_HandleOrgList_TenantAdminForbidden(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := newTestRegistry()
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "tenant-admin",
		OrganizationID: "org-1",
		Roles:          []string{auth.RoleAdmin},
	})
	req := httptest.NewRequest(http.MethodGet, "/api/orgs", nil).WithContext(ctx)
	rr := httptest.NewRecorder()
	reg.HandleOrgList(rr, req)
	if rr.Code != http.StatusForbidden {
		t.Errorf("expected 403 Forbidden for tenant admin, got %d", rr.Code)
	}
}

func TestNewMultiTenantServer(t *testing.T) {
	handler := NewMultiTenantServer(sharedAuthStore, nil)
	if handler == nil {
		t.Fatal("expected NewMultiTenantServer to return a valid handler")
	}
}

func TestTenantRegistry_ServeHTTP_HybridHealthFallback(t *testing.T) {
	// These tests verify cloud multi-tenant isolation, which is disabled in standalone mode.
	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Unsetenv("OHC_STANDALONE")
	defer func() {
		if originalStandalone != "" {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
		}
	}()

	reg := newTestRegistry()
	req := httptest.NewRequest(http.MethodGet, "/api/health/hybrid", nil)
	rr := httptest.NewRecorder()
	reg.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected 200 OK for /api/health/hybrid public route fallback, got %d", rr.Code)
	}
}
