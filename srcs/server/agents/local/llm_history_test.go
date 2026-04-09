package local

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAnthropicClient_HistoryTruncation(t *testing.T) {
	var capturedBody anthropicRequest
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		json.NewDecoder(r.Body).Decode(&capturedBody)
		resp := anthropicResponse{
			Content: []anthropicContent{{Type: "text", Text: "Hello"}},
			StopReason: "end_turn",
		}
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	client := NewAnthropicClient("test-key", "claude-3-5-sonnet", server.URL)

	var msgs []ConversationMessage
	for i := 0; i < 25; i++ {
		role := "user"
		if i%2 != 0 {
			role = "assistant"
		}
		msgs = append(msgs, ConversationMessage{
			Role: role,
			Content: []ContentPart{{Type: "text", Text: "test"}},
		})
	}

	// 25 messages, ends with user message at index 24 (since 0 is user, even is user).
	// Truncate to 20: index 5 to 24.
	// Index 5 is assistant. It should be removed, leaving 19 messages starting from index 6 (user).
	// Let's verify.
	_, err := client.Complete(context.Background(), CompletionRequest{
		Messages: msgs,
	})
	assert.NoError(t, err)

	assert.Equal(t, 19, len(capturedBody.Messages))
	assert.Equal(t, "user", capturedBody.Messages[0].Role)
}

func TestOpenAIClient_HistoryTruncation(t *testing.T) {
	var capturedBody openAIRequest
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		json.NewDecoder(r.Body).Decode(&capturedBody)

		// Create a valid dummy response
		var resp openAIResponse
		resp.Choices = append(resp.Choices, struct{
			Message      openAIMessage `json:"message"`
			FinishReason string        `json:"finish_reason"`
		}{
			Message: openAIMessage{Role: "assistant", Content: "Hello"},
			FinishReason: "stop",
		})
		json.NewEncoder(w).Encode(resp)
	}))
	defer server.Close()

	client := NewOpenAICompatClient(server.URL, "test-key", "gpt-4o")

	var msgs []ConversationMessage
	for i := 0; i < 25; i++ {
		role := "user"
		if i%2 != 0 {
			role = "assistant"
		}
		msgs = append(msgs, ConversationMessage{
			Role: role,
			Content: []ContentPart{{Type: "text", Text: "test"}},
		})
	}

	_, err := client.Complete(context.Background(), CompletionRequest{
		Messages: msgs,
	})
	assert.NoError(t, err)

	assert.Equal(t, 19, len(capturedBody.Messages))
	assert.Equal(t, "user", capturedBody.Messages[0].Role)
}
