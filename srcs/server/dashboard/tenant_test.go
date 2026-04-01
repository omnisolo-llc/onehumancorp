package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
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

func TestTenantRegistry_TenantIsolation(t *testing.T) {
	// A strictly enforced tenant isolation validation test to ensure no cross-contamination
	reg := newTestRegistry()

	ctxOrg1 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
	})

	reqOrg1 := httptest.NewRequest(http.MethodGet, "/api/dashboard", nil).WithContext(ctxOrg1)
	rrOrg1 := httptest.NewRecorder()
	reg.ServeHTTP(rrOrg1, reqOrg1)

	ctxOrg2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		Subject:        "user-2",
		OrganizationID: "org-2",
	})
	reqOrg2 := httptest.NewRequest(http.MethodGet, "/api/dashboard", nil).WithContext(ctxOrg2)
	rrOrg2 := httptest.NewRecorder()
	reg.ServeHTTP(rrOrg2, reqOrg2)

	h1 := reg.handler("org-1")
	h2 := reg.handler("org-2")

	if h1 == nil || h2 == nil {
		t.Fatal("Both tenants should be provisioned")
	}

	// Since NewServer returns a specific http.Handler wrapping mux etc, it's safer to rely on the fact that
	// different requests made with different context get different snapshots.
	rrOrg1Snapshot := httptest.NewRecorder()
	reqOrg1Snapshot := httptest.NewRequest(http.MethodGet, "/api/dashboard", nil).WithContext(ctxOrg1)
	reqOrg1Snapshot.Header.Set("Authorization", "Bearer "+adminToken(t)) // We need some valid token for dashboard endpoint if needed, or we just rely on claimsCtx

	// Ensure isolated contexts via handlers.
	h1.ServeHTTP(rrOrg1Snapshot, reqOrg1Snapshot)

	rrOrg2Snapshot := httptest.NewRecorder()
	reqOrg2Snapshot := httptest.NewRequest(http.MethodGet, "/api/dashboard", nil).WithContext(ctxOrg2)
	reqOrg2Snapshot.Header.Set("Authorization", "Bearer "+adminToken(t))
	h2.ServeHTTP(rrOrg2Snapshot, reqOrg2Snapshot)

	// They shouldn't share the same organization IDs in the snapshots
	if strings.Contains(rrOrg1Snapshot.Body.String(), "org-2") {
		t.Fatal("CRITICAL SECURITY VULNERABILITY: Tenant 1 dashboard exposed Tenant 2 data")
	}
	if strings.Contains(rrOrg2Snapshot.Body.String(), "org-1") {
		t.Fatal("CRITICAL SECURITY VULNERABILITY: Tenant 2 dashboard exposed Tenant 1 data")
	}
}

func TestTenantRegistry_ServeHTTP_Fallback(t *testing.T) {
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
	reg := NewTenantRegistry(sharedAuthStore, nil)
	req := httptest.NewRequest(http.MethodGet, "/api/orgs/register", nil)
	rr := httptest.NewRecorder()
	reg.HandleOrgRegister(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405 Method Not Allowed, got %d", rr.Code)
	}
}

func TestTenantRegistry_HandleOrgRegister_NoAdminRole(t *testing.T) {
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

func TestTenantRegistry_HandleOrgRegister_InvalidJSON(t *testing.T) {
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

func TestNewMultiTenantServer(t *testing.T) {
	handler := NewMultiTenantServer(sharedAuthStore, nil)
	if handler == nil {
		t.Fatal("expected NewMultiTenantServer to return a valid handler")
	}
}
