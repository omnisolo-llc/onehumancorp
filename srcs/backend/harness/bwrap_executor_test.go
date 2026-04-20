package harness

import (
	"context"
	"strings"
	"testing"
)

func TestBwrapExecutor_Execute(t *testing.T) {
	executor := NewBwrapExecutor()
	ctx := context.Background()
	_, err := executor.Execute(ctx, "echo", "hello")
	if err != nil {
		if !strings.Contains(err.Error(), "executable file not found") && !strings.Contains(err.Error(), "exit status") {
			t.Errorf("Unexpected error: %v", err)
		}
	}
}
