package orchestration

import (
	"context"
	"testing"
	"database/sql"
	"github.com/DATA-DOG/go-sqlmock"
	"github.com/pgvector/pgvector-go"
)

func TestAutoDreamPipeline_DimensionsError(t *testing.T) {
	p := NewAutoDreamPipeline(nil)

	mem := SwarmMemoryEmbedding{
		MemoryID:        "mem-1",
		Context:         "test",
		VectorEmbedding: []float32{0.1, 0.2}, // Not 1536
		SourcePlugin:    "test-plugin",
	}

	err := p.InsertEmbedding(context.Background(), mem)
	if err == nil {
		t.Fatalf("expected dimension error")
	}

	_, err = p.SearchSimilarity(context.Background(), "org-1", []float32{0.1}, 10)
	if err == nil {
		t.Fatalf("expected dimension error")
	}
}

func TestAutoDreamPipeline_InsertAndSearchMock(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

	p := NewAutoDreamPipeline(db)

	vec1536 := make([]float32, 1536)
	for i := range vec1536 {
		vec1536[i] = 0.1
	}

	mem := SwarmMemoryEmbedding{
		MemoryID:        "mem-1",
		Context:         "test context",
		VectorEmbedding: vec1536,
		SourcePlugin:    "test-plugin",
        OrganizationID:  "org-1",
	}

	mock.ExpectExec("INSERT INTO swarm_memory_embeddings").
		WithArgs("mem-1", "test context", pgvector.NewVector(vec1536), "test-plugin", "org-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = p.InsertEmbedding(context.Background(), mem)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rows := sqlmock.NewRows([]string{"memory_id", "context", "vector_embedding", "source_plugin", "organization_id"}).
		AddRow("mem-1", "test context", pgvector.NewVector(vec1536), "test-plugin", "org-1")

	mock.ExpectQuery("SELECT memory_id, context, vector_embedding, source_plugin, organization_id").
		WithArgs("org-1", pgvector.NewVector(vec1536), 10).
		WillReturnRows(rows)

	res, err := p.SearchSimilarity(context.Background(), "org-1", vec1536, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(res) != 1 {
		t.Fatalf("expected 1 result, got %d", len(res))
	}
	if res[0].MemoryID != "mem-1" {
		t.Fatalf("expected mem-1, got %s", res[0].MemoryID)
	}
}

func TestAutoDreamPipeline_SearchMockError(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

	p := NewAutoDreamPipeline(db)

	vec1536 := make([]float32, 1536)

    // test query error
	mock.ExpectQuery("SELECT memory_id, context, vector_embedding, source_plugin, organization_id").
		WithArgs("org-1", pgvector.NewVector(vec1536), 10).
		WillReturnError(sql.ErrConnDone)

	_, err = p.SearchSimilarity(context.Background(), "org-1", vec1536, 10)
	if err == nil {
		t.Fatalf("expected error")
	}
}

func TestAutoDreamPipeline_ScanError(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

	p := NewAutoDreamPipeline(db)

	vec1536 := make([]float32, 1536)

	// Rows with invalid type to force scan error
	rows := sqlmock.NewRows([]string{"memory_id", "context", "vector_embedding", "source_plugin", "organization_id"}).
		AddRow("mem-1", "test context", "invalid_vector", "test-plugin", "org-1")

	mock.ExpectQuery("SELECT memory_id, context, vector_embedding, source_plugin, organization_id").
		WithArgs("org-1", pgvector.NewVector(vec1536), 10).
		WillReturnRows(rows)

	_, err = p.SearchSimilarity(context.Background(), "org-1", vec1536, 10)
	if err == nil {
		t.Fatalf("expected scan error")
	}
}

func TestAutoDreamPipeline_RowErr(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to open sqlmock: %v", err)
	}
	defer db.Close()

	p := NewAutoDreamPipeline(db)

	vec1536 := make([]float32, 1536)

	rows := sqlmock.NewRows([]string{"memory_id", "context", "vector_embedding", "source_plugin", "organization_id"}).
        RowError(0, sql.ErrConnDone).
		AddRow("mem-1", "test context", pgvector.NewVector(vec1536), "test-plugin", "org-1")

	mock.ExpectQuery("SELECT memory_id, context, vector_embedding, source_plugin, organization_id").
		WithArgs("org-1", pgvector.NewVector(vec1536), 10).
		WillReturnRows(rows)

	_, err = p.SearchSimilarity(context.Background(), "org-1", vec1536, 10)
	if err == nil {
		t.Fatalf("expected rows error")
	}
}
