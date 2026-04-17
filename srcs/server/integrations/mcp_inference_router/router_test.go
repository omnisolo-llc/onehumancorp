package mcp_inference_router

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func mockServer(response string, statusCode int) *httptest.Server {
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(statusCode)
		resp := InferenceResponse{Result: response}
		json.NewEncoder(w).Encode(resp)
	}))
}

func TestRouteInference_SensitiveForcedLocal(t *testing.T) {
	localServer := mockServer("local result", http.StatusOK)
	defer localServer.Close()

	cloudServer := mockServer("cloud result", http.StatusOK)
	defer cloudServer.Close()

	router := NewInferenceRouter(localServer.URL, cloudServer.URL)

	req := InferenceRequest{
		Prompt:      "secret data",
		TokenCount:  2000, // Normally large enough for cloud
		IsSensitive: true, // Forces local
	}

	resp, err := router.RouteInference(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp.Source != "Local" {
		t.Errorf("expected source Local, got %s", resp.Source)
	}
}

func TestRouteInference_LargePromptOffloadsToCloud(t *testing.T) {
	localServer := mockServer("local result", http.StatusOK)
	defer localServer.Close()

	cloudServer := mockServer("cloud result", http.StatusOK)
	defer cloudServer.Close()

	router := NewInferenceRouter(localServer.URL, cloudServer.URL)

	req := InferenceRequest{
		Prompt:      "very large data",
		TokenCount:  1500, // Triggers cloud offload
		IsSensitive: false,
	}

	resp, err := router.RouteInference(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp.Source != "Cloud" {
		t.Errorf("expected source Cloud, got %s", resp.Source)
	}
}

func TestRouteInference_CloudFailsFallbackToLocal(t *testing.T) {
	localServer := mockServer("local result", http.StatusOK)
	defer localServer.Close()

	cloudServer := mockServer("cloud error", http.StatusInternalServerError)
	defer cloudServer.Close()

	router := NewInferenceRouter(localServer.URL, cloudServer.URL)

	req := InferenceRequest{
		Prompt:      "large data",
		TokenCount:  1500,
		IsSensitive: false,
	}

	resp, err := router.RouteInference(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp.Source != "Local" {
		t.Errorf("expected fallback to Local, got %s", resp.Source)
	}
}

func TestRouteInference_SmallPromptLocalExecution(t *testing.T) {
	localServer := mockServer("local result", http.StatusOK)
	defer localServer.Close()

	cloudServer := mockServer("cloud result", http.StatusOK)
	defer cloudServer.Close()

	router := NewInferenceRouter(localServer.URL, cloudServer.URL)

	req := InferenceRequest{
		Prompt:      "small data",
		TokenCount:  100,
		IsSensitive: false,
	}

	resp, err := router.RouteInference(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp.Source != "Local" {
		t.Errorf("expected source Local, got %s", resp.Source)
	}
}

func TestRouteInference_LocalFailsFallbackToCloud(t *testing.T) {
	localServer := mockServer("local error", http.StatusInternalServerError)
	defer localServer.Close()

	cloudServer := mockServer("cloud result", http.StatusOK)
	defer cloudServer.Close()

	router := NewInferenceRouter(localServer.URL, cloudServer.URL)

	req := InferenceRequest{
		Prompt:      "small data",
		TokenCount:  100,
		IsSensitive: false,
	}

	resp, err := router.RouteInference(context.Background(), req)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if resp.Source != "Cloud" {
		t.Errorf("expected fallback to Cloud, got %s", resp.Source)
	}
}

func TestRouteInference_BothFail(t *testing.T) {
	localServer := mockServer("local error", http.StatusInternalServerError)
	defer localServer.Close()

	cloudServer := mockServer("cloud error", http.StatusInternalServerError)
	defer cloudServer.Close()

	router := NewInferenceRouter(localServer.URL, cloudServer.URL)

	req := InferenceRequest{
		Prompt:      "data",
		TokenCount:  100,
		IsSensitive: false,
	}

	_, err := router.RouteInference(context.Background(), req)
	if err == nil {
		t.Fatalf("expected error when both fail, got none")
	}
}