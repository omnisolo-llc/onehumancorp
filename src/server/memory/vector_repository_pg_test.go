package memory

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockPgProvider struct {
	db.Provider
	queries []string
	args    [][]any
}

func (m *mockPgProvider) IsSQLite() bool {
	return false
}

func (m *mockPgProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	m.queries = append(m.queries, sql)
	m.args = append(m.args, optionsAndArgs)
	return &mockPgRows{}, nil
}

type mockPgRows struct{}

func (m *mockPgRows) Next() bool { return false }
func (m *mockPgRows) Scan(dest ...any) error { return nil }
func (m *mockPgRows) Close() {}
func (m *mockPgRows) Columns() ([]string, error) { return nil, nil }
func (m *mockPgRows) Err() error { return nil }

func TestVectorRepository_SemanticSearch_Postgres(t *testing.T) {
	mockPg := &mockPgProvider{}
	repo := NewVectorRepository(mockPg)

	ctx := context.Background()
	_, err := repo.SemanticSearch(ctx, "test-org", []float32{1.0, 0.0}, 5)

	if err != nil {
		t.Fatalf("SemanticSearch failed: %v", err)
	}

	if len(mockPg.queries) == 0 {
		t.Fatal("expected Query to be called")
	}

	arg := mockPg.args[0][1]
	if _, ok := arg.(string); !ok {
		t.Errorf("expected string argument for PostgreSQL embedding parameter, got %T", arg)
	}
}
