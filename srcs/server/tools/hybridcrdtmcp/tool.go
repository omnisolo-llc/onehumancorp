package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type HybridCRDTMCP struct {
	// dependencies (db layer, etc.) could go here
}

func NewHybridCRDTMCP() *HybridCRDTMCP {
	return &HybridCRDTMCP{}
}

func (m *HybridCRDTMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "crdt_pull",
			Description: "Fetch the latest CRDT state vector for a given entity.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"entity_id": {"type": "string"}}, "required": ["entity_id"]}`),
		},
		{
			Name:        "crdt_push",
			Description: "Submit local CRDT mutations to the Cloud backend.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"entity_id": {"type": "string"}, "mutations": {"type": "object"}}, "required": ["entity_id", "mutations"]}`),
		},
		{
			Name:        "crdt_merge",
			Description: "Locally compute the intersection of state vectors.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"local_vector": {"type": "object"}, "remote_vector": {"type": "object"}}, "required": ["local_vector", "remote_vector"]}`),
		},
	}
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

func (m *HybridCRDTMCP) checkTenant(ctx context.Context) error {
	if envBoolDefault("OHC_MULTITENANT", false) {
		claims := auth.ClaimsFromContext(ctx)
		if claims == nil || claims.OrganizationID == "" {
			return errors.New("unauthorized: missing claims or organization ID")
		}
	}
	return nil
}

func (m *HybridCRDTMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "crdt_pull":
		if err := m.checkTenant(ctx); err != nil {
			return nil, err
		}
		entityID, ok := arguments["entity_id"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'entity_id' argument")
		}
		// mock implementation
		return map[string]interface{}{"entity_id": entityID, "vector": map[string]interface{}{}}, nil

	case "crdt_push":
		if err := m.checkTenant(ctx); err != nil {
			return nil, err
		}
		entityID, ok := arguments["entity_id"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'entity_id' argument")
		}
		mutations, ok := arguments["mutations"].(map[string]interface{})
		if !ok {
			return nil, errors.New("missing or invalid 'mutations' argument")
		}
		// mock implementation
		return map[string]interface{}{"entity_id": entityID, "status": "success", "mutations": mutations}, nil

	case "crdt_merge":
		localVector, ok := arguments["local_vector"].(map[string]interface{})
		if !ok {
			return nil, errors.New("missing or invalid 'local_vector' argument")
		}
		remoteVector, ok := arguments["remote_vector"].(map[string]interface{})
		if !ok {
			return nil, errors.New("missing or invalid 'remote_vector' argument")
		}

		merged := make(map[string]interface{})
		for k, v := range localVector {
			merged[k] = v
		}
		for k, v := range remoteVector {
			merged[k] = v // simple last-writer-wins for mock
		}

		return map[string]interface{}{"merged_vector": merged}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
