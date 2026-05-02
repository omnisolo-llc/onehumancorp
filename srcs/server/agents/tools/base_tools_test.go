package tools

import (
	"context"
	"testing"
)

func TestBaseTools(t *testing.T) {
	registry := NewDefaultRegistry()

	t.Run("CalculatorTool", func(t *testing.T) {
		tool, exists := registry.GetTool("local-calculator")
		if !exists {
			t.Fatal("expected local-calculator to be registered")
		}
		if tool.Name() != "local-calculator" {
			t.Fatalf("expected name local-calculator, got %s", tool.Name())
		}

		res, err := tool.Execute(context.Background(), []byte(`{"expression": "1+1"}`))
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if string(res) != "calculated result" {
			t.Fatalf("expected 'calculated result', got %s", string(res))
		}
	})

	t.Run("GrepTool", func(t *testing.T) {
		tool, exists := registry.GetTool("local-grep")
		if !exists {
			t.Fatal("expected local-grep to be registered")
		}
		if tool.Name() != "local-grep" {
			t.Fatalf("expected name local-grep, got %s", tool.Name())
		}

		res, err := tool.Execute(context.Background(), []byte(`{"pattern": "test", "path": "."}`))
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if string(res) != "grep result" {
			t.Fatalf("expected 'grep result', got %s", string(res))
		}
	})
}
