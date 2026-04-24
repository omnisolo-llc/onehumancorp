package harness

import (
	"context"
	"fmt"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockRows struct {
	action string
	called bool
}

func (m *mockRows) Next() bool {
	if !m.called {
		m.called = true
		return true
	}
	return false
}
func (m *mockRows) Scan(dest ...any) error {
	if len(dest) > 0 {
		if ptr, ok := dest[0].(*string); ok {
			*ptr = m.action
		}
	}
	return nil
}
func (m *mockRows) Close()                     {}
func (m *mockRows) Err() error                 { return nil }
func (m *mockRows) Columns() ([]string, error) { return []string{"action"}, nil }

type mockProvider struct {
	db.Provider
	command string
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	cmd := optionsAndArgs[0].(string)
	if cmd == "ls -la" {
		return &mockRows{action: "ALLOW"}, nil
	} else if cmd == "rm -rf /" {
		return &mockRows{action: "DENY"}, nil
	} else if cmd == "fail_query" {
		return nil, fmt.Errorf("fake query error")
	}
	return nil, fmt.Errorf("no such table")
}

func TestPolicyEngine_AllowSafe(t *testing.T) {
	pe := NewPolicyEngine(&mockProvider{})
	allowed, err := pe.CheckPolicy(context.Background(), "ls -la")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !allowed {
		t.Errorf("Expected safe command 'ls -la' to be allowed")
	}
}

func TestPolicyEngine_DenyMalicious(t *testing.T) {
	pe := NewPolicyEngine(&mockProvider{})
	allowed, err := pe.CheckPolicy(context.Background(), "rm -rf /")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if allowed {
		t.Errorf("Expected malicious command 'rm -rf /' to be denied")
	}
}

func TestPolicyEngine_Fallback(t *testing.T) {
	pe := NewPolicyEngine(&mockProvider{})
	allowed, _ := pe.CheckPolicy(context.Background(), "unknown command")
	if !allowed {
		t.Errorf("Expected fallback to allow safe unknown command")
	}

	allowed, _ = pe.CheckPolicy(context.Background(), "sudo rm -rf /")
	if allowed {
		t.Errorf("Expected fallback to deny rm -rf")
	}
}

func TestPolicyEngine_NilDB(t *testing.T) {
	pe := NewPolicyEngine(nil)
	_, err := pe.CheckPolicy(context.Background(), "ls")
	if err == nil {
		t.Errorf("Expected error for nil db provider")
	}
}

func TestPolicyEngine_QueryError(t *testing.T) {
	pe := NewPolicyEngine(&mockProvider{})
	_, err := pe.CheckPolicy(context.Background(), "fail_query")
	if err == nil {
		t.Errorf("Expected error for failed query")
	}
}

type scanFailRows struct {
	mockRows
}

func (m *scanFailRows) Scan(dest ...any) error {
	return fmt.Errorf("scan error")
}
func (m *scanFailRows) Next() bool {
	if !m.called {
		m.called = true
		return true
	}
	return false
}
func (m *scanFailRows) Close() {}

type mockScanFailProvider struct {
	db.Provider
}

func (m *mockScanFailProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &scanFailRows{}, nil
}

func TestPolicyEngine_ScanError(t *testing.T) {
	pe := NewPolicyEngine(&mockScanFailProvider{})
	_, err := pe.CheckPolicy(context.Background(), "ls")
	if err == nil {
		t.Errorf("Expected scan error")
	}
}

type emptyRows struct {
}

func (m *emptyRows) Next() bool {
	return false
}
func (m *emptyRows) Scan(dest ...any) error {
	return nil
}
func (m *emptyRows) Close()                     {}
func (m *emptyRows) Err() error                 { return nil }
func (m *emptyRows) Columns() ([]string, error) { return []string{}, nil }

type mockEmptyProvider struct {
	db.Provider
}

func (m *mockEmptyProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &emptyRows{}, nil
}
func TestPolicyEngine_Empty(t *testing.T) {
	pe := NewPolicyEngine(&mockEmptyProvider{})
	allowed, _ := pe.CheckPolicy(context.Background(), "ls")
	if allowed {
		t.Errorf("Expected default deny on empty result")
	}
}
