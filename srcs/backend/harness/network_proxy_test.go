package harness

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

type mockSandboxAdapter struct {
	violations []string
}

func (m *mockSandboxAdapter) EmitViolation(ctx context.Context, violationType string, agentID string, path string) {
	m.violations = append(m.violations, path)
}

func TestNetworkProxy(t *testing.T) {
	mockTelemetry := &mockSandboxAdapter{}
	proxy := NewNetworkProxy([]string{"allowed.com"}, mockTelemetry, "agent-1")

	reqAllowed, _ := http.NewRequest("GET", "http://allowed.com/test", nil)
	rrAllowed := httptest.NewRecorder()
	proxy.ServeHTTP(rrAllowed, reqAllowed)

	if rrAllowed.Code != http.StatusOK {
		t.Errorf("Expected status OK, got %v", rrAllowed.Code)
	}

	reqDenied, _ := http.NewRequest("GET", "http://denied.com/test", nil)
	rrDenied := httptest.NewRecorder()
	proxy.ServeHTTP(rrDenied, reqDenied)

	if rrDenied.Code != http.StatusForbidden {
		t.Errorf("Expected status Forbidden, got %v", rrDenied.Code)
	}

	if len(mockTelemetry.violations) != 1 {
		t.Errorf("Expected 1 violation, got %v", len(mockTelemetry.violations))
	}
}
