package vector

import (
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestPgvectorProvider_Store(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("cloud", db)
	assert.NoError(t, err)

	metadata := map[string]interface{}{"key": "value"}
	vector := []float32{0.1, 0.2, 0.3}

	mock.ExpectExec(`INSERT INTO vector_store \(namespace, id, embedding, metadata\)`).
		WithArgs("ns1", "doc1", `[0.1,0.2,0.3]`, []byte(`{"key":"value"}`)).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = provider.Store("ns1", "doc1", vector, metadata)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPgvectorProvider_Search(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("cloud", db)
	assert.NoError(t, err)

	queryVector := []float32{0.1, 0.2, 0.3}

	rows := sqlmock.NewRows([]string{"id", "distance", "metadata"}).
		AddRow("doc1", 0.1, `{"key":"value"}`)

	mock.ExpectQuery(`SELECT id, \(embedding <=> \$1::vector\) AS distance, metadata`).
		WithArgs(`[0.1,0.2,0.3]`, "ns1", 5).
		WillReturnRows(rows)

	results, err := provider.Search("ns1", queryVector, 5)
	assert.NoError(t, err)
	assert.Len(t, results, 1)
	assert.Equal(t, "doc1", results[0].ID)
	assert.InDelta(t, 0.9, results[0].Score, 0.001)
	assert.Equal(t, "value", results[0].Metadata["key"])
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSqliteVssProvider_Store(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("standalone", db)
	assert.NoError(t, err)

	metadata := map[string]interface{}{"key": "value"}
	vector := []float32{0.1, 0.2, 0.3}

	mock.ExpectExec(`INSERT OR REPLACE INTO vector_store \(namespace, id, embedding, metadata\)`).
		WithArgs("ns1", "doc1", `[0.1,0.2,0.3]`, `{"key":"value"}`).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = provider.Store("ns1", "doc1", vector, metadata)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSqliteVssProvider_Search(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("standalone", db)
	assert.NoError(t, err)

	queryVector := []float32{0.1, 0.2, 0.3}

	rows := sqlmock.NewRows([]string{"id", "distance", "metadata"}).
		AddRow("doc1", 0.1, `{"key":"value"}`)

	mock.ExpectQuery(`SELECT id, vss_distance\(embedding, \?\) AS distance, metadata`).
		WithArgs(`[0.1,0.2,0.3]`, "ns1", 5).
		WillReturnRows(rows)

	results, err := provider.Search("ns1", queryVector, 5)
	assert.NoError(t, err)
	assert.Len(t, results, 1)
	assert.Equal(t, "doc1", results[0].ID)
	assert.InDelta(t, 0.9, results[0].Score, 0.001)
	assert.Equal(t, "value", results[0].Metadata["key"])
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestNewVectorStorageTransport_Invalid(t *testing.T) {
	_, err := NewVectorStorageTransport("invalid", nil)
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "unsupported mode")
}

func TestMCPHandler_ExecuteStore_E2E_Cloud(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("cloud", db)
	assert.NoError(t, err)

	handler := NewMCPHandler(provider)

	args := map[string]interface{}{
		"namespace": "ns1",
		"id":        "doc1",
		"vector":    []interface{}{0.1, 0.2, 0.3},
		"metadata":  map[string]interface{}{"key": "value"},
	}

	mock.ExpectExec(`INSERT INTO vector_store \(namespace, id, embedding, metadata\)`).
		WithArgs("ns1", "doc1", `[0.1,0.2,0.3]`, []byte(`{"key":"value"}`)).
		WillReturnResult(sqlmock.NewResult(1, 1))

	res, err := handler.ExecuteStore(args)
	assert.NoError(t, err)
	assert.Equal(t, "success", res["status"])
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestMCPHandler_ExecuteSearch_E2E_Standalone(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider, err := NewVectorStorageTransport("standalone", db)
	assert.NoError(t, err)

	handler := NewMCPHandler(provider)

	args := map[string]interface{}{
		"namespace":    "ns1",
		"query_vector": []interface{}{0.1, 0.2, 0.3},
		"top_k":        float64(5),
	}

	rows := sqlmock.NewRows([]string{"id", "distance", "metadata"}).
		AddRow("doc1", 0.1, `{"key":"value"}`)

	mock.ExpectQuery(`SELECT id, vss_distance\(embedding, \?\) AS distance, metadata`).
		WithArgs(`[0.1,0.2,0.3]`, "ns1", 5).
		WillReturnRows(rows)

	res, err := handler.ExecuteSearch(args)
	assert.NoError(t, err)
	assert.Equal(t, "success", res["status"])

	results := res["results"].([]map[string]interface{})
	assert.Len(t, results, 1)
	assert.Equal(t, "doc1", results[0]["id"])
	assert.InDelta(t, 0.9, results[0]["score"], 0.001)
	assert.Equal(t, "value", results[0]["metadata"].(map[string]interface{})["key"])

	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestMCPHandler_InvalidStoreArgs(t *testing.T) {
	handler := NewMCPHandler(nil)
	_, err := handler.ExecuteStore(map[string]interface{}{})
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "missing or invalid 'namespace'")
}

func TestMCPHandler_InvalidSearchArgs(t *testing.T) {
	handler := NewMCPHandler(nil)
	_, err := handler.ExecuteSearch(map[string]interface{}{})
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "missing or invalid 'namespace'")
}
