package hybridcrdtmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         = otel.Meter("hybrid_crdt_mcp")
	syncCount     metric.Int64Counter
	syncLatency   metric.Float64Histogram
	syncErrorCount metric.Int64Counter
)

func init() {
	var err error
	syncCount, err = meter.Int64Counter("crdt_sync_total", metric.WithDescription("Total number of CRDT sync operations"))
	if err != nil {
		panic(err)
	}
	syncLatency, err = meter.Float64Histogram("crdt_sync_latency_ms", metric.WithDescription("Latency of CRDT sync operations"))
	if err != nil {
		panic(err)
	}
	syncErrorCount, err = meter.Int64Counter("crdt_sync_errors_total", metric.WithDescription("Total number of CRDT sync errors"))
	if err != nil {
		panic(err)
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// CRDTProvider abstracts the CRDT state sync logic.
type CRDTProvider interface {
	Pull(ctx context.Context, claims *auth.Claims, entityID string) (json.RawMessage, error)
	Push(ctx context.Context, claims *auth.Claims, entityID string, stateVector json.RawMessage) error
	Merge(ctx context.Context, localVector json.RawMessage, remoteVector json.RawMessage) (json.RawMessage, error)
}

// HybridCRDTMCP implements the MCP interface for CRDT state sync.
type HybridCRDTMCP struct {
	provider     CRDTProvider
	isMultiTenant bool
}

// NewHybridCRDTMCP creates a new HybridCRDTMCP instance.
func NewHybridCRDTMCP(provider CRDTProvider, isMultiTenant bool) *HybridCRDTMCP {
	return &HybridCRDTMCP{
		provider:     provider,
		isMultiTenant: isMultiTenant,
	}
}

// ListTools returns the list of available tools.
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
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"entity_id": {"type": "string"}, "state_vector": {"type": "object"}}, "required": ["entity_id", "state_vector"]}`),
		},
		{
			Name:        "crdt_merge",
			Description: "Locally compute the intersection of state vectors.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"local_vector": {"type": "object"}, "remote_vector": {"type": "object"}}, "required": ["local_vector", "remote_vector"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridCRDTMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	start := time.Now()

	claims := auth.ClaimsFromContext(ctx)
	if m.isMultiTenant {
		if claims == nil || claims.OrganizationID == "" {
			return nil, errors.New("unauthorized: missing claims or organization ID in multi-tenant mode")
		}
	} else {
		// In standalone, just ensure we have some claims context if needed, but relaxed auth
		if claims == nil {
			claims = &auth.Claims{}
		}
	}

	attrs := metric.WithAttributes(
		attribute.String("tool", toolName),
		attribute.Bool("multi_tenant", m.isMultiTenant),
	)

	var res interface{}
	var err error

	defer func() {
		latency := float64(time.Since(start).Milliseconds())
		syncLatency.Record(ctx, latency, attrs)
		syncCount.Add(ctx, 1, attrs)

		if err != nil {
			syncErrorCount.Add(ctx, 1, attrs)
		}
	}()

	switch toolName {
	case "crdt_pull":
		entityID, ok := arguments["entity_id"].(string)
		if !ok || entityID == "" {
			err = errors.New("missing or invalid 'entity_id' argument")
			return nil, err
		}
		vector, pullErr := m.provider.Pull(ctx, claims, entityID)
		if pullErr != nil {
			err = pullErr
			return nil, err
		}
		res = map[string]interface{}{"state_vector": vector}
		return res, nil

	case "crdt_push":
		entityID, ok := arguments["entity_id"].(string)
		if !ok || entityID == "" {
			err = errors.New("missing or invalid 'entity_id' argument")
			return nil, err
		}

		// Handle json.RawMessage
		var stateVector json.RawMessage
		switch v := arguments["state_vector"].(type) {
		case string:
			stateVector = json.RawMessage(v)
		case map[string]interface{}:
			b, parseErr := json.Marshal(v)
			if parseErr != nil {
				err = parseErr
				return nil, err
			}
			stateVector = json.RawMessage(b)
		default:
			err = errors.New("invalid 'state_vector' argument type")
			return nil, err
		}

		pushErr := m.provider.Push(ctx, claims, entityID, stateVector)
		if pushErr != nil {
			err = pushErr
			return nil, err
		}
		res = map[string]interface{}{"status": "success"}
		return res, nil

	case "crdt_merge":
		var localVector, remoteVector json.RawMessage

		switch v := arguments["local_vector"].(type) {
		case string:
			localVector = json.RawMessage(v)
		case map[string]interface{}:
			b, parseErr := json.Marshal(v)
			if parseErr != nil {
				err = parseErr
				return nil, err
			}
			localVector = json.RawMessage(b)
		default:
			err = errors.New("invalid 'local_vector' argument type")
			return nil, err
		}

		switch v := arguments["remote_vector"].(type) {
		case string:
			remoteVector = json.RawMessage(v)
		case map[string]interface{}:
			b, parseErr := json.Marshal(v)
			if parseErr != nil {
				err = parseErr
				return nil, err
			}
			remoteVector = json.RawMessage(b)
		default:
			err = errors.New("invalid 'remote_vector' argument type")
			return nil, err
		}

		merged, mergeErr := m.provider.Merge(ctx, localVector, remoteVector)
		if mergeErr != nil {
			err = mergeErr
			return nil, err
		}
		res = map[string]interface{}{"merged_vector": merged}
		return res, nil

	default:
		err = fmt.Errorf("unknown tool: %s", toolName)
		return nil, err
	}
}
