package agents

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
)

// DockerManager implements AgentManager using the Docker HTTP API over a Unix socket.
type DockerManager struct {
	socketPath string
	client     *http.Client
}

// NewDockerManager creates a new DockerManager.
func NewDockerManager(socketPath string) *DockerManager {
	// Configure HTTP client to use Unix socket
	transport := &http.Transport{
		DialContext: func(ctx context.Context, _, _ string) (net.Conn, error) {
			return net.Dial("unix", socketPath)
		},
	}
	client := &http.Client{
		Transport: transport,
	}

	return &DockerManager{
		socketPath: socketPath,
		client:     client,
	}
}

// SpawnAgent starts a new agent instance in a Docker container.
func (m *DockerManager) SpawnAgent(ctx context.Context, agent Agent, config string) error {
	// Prepare container creation request
	// We use a simple mapping for now. In reality, we might parse 'config' JSON.
	image := "ohc-builtin-agent:latest"

	createReq := map[string]interface{}{
		"Image": image,
		"Env": []string{
			fmt.Sprintf("OHC_AGENT_ID=%s", agent.ID),
			fmt.Sprintf("OHC_AGENT_ROLE=%s", agent.Role),
			// We need to pass the message bus URL. Assuming it's in env or default.
			fmt.Sprintf("OHC_MESSAGE_BUS_URL=%s", "nats://127.0.0.1:4222"), // Hardcoded default for now
		},
	}

	reqBody, err := json.Marshal(createReq)
	if err != nil {
		return fmt.Errorf("failed to marshal create request: %w", err)
	}

	// 1. Create container
	url := "http://localhost/v1.41/containers/create"
	req, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewBuffer(reqBody))
	if err != nil {
		return fmt.Errorf("failed to create HTTP request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := m.client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send create request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusCreated {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("failed to create container, status %d: %s", resp.StatusCode, string(body))
	}

	var createResp struct {
		Id string `json:"Id"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&createResp); err != nil {
		return fmt.Errorf("failed to decode create response: %w", err)
	}

	// 2. Start container
	startUrl := fmt.Sprintf("http://localhost/v1.41/containers/%s/start", createResp.Id)
	startReq, err := http.NewRequestWithContext(ctx, "POST", startUrl, nil)
	if err != nil {
		return fmt.Errorf("failed to create start request: %w", err)
	}

	startResp, err := m.client.Do(startReq)
	if err != nil {
		return fmt.Errorf("failed to send start request: %w", err)
	}
	defer startResp.Body.Close()

	if startResp.StatusCode != http.StatusNoContent {
		body, _ := io.ReadAll(startResp.Body)
		return fmt.Errorf("failed to start container, status %d: %s", startResp.StatusCode, string(body))
	}

	return nil
}

// TerminateAgent stops and removes a running agent container.
func (m *DockerManager) TerminateAgent(ctx context.Context, agentID string) error {
	// 1. Stop container
	stopUrl := fmt.Sprintf("http://localhost/v1.41/containers/%s/stop", agentID)
	req, err := http.NewRequestWithContext(ctx, "POST", stopUrl, nil)
	if err != nil {
		return fmt.Errorf("failed to create stop request: %w", err)
	}

	resp, err := m.client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send stop request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusNoContent && resp.StatusCode != http.StatusNotModified {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("failed to stop container, status %d: %s", resp.StatusCode, string(body))
	}

	// 2. Remove container
	removeUrl := fmt.Sprintf("http://localhost/v1.41/containers/%s", agentID)
	removeReq, err := http.NewRequestWithContext(ctx, "DELETE", removeUrl, nil)
	if err != nil {
		return fmt.Errorf("failed to create remove request: %w", err)
	}

	removeResp, err := m.client.Do(removeReq)
	if err != nil {
		return fmt.Errorf("failed to send remove request: %w", err)
	}
	defer removeResp.Body.Close()

	if removeResp.StatusCode != http.StatusNoContent {
		body, _ := io.ReadAll(removeResp.Body)
		return fmt.Errorf("failed to remove container, status %d: %s", removeResp.StatusCode, string(body))
	}

	return nil
}

// GetAgentStatus retrieves the current status of an agent container.
func (m *DockerManager) GetAgentStatus(ctx context.Context, agentID string) (Status, error) {
	url := fmt.Sprintf("http://localhost/v1.41/containers/%s/json", agentID)
	req, err := http.NewRequestWithContext(ctx, "GET", url, nil)
	if err != nil {
		return StatusIdle, fmt.Errorf("failed to create status request: %w", err)
	}

	resp, err := m.client.Do(req)
	if err != nil {
		return StatusIdle, fmt.Errorf("failed to send status request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return StatusIdle, fmt.Errorf("failed to get container status, status %d: %s", resp.StatusCode, string(body))
	}

	var inspectResp struct {
		State struct {
			Running bool   `json:"Running"`
			Status  string `json:"Status"`
		} `json:"State"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&inspectResp); err != nil {
		return StatusIdle, fmt.Errorf("failed to decode status response: %w", err)
	}

	if inspectResp.State.Running {
		return StatusActive, nil
	}
	return StatusIdle, nil
}
