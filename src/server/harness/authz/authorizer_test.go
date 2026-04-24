package authz

import (
	"context"
	"testing"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockRow struct {
	caps []string
	err  error
	empty bool
}

func (m *mockRow) Scan(dest ...any) error {
	if m.err != nil {
		return m.err
	}
	if m.empty {
		*dest[0].(*[]byte) = []byte{}
		return nil
	}
	bytes, _ := json.Marshal(m.caps)
	*dest[0].(*[]byte) = bytes
	return nil
}

type mockProvider struct {
	db.Provider
	row *mockRow
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return m.row
}

func TestAuthorizer_Authorize(t *testing.T) {
	t.Run("capability allowed", func(t *testing.T) {
		p := &mockProvider{
			row: &mockRow{caps: []string{"bash"}},
		}
		a := NewAuthorizer(p)

		err := a.Authorize(context.Background(), "sess1", "bash")
		if err != nil {
			t.Errorf("unexpected error: %v", err)
		}
	})

	t.Run("capability denied", func(t *testing.T) {
		p := &mockProvider{
			row: &mockRow{caps: []string{"read"}},
		}
		a := NewAuthorizer(p)

		err := a.Authorize(context.Background(), "sess1", "bash")
		if err != ErrCapabilityDenied {
			t.Errorf("expected ErrCapabilityDenied, got %v", err)
		}
	})

	t.Run("empty capabilities array", func(t *testing.T) {
		p := &mockProvider{
			row: &mockRow{caps: []string{}},
		}
		a := NewAuthorizer(p)

		err := a.Authorize(context.Background(), "sess1", "bash")
		if err != ErrCapabilityDenied {
			t.Errorf("expected ErrCapabilityDenied, got %v", err)
		}
	})

	t.Run("database error", func(t *testing.T) {
		p := &mockProvider{
			row: &mockRow{err: fmt.Errorf("db error")},
		}
		a := NewAuthorizer(p)

		err := a.Authorize(context.Background(), "sess1", "bash")
		if err == nil || err.Error() != "failed to fetch session capabilities: db error" {
			t.Errorf("expected db error, got %v", err)
		}
	})

	t.Run("empty caps string", func(t *testing.T) {
		p := &mockProvider{
			row: &mockRow{empty: true},
		}
		a := NewAuthorizer(p)

		err := a.Authorize(context.Background(), "sess1", "bash")
		if err != ErrCapabilityDenied {
			t.Errorf("expected ErrCapabilityDenied, got %v", err)
		}
	})
}

type mockRowInvalidJSON struct {}
func (m *mockRowInvalidJSON) Scan(dest ...any) error {
	*dest[0].(*[]byte) = []byte("invalid json")
	return nil
}
type mockProviderInvalidJSON struct {
	db.Provider
}
func (m *mockProviderInvalidJSON) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return &mockRowInvalidJSON{}
}

func TestAuthorizer_Authorize_InvalidJSON(t *testing.T) {
	p := &mockProviderInvalidJSON{}
	a := NewAuthorizer(p)

	err := a.Authorize(context.Background(), "sess1", "bash")
	if err == nil {
		t.Errorf("expected unmarshal error, got nil")
	}
}
