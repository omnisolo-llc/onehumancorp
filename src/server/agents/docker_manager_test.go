package agents

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net/http"
	"testing"
)

type mockRoundTripper struct {
	roundTripFunc func(req *http.Request) (*http.Response, error)
}

func (m *mockRoundTripper) RoundTrip(req *http.Request) (*http.Response, error) {
	return m.roundTripFunc(req)
}

func TestDockerManager_SpawnAgent(t *testing.T) {
	mockResp := `{
		"Id": "test-container-id"
	}`

	client := &http.Client{
		Transport: &mockRoundTripper{
			roundTripFunc: func(req *http.Request) (*http.Response, error) {
				if req.URL.Path == "/v1.41/containers/create" {
					return &http.Response{
						StatusCode: http.StatusCreated,
						Body:       io.NopCloser(bytes.NewBufferString(mockResp)),
					}, nil
				}
				if req.URL.Path == "/v1.41/containers/test-container-id/start" {
					return &http.Response{
						StatusCode: http.StatusNoContent,
						Body:       io.NopCloser(bytes.NewBufferString("")),
					}, nil
				}
				return nil, fmt.Errorf("unexpected request: %s", req.URL.Path)
			},
		},
	}

	manager := &DockerManager{
		client: client,
	}

	agent := Agent{
		ID:   "agent-1",
		Role: "admin",
	}

	err := manager.SpawnAgent(context.Background(), agent, "")
	if err != nil {
		t.Fatalf("SpawnAgent failed: %v", err)
	}
}
