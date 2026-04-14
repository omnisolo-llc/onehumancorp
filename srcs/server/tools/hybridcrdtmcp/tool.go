package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type HybridCRDTMCP struct {
	provider db.Provider
}

func NewHybridCRDTMCP(provider db.Provider) *HybridCRDTMCP {
	return &HybridCRDTMCP{provider: provider}
}

func envBoolDefault(key string, def bool) bool {
	if val := os.Getenv(key); val != "" {
		return val == "true"
	}
	return def
}

func (m *HybridCRDTMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "crdt_pull",
			Description: "Fetch the latest CRDT state vector for a given entity from the Cloud backend.",
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

func (m *HybridCRDTMCP) crdtPull(ctx context.Context, entityID string) (interface{}, error) {
	var vectorStr string
	var err error
	if envBoolDefault("OHC_MULTITENANT", false) {
		orgID := auth.OrganizationIDFromContext(ctx)
		row := m.provider.QueryRow(ctx, "SELECT crdt_vector FROM shared_tasks WHERE id = $1 AND organization_id = $2", entityID, orgID)
		err = row.Scan(&vectorStr)
	} else {
		row := m.provider.QueryRow(ctx, "SELECT crdt_vector FROM shared_tasks WHERE id = ?", entityID)
		err = row.Scan(&vectorStr)
	}
	if err != nil {
		return nil, fmt.Errorf("failed to fetch CRDT vector: %v", err)
	}

	var vector map[string]interface{}
	if vectorStr != "" {
		if err := json.Unmarshal([]byte(vectorStr), &vector); err != nil {
			return nil, fmt.Errorf("failed to parse CRDT vector: %v", err)
		}
	} else {
		vector = map[string]interface{}{"clock": 1}
	}
	return map[string]interface{}{"status": "success", "vector": vector}, nil
}

func (m *HybridCRDTMCP) crdtPush(ctx context.Context, entityID string, mutations map[string]interface{}) (interface{}, error) {
	mutationsBytes, err := json.Marshal(mutations)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal mutations: %v", err)
	}

	var affected int64
	if envBoolDefault("OHC_MULTITENANT", false) {
		orgID := auth.OrganizationIDFromContext(ctx)
		affected, err = m.provider.Exec(ctx, "UPDATE shared_tasks SET crdt_vector = $1 WHERE id = $2 AND organization_id = $3", string(mutationsBytes), entityID, orgID)
	} else {
		affected, err = m.provider.Exec(ctx, "UPDATE shared_tasks SET crdt_vector = ? WHERE id = ?", string(mutationsBytes), entityID)
	}
	if err != nil {
		return nil, fmt.Errorf("failed to push CRDT vector: %v", err)
	}

	if affected == 0 {
		return nil, errors.New("no rows affected, entity not found")
	}

	return map[string]interface{}{"status": "success", "message": "mutations pushed"}, nil
}

func (m *HybridCRDTMCP) crdtMerge(ctx context.Context, localVector map[string]interface{}, remoteVector map[string]interface{}) (interface{}, error) {
	localClock, localOk := localVector["clock"].(float64)
	remoteClock, remoteOk := remoteVector["clock"].(float64)

	mergedClock := 1.0
	if localOk && remoteOk {
		if localClock > remoteClock {
			mergedClock = localClock
		} else {
			mergedClock = remoteClock
		}
	} else if localOk {
		mergedClock = localClock
	} else if remoteOk {
		mergedClock = remoteClock
	}

	return map[string]interface{}{"status": "success", "merged_vector": map[string]interface{}{"clock": mergedClock + 1}}, nil
}

func (m *HybridCRDTMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if envBoolDefault("OHC_MULTITENANT", false) {
		orgID := auth.OrganizationIDFromContext(ctx)
		if orgID == "" {
			return nil, errors.New("unauthorized: missing organization ID in multi-tenant mode")
		}
	}

	switch toolName {
	case "crdt_pull":
		entityID, ok := arguments["entity_id"].(string)
		if !ok {
			return nil, errors.New("invalid or missing entity_id")
		}
		return m.crdtPull(ctx, entityID)
	case "crdt_push":
		entityID, ok := arguments["entity_id"].(string)
		if !ok {
			return nil, errors.New("invalid or missing entity_id")
		}
		mutations, ok := arguments["mutations"].(map[string]interface{})
		if !ok {
			return nil, errors.New("invalid or missing mutations")
		}
		return m.crdtPush(ctx, entityID, mutations)
	case "crdt_merge":
		localVector, ok := arguments["local_vector"].(map[string]interface{})
		if !ok {
			return nil, errors.New("invalid or missing local_vector")
		}
		remoteVector, ok := arguments["remote_vector"].(map[string]interface{})
		if !ok {
			return nil, errors.New("invalid or missing remote_vector")
		}
		return m.crdtMerge(ctx, localVector, remoteVector)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
