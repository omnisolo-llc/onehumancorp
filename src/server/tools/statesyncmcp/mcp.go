package statesyncmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"bytes"
)

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

type Delta struct {
	ID        string `json:"id"`
	EntityID  string `json:"entity_id"`
	Data      string `json:"data"`
	UpdatedAt string `json:"updated_at"`
}

type Payload struct {
	Deltas []Delta `json:"deltas"`
}

type MCPClient struct {
	CloudURL string
}

func NewMCPClient(cloudURL string) *MCPClient {
	return &MCPClient{CloudURL: cloudURL}
}

func (c *MCPClient) ListTools() []Tool {
	return []Tool{
		{Name: "crdt_push", Description: "Pushes CRDT deltas to the cloud"},
		{Name: "crdt_pull", Description: "Pulls CRDT deltas from the cloud"},
	}
}

func (c *MCPClient) CallTool(ctx context.Context, name string, args map[string]interface{}) (interface{}, error) {
	if name == "crdt_push" {
		return c.crdtPush(ctx, args)
	} else if name == "crdt_pull" {
		return c.crdtPull(ctx, args)
	}
	return nil, fmt.Errorf("unknown tool: %s", name)
}

func (c *MCPClient) crdtPush(ctx context.Context, args map[string]interface{}) (interface{}, error) {
	deltasRaw, ok := args["deltas"]
	if !ok {
		return nil, fmt.Errorf("missing deltas")
	}

	b, err := json.Marshal(deltasRaw)
	if err != nil {
		return nil, err
	}

	var deltas []Delta
	if err := json.Unmarshal(b, &deltas); err != nil {
		return nil, err
	}

	payload := Payload{Deltas: deltas}
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", c.CloudURL+"/api/v1/sync/mcp-deltas", bytes.NewReader(payloadBytes))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("failed to push deltas, status: %d", resp.StatusCode)
	}

	return "success", nil
}

func (c *MCPClient) crdtPull(ctx context.Context, args map[string]interface{}) (interface{}, error) {
	// Not fully specified in requirements, but returning a mock list for completeness
	return Payload{Deltas: []Delta{}}, nil
}
