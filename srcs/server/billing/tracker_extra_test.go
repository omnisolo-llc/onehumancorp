package billing

import (
	"context"
	"testing"
)

// mockUsageRepo is a simple mock for testing the ActiveOrganizations fallback behavior.
type mockUsageRepo struct {
	orgs []string
	err  error
}

func (m *mockUsageRepo) Track(ctx context.Context, usage Usage) (Usage, error) {
	return usage, nil
}

func (m *mockUsageRepo) Summary(ctx context.Context, organizationID string) (Summary, error) {
	return Summary{}, nil
}

func (m *mockUsageRepo) ActiveOrganizations(ctx context.Context) ([]string, error) {
	return m.orgs, m.err
}

func TestTracker_ActiveOrganizations_WithRepo(t *testing.T) {
	ctx := context.Background()

	tests := []struct{
		name string
		orgs []string
		err error
		expected []string
	}{
		{
			name: "returns orgs when found",
			orgs: []string{"org1", "org2"},
			err: nil,
			expected: []string{"org1", "org2"},
		},
		{
			name: "returns default when empty",
			orgs: []string{},
			err: nil,
			expected: []string{"demo", "default"},
		},
		{
			name: "returns default on error",
			orgs: nil,
			err: context.DeadlineExceeded,
			expected: []string{"demo", "default"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			repo := &mockUsageRepo{orgs: tt.orgs, err: tt.err}
			tracker := NewTrackerWithRepository(DefaultCatalog, repo)
			got := tracker.ActiveOrganizations(ctx)

			if len(got) != len(tt.expected) {
				t.Errorf("expected len %d, got %d", len(tt.expected), len(got))
			}
			for i, v := range got {
				if v != tt.expected[i] {
					t.Errorf("expected %v at index %d, got %v", tt.expected[i], i, v)
				}
			}
		})
	}
}
