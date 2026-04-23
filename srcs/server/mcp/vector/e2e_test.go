package vector

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/pgvector/pgvector-go"
	"github.com/stretchr/testify/assert"
)

// TestE2E_CloudMode_MCP verifies the flow of creating a cloud provider via factory
// and storing then searching for vectors using the VectorMCP CallTool handler.
func TestE2E_CloudMode_MCP(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("cloud", db)
	assert.NoError(t, err)
	assert.NotNil(t, provider)

	mcp := NewVectorMCP(provider)

	ctx := context.Background()
	namespace := "e2e_namespace"
	docID := "e2e_doc_1"
	vec := []float32{0.5, 0.5, 0.5}
	metadata := map[string]interface{}{"type": "e2e_cloud"}
	metaJSON, _ := json.Marshal(metadata)

	// Mock Store
	mock.ExpectExec("INSERT INTO mcp_vector_store").
		WithArgs(namespace, docID, pgvector.NewVector(vec), metaJSON).
		WillReturnResult(sqlmock.NewResult(1, 1))

	// Call vector_store tool
	storeArgs := map[string]interface{}{
		"namespace": namespace,
		"id":        docID,
		"vector":    []interface{}{0.5, 0.5, 0.5},
		"metadata":  metadata,
	}

	res, err := mcp.CallTool(ctx, "vector_store", storeArgs)
	assert.NoError(t, err)
	resMap, ok := res.(map[string]interface{})
	assert.True(t, ok)
	assert.Equal(t, "success", resMap["status"])

	// Mock Search
	topK := 1
	rows := sqlmock.NewRows([]string{"id", "embedding", "metadata", "score"}).
		AddRow(docID, pgvector.NewVector(vec), metaJSON, 0.0)

	mock.ExpectQuery("SELECT id, embedding, metadata, embedding <=> \\$1 AS score").
		WithArgs(pgvector.NewVector(vec), namespace, topK).
		WillReturnRows(rows)

	// Call vector_search tool
	searchArgs := map[string]interface{}{
		"namespace":    namespace,
		"query_vector": []interface{}{0.5, 0.5, 0.5},
		"top_k":        topK,
	}

	resSearch, err := mcp.CallTool(ctx, "vector_search", searchArgs)
	assert.NoError(t, err)
	resSearchMap, ok := resSearch.(map[string]interface{})
	assert.True(t, ok)
	assert.Equal(t, "success", resSearchMap["status"])

	results := resSearchMap["results"].([]SearchResult)
	assert.Len(t, results, 1)
	assert.Equal(t, docID, results[0].ID)
	assert.Equal(t, vec, results[0].Vector)
	assert.Equal(t, float32(0.0), results[0].Score)

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}

// TestE2E_StandaloneMode_MCP verifies the flow of creating a standalone provider via factory
// and storing then searching for vectors using the VectorMCP CallTool handler.
func TestE2E_StandaloneMode_MCP(t *testing.T) {
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("standalone", db)
	assert.NoError(t, err)
	assert.NotNil(t, provider)

	mcp := NewVectorMCP(provider)

	ctx := context.Background()
	namespace := "e2e_namespace"
	docID := "e2e_doc_1"
	vec := []float32{0.5, 0.5, 0.5}
	metadata := map[string]interface{}{"type": "e2e_standalone"}

	metaJSON, _ := json.Marshal(metadata)
	vecJSON := float32SliceToJSON(vec)

	// Mock Store
	storeQuery := `
		INSERT INTO mcp_vector_store (namespace, id, embedding, metadata)
		VALUES (?, ?, ?, ?)
		ON CONFLICT (namespace, id)
		DO UPDATE SET embedding = excluded.embedding, metadata = excluded.metadata
	`
	mock.ExpectExec(storeQuery).
		WithArgs(namespace, docID, vecJSON, metaJSON).
		WillReturnResult(sqlmock.NewResult(1, 1))

	// Call vector_store tool
	storeArgs := map[string]interface{}{
		"namespace": namespace,
		"id":        docID,
		"vector":    []interface{}{float64(0.5), float64(0.5), float64(0.5)}, // json decodes numbers as float64 usually
		"metadata":  metadata,
	}

	res, err := mcp.CallTool(ctx, "vector_store", storeArgs)
	assert.NoError(t, err)
	resMap, ok := res.(map[string]interface{})
	assert.True(t, ok)
	assert.Equal(t, "success", resMap["status"])

	// Mock Search
	topK := 1
	rows := sqlmock.NewRows([]string{"id", "embedding", "metadata", "score"}).
		AddRow(docID, vecJSON, metaJSON, 0.0)

	searchQuery := `
		SELECT id, embedding, metadata, vss_distance(embedding, ?) AS score
		FROM mcp_vector_store
		WHERE namespace = ?
		ORDER BY score ASC
		LIMIT ?
	`
	mock.ExpectQuery(searchQuery).
		WithArgs(vecJSON, namespace, topK).
		WillReturnRows(rows)

	// Call vector_search tool
	searchArgs := map[string]interface{}{
		"namespace":    namespace,
		"query_vector": []interface{}{0.5, 0.5, 0.5},
		"top_k":        float64(1), // test json number
	}

	resSearch, err := mcp.CallTool(ctx, "vector_search", searchArgs)
	assert.NoError(t, err)
	resSearchMap, ok := resSearch.(map[string]interface{})
	assert.True(t, ok)
	assert.Equal(t, "success", resSearchMap["status"])

	results := resSearchMap["results"].([]SearchResult)
	assert.Len(t, results, 1)
	assert.Equal(t, docID, results[0].ID)
	assert.Equal(t, vec, results[0].Vector)
	assert.Equal(t, float32(0.0), results[0].Score)

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}

func TestE2E_InvalidMode(t *testing.T) {
	db, _, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("unknown_mode", db)
	assert.Error(t, err)
	assert.Nil(t, provider)
}

func TestVectorMCP_ListTools(t *testing.T) {
	mcp := NewVectorMCP(nil)
	tools := mcp.ListTools()
	assert.Len(t, tools, 2)
	assert.Equal(t, "vector_store", tools[0].Name)
	assert.Equal(t, "vector_search", tools[1].Name)
}

func TestVectorMCP_CallTool_Invalid(t *testing.T) {
	mcp := NewVectorMCP(nil)
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unknown tool")

	_, err = mcp.CallTool(ctx, "vector_store", map[string]interface{}{})
	assert.Error(t, err)

	_, err = mcp.CallTool(ctx, "vector_search", map[string]interface{}{})
	assert.Error(t, err)
}
