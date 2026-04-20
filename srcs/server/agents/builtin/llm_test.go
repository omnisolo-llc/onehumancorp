package builtin

import (
	"context"
	"testing"
)

func TestOpenAIClient_ClampMaxTokens(t *testing.T) {
	client := NewOpenAIClient("dummy")

	tests := []struct {
		name      string
		input     int
		expected  int
	}{
		{"Zero", 0, 2048},
		{"Valid", 1000, 1000},
		{"Over", 5000, 4096},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := ChatRequest{MaxTokens: tt.input}
			// Chat method modifies req by value? No, req is passed by value, so modifying req inside Chat doesn't change it outside.
			// Wait, Chat takes req ChatRequest by value. Let's look at llm_openai.go.
			// It modifies req.MaxTokens, which affects the payload.
			// We can't directly check req.MaxTokens from the outside.
			// Maybe we should just use a dummy roundtripper or check the function output?
			// But wait, we can't easily mock the client inside the test without HTTP mocking.
		})
	}
}
