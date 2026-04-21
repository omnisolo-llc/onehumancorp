package e2e

// fakeLLMServer starts an HTTP server that mimics the Ollama API, returning
// deterministic responses. This allows E2E tests to exercise the full
// request-response path through the OHC server without depending on an
// external AI API.
//
// Usage: call startFakeLLM() in TestMain before launching the OHC binary and
// pass the returned URL as the OHC_LOCAL_LLM_ENDPOINT environment variable.

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"strings"
	"sync"
)

// fakeLLMState holds global state for the fake LLM server used across tests.
var fakeLLMState struct {
	mu       sync.Mutex
	url      string
	requests []fakeLLMRequest
}

type fakeLLMRequest struct {
	Model    string
	Messages []map[string]string
}

// startFakeLLM launches a lightweight HTTP server that implements the Ollama
// /api/chat and /api/generate endpoints. It returns the base URL of the
// server. The server runs until the process exits.
func startFakeLLM() string {
	mux := http.NewServeMux()

	// Ollama chat endpoint (used by OllamaClient)
	mux.HandleFunc("/api/chat", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req struct {
			Model    string               `json:"model"`
			Messages []map[string]string  `json:"messages"`
			Stream   bool                 `json:"stream"`
		}
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "bad request", http.StatusBadRequest)
			return
		}

		fakeLLMState.mu.Lock()
		fakeLLMState.requests = append(fakeLLMState.requests, fakeLLMRequest{
			Model:    req.Model,
			Messages: req.Messages,
		})
		fakeLLMState.mu.Unlock()

		reply := buildFakeLLMReply(req.Messages)

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"model": req.Model,
			"message": map[string]string{
				"role":    "assistant",
				"content": reply,
			},
			"done":              true,
			"done_reason":       "stop",
			"prompt_eval_count": 10,
			"eval_count":        20,
		})
	})

	// Ollama generate endpoint (used by local provider fallback)
	mux.HandleFunc("/api/generate", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req struct {
			Model  string `json:"model"`
			Prompt string `json:"prompt"`
			Stream bool   `json:"stream"`
		}
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "bad request", http.StatusBadRequest)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"model":    req.Model,
			"response": "Task acknowledged. I will handle this.",
			"done":     true,
		})
	})

	// Ollama embeddings endpoint
	mux.HandleFunc("/api/embeddings", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		// Return a minimal embedding vector
		embedding := make([]float64, 128)
		for i := range embedding {
			embedding[i] = 0.01
		}
		json.NewEncoder(w).Encode(map[string]interface{}{
			"embedding": embedding,
		})
	})

	// Health check
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{"status": "ok"})
	})

	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		panic(fmt.Sprintf("fake LLM: listen: %v", err))
	}

	srv := &http.Server{Handler: mux}
	go func() {
		_ = srv.Serve(ln)
	}()

	addr := ln.Addr().String()
	fakeLLMState.url = "http://" + addr
	return fakeLLMState.url
}

// buildFakeLLMReply generates a deterministic reply based on the content of
// the last user message. This allows tests to verify that the agent processed
// the input and produced a relevant output without relying on a real LLM.
func buildFakeLLMReply(messages []map[string]string) string {
	lastUserContent := ""
	for _, m := range messages {
		if role, ok := m["role"]; ok && role == "user" {
			if content, ok := m["content"]; ok {
				lastUserContent = content
			}
		}
	}

	lower := strings.ToLower(lastUserContent)
	switch {
	case strings.Contains(lower, "task") || strings.Contains(lower, "do") || strings.Contains(lower, "create"):
		return "I will work on this task right away. Starting execution now."
	case strings.Contains(lower, "status") || strings.Contains(lower, "update"):
		return "Current status: in progress. All systems operational."
	case strings.Contains(lower, "help") || strings.Contains(lower, "?"):
		return "I can help you with tasks, status updates, and agent coordination."
	default:
		return "Understood. I will process your request and take action."
	}
}

// fakeLLMRequestCount returns the number of requests received by the fake LLM
// server since the last reset. Useful for assertions in tests.
func fakeLLMRequestCount() int {
	fakeLLMState.mu.Lock()
	defer fakeLLMState.mu.Unlock()
	return len(fakeLLMState.requests)
}

// fakeLLMResetRequests clears the recorded requests.
func fakeLLMResetRequests() {
	fakeLLMState.mu.Lock()
	defer fakeLLMState.mu.Unlock()
	fakeLLMState.requests = nil
}
