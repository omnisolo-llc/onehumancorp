package hybridfsmcp

import (
	"context"
	"testing"
)

func TestEscalator_AnalyzeComplexity(t *testing.T) {
	e := NewRAGEscalator("cloud")

	if e.AnalyzeComplexity("simple query") {
		t.Errorf("Expected false for simple query")
	}

	if !e.AnalyzeComplexity("this is a massive query") {
		t.Errorf("Expected true for 'massive' query")
	}

	if !e.AnalyzeComplexity("analyze all history") {
		t.Errorf("Expected true for 'all history' query")
	}

	longQuery := ""
	for i := 0; i < 110; i++ {
		longQuery += "a"
	}
	if !e.AnalyzeComplexity(longQuery) {
		t.Errorf("Expected true for long query")
	}
}

func TestEscalator_ExecuteEscalatedRAG(t *testing.T) {
	e := NewRAGEscalator("cloud")
	ctx := context.Background()

	result, err := e.ExecuteEscalatedRAG(ctx, "test query")
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if result != "Cloud RAG Result: test query" {
		t.Errorf("Unexpected result: %s", result)
	}

	eOffline := NewRAGEscalator("offline")
	result, err = eOffline.ExecuteEscalatedRAG(ctx, "test query")
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if result != "Local RAG Result: test query" {
		t.Errorf("Unexpected fallback result: %s", result)
	}
}

func TestHybridFSMCP_RAGQuery(t *testing.T) {
	e := NewRAGEscalator("cloud")
	provider := NewLocalFSProvider(".")
	mcp := NewHybridFSMCPWithEscalator(provider, e)
	ctx := context.Background()

	res, err := mcp.CallTool(ctx, "rag_query", map[string]interface{}{"query": "simple"})
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	m := res.(map[string]interface{})
	if m["content"] != "Local RAG Result: simple" {
		t.Errorf("Expected local result, got: %v", m["content"])
	}

	res, err = mcp.CallTool(ctx, "rag_query", map[string]interface{}{"query": "massive complex query"})
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	m = res.(map[string]interface{})
	if m["content"] != "Cloud RAG Result: massive complex query" {
		t.Errorf("Expected cloud result, got: %v", m["content"])
	}
}
