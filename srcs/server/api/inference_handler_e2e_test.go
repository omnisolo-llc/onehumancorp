package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/integrations/mcp_inference_router"
)

func mockServer(response string, statusCode int) *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(statusCode)
		resp := mcp_inference_router.InferenceResponse{Result: response}
		json.NewEncoder(w).Encode(resp)
	}))
}

func TestInferenceHandler_E2E(t *testing.T) {
	localServer := mockServer("local E2E result", http.StatusOK)
	defer localServer.Close()

	cloudServer := mockServer("cloud E2E result", http.StatusOK)
	defer cloudServer.Close()

	router := mcp_inference_router.NewInferenceRouter(localServer.URL, cloudServer.URL)
	handler := NewInferenceHandler(router)

	mux := http.NewServeMux()
	mux.Handle("/api/v1/inference/route", handler)
	server := httptest.NewServer(mux)
	defer server.Close()

	reqPayload := mcp_inference_router.InferenceRequest{
		Prompt:      "test prompt",
		TokenCount:  150,
		IsSensitive: true, // Should force local
	}
	bodyBytes, _ := json.Marshal(reqPayload)

	resp, err := http.Post(server.URL+"/api/v1/inference/route", "application/json", bytes.NewBuffer(bodyBytes))
	if err != nil {
		t.Fatalf("Failed to make request: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Expected status 200, got %d", resp.StatusCode)
	}

	var infResp mcp_inference_router.InferenceResponse
	if err := json.NewDecoder(resp.Body).Decode(&infResp); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}

	if infResp.Source != "Local" {
		t.Errorf("Expected source Local, got %s", infResp.Source)
	}
}
