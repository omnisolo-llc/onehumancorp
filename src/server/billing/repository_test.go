package billing

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
)

// mockProvider implements a simple mock of db.Provider
type mockProvider struct {
	db.Provider
}

func (m *mockProvider) Query(ctx context.Context, sql string, args ...any) (db.Rows, error) {
	return &mockRows{orgs: []string{"org1", "org2", "org3"}}, nil
}

type mockRows struct {
	orgs []string
	idx  int
}

func (m *mockRows) Next() bool {
	if m.idx < len(m.orgs) {
		m.idx++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	if m.idx > 0 && m.idx <= len(m.orgs) {
		if ptr, ok := dest[0].(*string); ok {
			*ptr = m.orgs[m.idx-1]
		}
	}
	return nil
}

func (m *mockRows) Close() {}

func (m *mockRows) Columns() ([]string, error) {
	return nil, nil
}

func (m *mockRows) Err() error {
	return nil
}

func TestPgUsageRepository_ActiveOrganizations(t *testing.T) {
	catalog := map[string]Price{
		"test-model": {InputPerMillionUSD: 10.0, OutputPerMillionUSD: 20.0},
	}
	repo := NewPgUsageRepository(&mockProvider{}, catalog)

	orgs, err := repo.ActiveOrganizations(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(orgs) != 3 {
		t.Fatalf("expected 3 orgs, got %d", len(orgs))
	}

	if orgs[0] != "org1" || orgs[1] != "org2" || orgs[2] != "org3" {
		t.Fatalf("unexpected orgs returned: %v", orgs)
	}
}

func TestSqliteUsageRepository_ActiveOrganizations(t *testing.T) {
	catalog := map[string]Price{
		"test-model": {InputPerMillionUSD: 10.0, OutputPerMillionUSD: 20.0},
	}
	repo := NewSqliteUsageRepository(&mockProvider{}, catalog)

	orgs, err := repo.ActiveOrganizations(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(orgs) != 3 {
		t.Fatalf("expected 3 orgs, got %d", len(orgs))
	}

	if orgs[0] != "org1" || orgs[1] != "org2" || orgs[2] != "org3" {
		t.Fatalf("unexpected orgs returned: %v", orgs)
	}
}

func TestTracker_ActiveOrganizations_WithRepo(t *testing.T) {
	catalog := map[string]Price{
		"test-model": {InputPerMillionUSD: 10.0, OutputPerMillionUSD: 20.0},
	}
	repo := NewSqliteUsageRepository(&mockProvider{}, catalog)
	tracker := NewTrackerWithRepository(catalog, repo)

	orgs := tracker.ActiveOrganizations(context.Background())
	if len(orgs) != 3 {
		t.Fatalf("expected 3 orgs, got %d", len(orgs))
	}
	if orgs[0] != "org1" || orgs[1] != "org2" || orgs[2] != "org3" {
		t.Fatalf("unexpected orgs returned: %v", orgs)
	}
}
