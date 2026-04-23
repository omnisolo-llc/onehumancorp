package vector

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/pgvector/pgvector-go"
	"github.com/stretchr/testify/assert"
)

func TestPGVectorProvider_Store(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider := newPGVectorProvider(db)
	ctx := context.Background()

	namespace := "test_namespace"
	id := "doc1"
	vec := []float32{0.1, 0.2, 0.3}
	metadata := map[string]interface{}{
		"key": "value",
	}
	metaJSON, _ := json.Marshal(metadata)

	mock.ExpectExec("INSERT INTO mcp_vector_store").
		WithArgs(namespace, id, pgvector.NewVector(vec), metaJSON).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = provider.Store(ctx, namespace, id, vec, metadata)
	assert.NoError(t, err)

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}

func TestPGVectorProvider_Search(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider := newPGVectorProvider(db)
	ctx := context.Background()

	namespace := "test_namespace"
	queryVec := []float32{0.1, 0.2, 0.3}
	topK := 2

	metaJSON, _ := json.Marshal(map[string]interface{}{"key": "value"})

	rows := sqlmock.NewRows([]string{"id", "embedding", "metadata", "score"}).
		AddRow("doc1", pgvector.NewVector([]float32{0.1, 0.2, 0.3}), metaJSON, 0.0)

	mock.ExpectQuery("SELECT id, embedding, metadata, embedding <=> \\$1 AS score").
		WithArgs(pgvector.NewVector(queryVec), namespace, topK).
		WillReturnRows(rows)

	results, err := provider.Search(ctx, namespace, queryVec, topK)
	assert.NoError(t, err)
	assert.Len(t, results, 1)
	assert.Equal(t, "doc1", results[0].ID)
	assert.Equal(t, float32(0.0), results[0].Score)
	assert.Equal(t, "value", results[0].Metadata["key"])

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}

func TestSQLiteProvider_Store(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	assert.NoError(t, err)
	defer db.Close()

	provider := newSQLiteProvider(db)
	ctx := context.Background()

	namespace := "test_namespace"
	id := "doc1"
	vec := []float32{0.1, 0.2, 0.3}
	metadata := map[string]interface{}{
		"key": "value",
	}
	metaJSON, _ := json.Marshal(metadata)
	vecJSON := float32SliceToJSON(vec)

	query := `
		INSERT INTO mcp_vector_store (namespace, id, embedding, metadata)
		VALUES (?, ?, ?, ?)
		ON CONFLICT (namespace, id)
		DO UPDATE SET embedding = excluded.embedding, metadata = excluded.metadata
	`

	mock.ExpectExec(query).
		WithArgs(namespace, id, vecJSON, metaJSON).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = provider.Store(ctx, namespace, id, vec, metadata)
	assert.NoError(t, err)

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}

func TestSQLiteProvider_Search(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	assert.NoError(t, err)
	defer db.Close()

	provider := newSQLiteProvider(db)
	ctx := context.Background()

	namespace := "test_namespace"
	queryVec := []float32{0.1, 0.2, 0.3}
	topK := 2

	metaJSON, _ := json.Marshal(map[string]interface{}{"key": "value"})
	vecJSON := float32SliceToJSON([]float32{0.1, 0.2, 0.3})

	rows := sqlmock.NewRows([]string{"id", "embedding", "metadata", "score"}).
		AddRow("doc1", vecJSON, metaJSON, 0.0)

	query := `
		SELECT id, embedding, metadata, vss_distance(embedding, ?) AS score
		FROM mcp_vector_store
		WHERE namespace = ?
		ORDER BY score ASC
		LIMIT ?
	`

	mock.ExpectQuery(query).
		WithArgs(float32SliceToJSON(queryVec), namespace, topK).
		WillReturnRows(rows)

	results, err := provider.Search(ctx, namespace, queryVec, topK)
	assert.NoError(t, err)
	assert.Len(t, results, 1)
	assert.Equal(t, "doc1", results[0].ID)
	assert.Equal(t, float32(0.0), results[0].Score)
	assert.Equal(t, "value", results[0].Metadata["key"])

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}
