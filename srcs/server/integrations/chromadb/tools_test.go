package chromadb

import (
	"context"
	"testing"
)

func TestChromaDBTool_Execute_CloudModeMock(t *testing.T) {
	t.Setenv("OHC_HEADLESS", "false")
	t.Setenv("OHC_STANDALONE", "false")

	tool := NewChromaDBTool(nil)

	createCollTool := tool.CreateCollectionTool()
	args := []byte(`{"collection_name": "test-coll"}`)
	res, err := createCollTool.Execute(context.Background(), args)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if res == "" {
		t.Errorf("Expected mocked response")
	}

	addDocsTool := tool.AddDocumentsTool()
	args = []byte(`{"collection_id": "test-id", "documents": ["doc1"], "ids": ["1"]}`)
	res, err = addDocsTool.Execute(context.Background(), args)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if res == "" {
		t.Errorf("Expected mocked response")
	}

	queryTool := tool.QueryTool()
	args = []byte(`{"collection_id": "test-id", "query_texts": ["hello"]}`)
	res, err = queryTool.Execute(context.Background(), args)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if res == "" {
		t.Errorf("Expected mocked response")
	}
}
