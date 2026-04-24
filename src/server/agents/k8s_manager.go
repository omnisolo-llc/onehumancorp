package agents

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
)

// K8sManager implements AgentManager using the Kubernetes HTTP API.
type K8sManager struct {
	client    *http.Client
	apiServer string
	namespace string
}

// NewK8sManager creates a new K8sManager.
func NewK8sManager(apiServer, namespace string) *K8sManager {
	return &K8sManager{
		client:    http.DefaultClient, // Placeholder, will inject token in requests
		apiServer: apiServer,
		namespace: namespace,
	}
}

func (m *K8sManager) getAuthToken() string {
	tokenBytes, err := os.ReadFile("/var/run/secrets/kubernetes.io/serviceaccount/token")
	if err != nil {
		return ""
	}
	return string(tokenBytes)
}

// SpawnAgent starts a new agent instance as a K8s Pod.
func (m *K8sManager) SpawnAgent(ctx context.Context, agent Agent, config string) error {
	image := "ohc-builtin-agent:latest"

	pod := map[string]interface{}{
		"apiVersion": "v1",
		"kind":       "Pod",
		"metadata": map[string]interface{}{
			"name": fmt.Sprintf("agent-%s", agent.ID),
		},
		"spec": map[string]interface{}{
			"containers": []map[string]interface{}{
				{
					"name":  "agent",
					"image": image,
					"env": []map[string]string{
						{"name": "OHC_AGENT_ID", "value": agent.ID},
						{"name": "OHC_AGENT_ROLE", "value": agent.Role},
						{"name": "OHC_MESSAGE_BUS_URL", "value": "nats://nats-service:4222"}, // Assuming NATS service in K8s
					},
				},
			},
			"restartPolicy": "Never",
		},
	}

	reqBody, err := json.Marshal(pod)
	if err != nil {
		return fmt.Errorf("failed to marshal pod spec: %w", err)
	}

	url := fmt.Sprintf("%s/api/v1/namespaces/%s/pods", m.apiServer, m.namespace)
	req, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewBuffer(reqBody))
	if err != nil {
		return fmt.Errorf("failed to create HTTP request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	if token := m.getAuthToken(); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := m.client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send create pod request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusCreated {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("failed to create pod, status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}

// TerminateAgent stops and removes a running agent pod.
func (m *K8sManager) TerminateAgent(ctx context.Context, agentID string) error {
	url := fmt.Sprintf("%s/api/v1/namespaces/%s/pods/agent-%s", m.apiServer, m.namespace, agentID)
	req, err := http.NewRequestWithContext(ctx, "DELETE", url, nil)
	if err != nil {
		return fmt.Errorf("failed to create delete pod request: %w", err)
	}

	if token := m.getAuthToken(); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := m.client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send delete pod request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusAccepted {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("failed to delete pod, status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}

// GetAgentStatus retrieves the current status of an agent pod.
func (m *K8sManager) GetAgentStatus(ctx context.Context, agentID string) (Status, error) {
	url := fmt.Sprintf("%s/api/v1/namespaces/%s/pods/agent-%s/status", m.apiServer, m.namespace, agentID)
	req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
	if err != nil {
		return StatusIdle, fmt.Errorf("failed to create get pod status request: %w", err)
	}

	if token := m.getAuthToken(); token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}

	resp, err := m.client.Do(req)
	if err != nil {
		return StatusIdle, fmt.Errorf("failed to send get pod status request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return StatusIdle, fmt.Errorf("failed to get pod status, status %d: %s", resp.StatusCode, string(body))
	}

	var podResp struct {
		Status struct {
			Phase string `json:"phase"`
		} `json:"status"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&podResp); err != nil {
		return StatusIdle, fmt.Errorf("failed to decode pod status response: %w", err)
	}

	switch podResp.Status.Phase {
	case "Running":
		return StatusActive, nil
	case "Pending":
		return StatusIdle, nil
	default:
		return StatusBlocked, nil
	}
}
