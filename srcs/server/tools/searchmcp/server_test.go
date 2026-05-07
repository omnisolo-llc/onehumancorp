package searchmcp

import (
	"context"
	"os"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestLocalSearchProvider(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider := NewLocalSearchProvider(db)
	ctx := context.Background()

	t.Run("Search", func(t *testing.T) {
		mock.ExpectQuery("SELECT id, content FROM documents WHERE content MATCH \\? ORDER BY rank LIMIT 10").
			WithArgs("test query").
			WillReturnRows(sqlmock.NewRows([]string{"id", "content"}).AddRow("doc1", "test content"))

		results, err := provider.Search(ctx, "test query")
		assert.NoError(t, err)
		assert.Len(t, results, 1)
		assert.Equal(t, "doc1", results[0].ID)
		assert.Equal(t, "test content", results[0].Content)
	})

	t.Run("Index", func(t *testing.T) {
		mock.ExpectExec("INSERT INTO documents\\(id, content\\) VALUES \\(\\?, \\?\\)").
			WithArgs("doc2", "new content").
			WillReturnResult(sqlmock.NewResult(1, 1))

		err := provider.Index(ctx, Document{ID: "doc2", Content: "new content"})
		assert.NoError(t, err)
	})
}

func TestCloudSearchProvider(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider := NewCloudSearchProvider(db)
	ctx := context.WithValue(context.Background(), ClaimsKey, &Claims{OrganizationID: "org-123"})

	t.Run("Search", func(t *testing.T) {
		mock.ExpectQuery("SELECT id, content FROM documents WHERE tenant_id = \\$1 AND content ILIKE \\$2 LIMIT 10").
			WithArgs("org-123", "%test%").
			WillReturnRows(sqlmock.NewRows([]string{"id", "content"}).AddRow("doc1", "test cloud content"))

		results, err := provider.Search(ctx, "test")
		assert.NoError(t, err)
		assert.Len(t, results, 1)
		assert.Equal(t, "doc1", results[0].ID)
	})

	t.Run("Search_NoClaims", func(t *testing.T) {
		_, err := provider.Search(context.Background(), "test")
		assert.Error(t, err)
		assert.Contains(t, err.Error(), "unauthorized")
	})

	t.Run("Index", func(t *testing.T) {
		mock.ExpectExec("INSERT INTO documents\\(id, tenant_id, content\\) VALUES \\(\\$1, \\$2, \\$3\\)").
			WithArgs("doc2", "org-123", "cloud new").
			WillReturnResult(sqlmock.NewResult(1, 1))

		err := provider.Index(ctx, Document{ID: "doc2", Content: "cloud new"})
		assert.NoError(t, err)
	})

	t.Run("Index_NoClaims", func(t *testing.T) {
		err := provider.Index(context.Background(), Document{ID: "doc2", Content: "cloud new"})
		assert.Error(t, err)
		assert.Contains(t, err.Error(), "unauthorized")
	})
}

func TestHybridSearchMCP(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	provider := NewLocalSearchProvider(db)
	server := NewHybridSearchMCP(provider)
	ctx := context.Background()

	t.Run("ListTools", func(t *testing.T) {
		tools := server.ListTools()
		assert.Len(t, tools, 2)
		assert.Equal(t, "unified_search", tools[0]["name"])
		assert.Equal(t, "index_document", tools[1]["name"])
	})

	t.Run("CallTool_UnifiedSearch", func(t *testing.T) {
		mock.ExpectQuery("SELECT id, content FROM documents WHERE content MATCH \\? ORDER BY rank LIMIT 10").
			WithArgs("foo").
			WillReturnRows(sqlmock.NewRows([]string{"id", "content"}).AddRow("d1", "foo bar"))

		res, err := server.CallTool(ctx, "unified_search", map[string]interface{}{"query": "foo"})
		assert.NoError(t, err)

		resMap, ok := res.(map[string]interface{})
		assert.True(t, ok)
		results := resMap["results"].([]SearchResult)
		assert.Len(t, results, 1)
		assert.Equal(t, "d1", results[0].ID)
	})

	t.Run("CallTool_IndexDocument", func(t *testing.T) {
		mock.ExpectExec("INSERT INTO documents\\(id, content\\) VALUES \\(\\?, \\?\\)").
			WithArgs("d2", "content").
			WillReturnResult(sqlmock.NewResult(1, 1))

		res, err := server.CallTool(ctx, "index_document", map[string]interface{}{"id": "d2", "content": "content"})
		assert.NoError(t, err)

		resMap, ok := res.(map[string]interface{})
		assert.True(t, ok)
		assert.Equal(t, "success", resMap["status"])
	})

	t.Run("CallTool_InvalidArgs", func(t *testing.T) {
		_, err := server.CallTool(ctx, "unified_search", map[string]interface{}{})
		assert.Error(t, err)
	})

	t.Run("CallTool_UnknownTool", func(t *testing.T) {
		_, err := server.CallTool(ctx, "unknown_tool", map[string]interface{}{})
		assert.Error(t, err)
	})
}

func TestFactory(t *testing.T) {
	db, _, _ := sqlmock.New()
	defer db.Close()

	os.Setenv("OHC_STANDALONE", "true")
	p1 := NewProvider(db)
	_, ok1 := p1.(*LocalSearchProvider)
	assert.True(t, ok1)

	os.Setenv("OHC_STANDALONE", "false")
	p2 := NewProvider(db)
	_, ok2 := p2.(*CloudSearchProvider)
	assert.True(t, ok2)
}
