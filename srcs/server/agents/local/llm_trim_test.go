package local

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestTrimMessageHistory(t *testing.T) {
	// Create alternating user/assistant messages
	messages := make([]ConversationMessage, 15)
	for i := 0; i < 15; i++ {
		if i%2 == 0 {
			messages[i] = ConversationMessage{Role: "user"}
		} else {
			messages[i] = ConversationMessage{Role: "assistant"}
		}
	}

	// 15 messages, we preserve max 10.
	// messages[5] will be the first of the 10.
	// i=5 is an odd number, so role="assistant".
	// The function should drop it and return 9 messages starting with "user".
	trimmed := TrimMessageHistory(messages, 10)

	if len(trimmed) != 9 {
		t.Fatalf("expected 9 messages, got %d", len(trimmed))
	}
	if trimmed[0].Role != "user" {
		t.Fatalf("expected first message to be user, got %s", trimmed[0].Role)
	}
}

func TestLLM_HistoryTrimming_API(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Mock anthropic response
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"content":[{"type":"text","text":"response"}], "usage": {"input_tokens": 10, "output_tokens": 10}}`))
	}))
	defer ts.Close()

	client := NewAnthropicClient("test-key", ts.URL, "test-model")

	req := CompletionRequest{
		Messages: make([]ConversationMessage, 15),
	}
	for i := 0; i < 15; i++ {
		if i%2 == 0 {
			req.Messages[i] = ConversationMessage{Role: "user"}
		} else {
			req.Messages[i] = ConversationMessage{Role: "assistant"}
		}
	}

	_, err := client.Complete(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}
