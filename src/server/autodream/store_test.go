package autodream

import (
	"context"
	"database/sql"
	"errors"
	"math"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	_ "modernc.org/sqlite"
)

type mockPGProvider struct {
	db.Provider
	execArgs  []interface{}
	queryArgs []interface{}
	execErr   error
	queryErr  error
	rows      *mockRows
}

type mockRows struct {
	db.Rows
	nextCount int
	maxNext   int
	scanData  []interface{}
	scanErr   error
	errErr    error
}

func (m *mockRows) Next() bool {
	if m.nextCount < m.maxNext {
		m.nextCount++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	if m.scanErr != nil {
		return m.scanErr
	}
	if len(dest) != len(m.scanData) {
		panic("mismatch scan data")
	}
	for i, d := range dest {
		switch ptr := d.(type) {
		case *string:
			*ptr = m.scanData[i].(string)
		case *sql.NullString:
			ptr.Valid = true
			ptr.String = m.scanData[i].(string)
		}
	}
	return nil
}

func (m *mockRows) Close()     {}
func (m *mockRows) Err() error { return m.errErr }

func (m *mockPGProvider) IsSQLite() bool { return false }

func (m *mockPGProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execArgs = arguments
	return 0, m.execErr
}

func (m *mockPGProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	m.queryArgs = optionsAndArgs
	if m.queryErr != nil {
		return nil, m.queryErr
	}
	if m.rows == nil {
		m.rows = &mockRows{}
	}
	return m.rows, nil
}

func TestPGVectorStore_Store_Success(t *testing.T) {
	mockDB := &mockPGProvider{}
	store := NewPGVectorStore(mockDB)
	ctx := context.Background()

	id := "test-id"
	vector := []float32{0.1, 0.2}
	metadata := map[string]any{"key": "value"}
	content := "test content"

	err := store.Store(ctx, id, vector, metadata, content)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockDB.execArgs) != 4 {
		t.Fatalf("expected 4 arguments, got %d", len(mockDB.execArgs))
	}
	if mockDB.execArgs[0] != id {
		t.Errorf("expected id %s, got %v", id, mockDB.execArgs[0])
	}
	if mockDB.execArgs[1] != content {
		t.Errorf("expected content %s, got %v", content, mockDB.execArgs[1])
	}
	metaStr, ok := mockDB.execArgs[2].(string)
	if !ok || metaStr != `{"key":"value"}` {
		t.Errorf("expected metadata string, got %v", mockDB.execArgs[2])
	}
	embStr, ok := mockDB.execArgs[3].(string)
	if !ok || embStr != `[0.1,0.2]` {
		t.Errorf("expected embedding string, got %v", mockDB.execArgs[3])
	}
}

func TestPGVectorStore_Search_Success(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"test content",
				`{"key":"value"}`,
				`[0.1,0.2]`,
			},
		},
	}
	store := NewPGVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	records, err := store.Search(ctx, vector, 5)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "test-id" {
		t.Errorf("expected id test-id, got %s", records[0].ID)
	}
}

