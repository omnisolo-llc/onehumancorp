package hybridfsmcp

import (
	"context"
	"testing"
)

func TestEscalationRoutingLocal(t *testing.T) {
	escalator := NewComplexityAnalyzer(5)
	provider := NewLocalFSProvider(t.TempDir())
	mcp := NewHybridFSMCPWithEscalator(provider, escalator)

	query := "short simple query" // 3 words, should be local
	args := map[string]interface{}{"query": query}

	res, err := mcp.CallTool(context.Background(), "rag_query", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resultMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("unexpected return type")
	}

	if resultMap["source"] != "local" {
		t.Errorf("expected source local, got %v", resultMap["source"])
	}
}

func TestEscalationRoutingCloud(t *testing.T) {
	escalator := NewComplexityAnalyzer(2)
	provider := NewLocalFSProvider(t.TempDir())
	mcp := NewHybridFSMCPWithEscalator(provider, escalator)

	query := "this is a very complex and long query" // 8 words, should escalate
	args := map[string]interface{}{"query": query}

	res, err := mcp.CallTool(context.Background(), "rag_query", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resultMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("unexpected return type")
	}

	if resultMap["source"] != "cloud" {
		t.Errorf("expected source cloud, got %v", resultMap["source"])
	}
}

func TestEscalationFallback(t *testing.T) {
	escalator := NewComplexityAnalyzer(2)
	provider := NewLocalFSProvider(t.TempDir())
	mcp := NewHybridFSMCPWithEscalator(provider, escalator)

	query := "this is a very complex and long query" // 8 words, should escalate
	args := map[string]interface{}{"query": query}

	// create context that triggers cloud fail
	ctx := context.WithValue(context.Background(), "fail_cloud", true)

	res, err := mcp.CallTool(ctx, "rag_query", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resultMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("unexpected return type")
	}

	if resultMap["source"] != "local_fallback" {
		t.Errorf("expected source local_fallback, got %v", resultMap["source"])
	}
}

func TestEscalationInvalidArgs(t *testing.T) {
	escalator := NewComplexityAnalyzer(5)
	provider := NewLocalFSProvider(t.TempDir())
	mcp := NewHybridFSMCPWithEscalator(provider, escalator)

	args := map[string]interface{}{"wrong": "argument"}

	_, err := mcp.CallTool(context.Background(), "rag_query", args)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestComplexityAnalyzer(t *testing.T) {
	analyzer := NewComplexityAnalyzer(3)

	if analyzer.AnalyzeComplexity(context.Background(), "one two") {
		t.Errorf("expected false, got true")
	}

	if !analyzer.AnalyzeComplexity(context.Background(), "one two three four") {
		t.Errorf("expected true, got false")
	}
}
