package authz

import (
	"context"
	"encoding/json"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockRow struct {
	data []byte
	err  error
}

func (m *mockRow) Scan(dest ...any) error {
	if m.err != nil {
		return m.err
	}
	*dest[0].(*[]byte) = m.data
	return nil
}

type mockProvider struct {
	db.Provider
	row *mockRow
}

func (m *mockProvider) IsSQLite() bool {
	return true
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return m.row
}

func TestCapabilityAuthorizer_Authorize(t *testing.T) {
	tests := []struct {
		name         string
		capabilities []string
		providerErr  error
		capability   string
		wantErr      bool
		errContains  error
	}{
		{
			name:         "Capability allowed",
			capabilities: []string{"read", "write"},
			capability:   "read",
			wantErr:      false,
		},
		{
			name:         "Wildcard allowed",
			capabilities: []string{"*"},
			capability:   "admin_action",
			wantErr:      false,
		},
		{
			name:         "Capability denied",
			capabilities: []string{"read"},
			capability:   "write",
			wantErr:      true,
			errContains:  ErrCapabilityDenied,
		},
		{
			name:         "Provider error",
			capabilities: nil,
			providerErr:  errors.New("db error"),
			capability:   "read",
			wantErr:      true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var data []byte
			if tt.capabilities != nil {
				data, _ = json.Marshal(tt.capabilities)
			}
			provider := &mockProvider{
				row: &mockRow{
					data: data,
					err:  tt.providerErr,
				},
			}
			authorizer := NewCapabilityAuthorizer(provider)

			err := authorizer.Authorize(context.Background(), "session123", tt.capability)

			if (err != nil) != tt.wantErr {
				t.Errorf("Authorize() error = %v, wantErr %v", err, tt.wantErr)
			}

			if tt.errContains != nil && !errors.Is(err, tt.errContains) {
				t.Errorf("Authorize() error = %v, expected to contain %v", err, tt.errContains)
			}
		})
	}
}
