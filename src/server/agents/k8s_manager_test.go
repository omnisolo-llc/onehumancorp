package agents

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net/http"
	"testing"
)

func TestK8sManager_SpawnAgent(t *testing.T) {
	client := &http.Client{
		Transport: &mockRoundTripper{
			roundTripFunc: func(req *http.Request) (*http.Response, error) {
				if req.URL.Path == "/api/v1/namespaces/default/pods" {
					return &http.Response{
						StatusCode: http.StatusCreated,
						Body:       io.NopCloser(bytes.NewBufferString("")),
					}, nil
				}
				return nil, fmt.Errorf("unexpected request: %s", req.URL.Path)
			},
		},
	}

	manager := &K8sManager{
		client:    client,
		apiServer: "http://localhost:8080",
		namespace: "default",
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