func TestPGVectorStore_Search_QueryError(t *testing.T) {
	mockDB := &mockPGProvider{
		queryErr: errors.New("query error"),
	}
	store := NewPGVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestPGVectorStore_Search_ScanError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanErr: errors.New("scan error"),
		},
	}
	store := NewPGVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestPGVectorStore_Search_UnmarshalMetaError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"test content",
				`invalid json`,
				`[0.1,0.2]`,
			},
		},
	}
	store := NewPGVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestPGVectorStore_Search_UnmarshalEmbError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"test content",
				`{"key":"value"}`,
				`invalid json`,
			},
		},
	}
	store := NewPGVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestSQLiteVectorStore_E2E(t *testing.T) {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)
	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE knowledge_base (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			metadata TEXT,
			embedding TEXT
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	store := NewSQLiteVectorStore(provider)

	id1 := "test-id-1"
	vector1 := []float32{1.0, 0.0, 0.0}
	metadata := map[string]any{"type": "trace"}
	content1 := "some task execution trace"

	err = store.Store(ctx, id1, vector1, metadata, content1)
	if err != nil {
		t.Fatalf("failed to store vector: %v", err)
	}

	id2 := "test-id-2"
	vector2 := []float32{0.0, 1.0, 0.0}
	content2 := "another trace"

	err = store.Store(ctx, id2, vector2, metadata, content2)
	if err != nil {
		t.Fatalf("failed to store vector: %v", err)
	}

	// Search closest to vector1
	records, err := store.Search(ctx, []float32{0.9, 0.1, 0.0}, 5)
	if err != nil {
		t.Fatalf("failed to search vectors: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	if records[0].ID != id1 {
		t.Errorf("expected id %s, got %s", id1, records[0].ID)
	}
	if records[1].ID != id2 {
		t.Errorf("expected id %s, got %s", id2, records[1].ID)
	}
}

func TestSQLiteVectorStore_Search_QueryError(t *testing.T) {
	mockDB := &mockPGProvider{
		queryErr: errors.New("query error"),
	}
	store := NewSQLiteVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestSQLiteVectorStore_Search_ScanError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanErr: errors.New("scan error"),
		},
	}
	store := NewSQLiteVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestSQLiteVectorStore_Search_RowsErr(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 0,
			errErr:  errors.New("rows err"),
		},
	}
	store := NewSQLiteVectorStore(mockDB)
	ctx := context.Background()

	vector := []float32{0.1, 0.2}
	_, err := store.Search(ctx, vector, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestCosineSimilarity(t *testing.T) {
	a := []float32{1, 0, 0}
	b := []float32{0, 1, 0}
	if cosineSimilarity(a, b) != 0 {
		t.Errorf("expected 0")
	}

	a = []float32{1, 0, 0}
	b = []float32{1, 0, 0}
	if cosineSimilarity(a, b) != 1 {
		t.Errorf("expected 1")
	}

	a = []float32{1, 0, 0}
	b = []float32{0, 0, 0}
	if cosineSimilarity(a, b) != 0 {
		t.Errorf("expected 0")
	}

	a = []float32{1, 2}
	b = []float32{1}
	if cosineSimilarity(a, b) != 0 {
		t.Errorf("expected 0 due to length mismatch")
	}
}

func TestPGVectorStore_Store_InvalidMetadata(t *testing.T) {
	store := NewPGVectorStore(&mockPGProvider{})
	err := store.Store(context.Background(), "id", []float32{1.0}, map[string]any{"invalid": make(chan int)}, "content")
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestPGVectorStore_Store_InvalidEmbedding(t *testing.T) {
	store := NewPGVectorStore(&mockPGProvider{})
	err := store.Store(context.Background(), "id", []float32{float32(math.NaN())}, map[string]any{}, "content")
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestPGVectorStore_Search_InvalidEmbedding(t *testing.T) {
	store := NewPGVectorStore(&mockPGProvider{})
	_, err := store.Search(context.Background(), []float32{float32(math.NaN())}, 5)
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestSQLiteVectorStore_Store_InvalidMetadata(t *testing.T) {
	store := NewSQLiteVectorStore(&mockPGProvider{})
	err := store.Store(context.Background(), "id", []float32{1.0}, map[string]any{"invalid": make(chan int)}, "content")
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestSQLiteVectorStore_Store_InvalidEmbedding(t *testing.T) {
	store := NewSQLiteVectorStore(&mockPGProvider{})
	err := store.Store(context.Background(), "id", []float32{float32(math.NaN())}, map[string]any{}, "content")
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}

func TestSQLiteVectorStore_Search_UnmarshalMetaError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"test content",
				`invalid json`,
				`[0.1,0.2]`,
			},
		},
	}
	store := NewSQLiteVectorStore(mockDB)
	_, err := store.Search(context.Background(), []float32{0.1, 0.2}, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestSQLiteVectorStore_Search_UnmarshalEmbError(t *testing.T) {
	mockDB := &mockPGProvider{
		rows: &mockRows{
			maxNext: 1,
			scanData: []interface{}{
				"test-id",
				"test content",
				`{"key":"value"}`,
				`invalid json`,
			},
		},
	}
	store := NewSQLiteVectorStore(mockDB)
	_, err := store.Search(context.Background(), []float32{0.1, 0.2}, 5)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
