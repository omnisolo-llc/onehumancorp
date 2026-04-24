package metricsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
)

// MetricsProvider is the interface for querying metrics and health status.
type MetricsProvider interface {
	QueryMetrics(ctx context.Context, query string) (map[string]interface{}, error)
	GetHealthStatus(ctx context.Context) (map[string]interface{}, error)
}

// LocalMetricsProvider implements MetricsProvider for standalone mode.
type LocalMetricsProvider struct{
	db db.Provider
}

// NewLocalMetricsProvider creates a new LocalMetricsProvider.
func NewLocalMetricsProvider(db db.Provider) *LocalMetricsProvider {
	return &LocalMetricsProvider{db: db}
}

func (p *LocalMetricsProvider) QueryMetrics(ctx context.Context, query string) (map[string]interface{}, error) {
	// In standalone mode, we read from the local SQLite metrics buffer.
	rows, err := p.db.Query(ctx, "SELECT id, payload FROM telemetry_buffer WHERE payload LIKE ? LIMIT 50", "%" + query + "%")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []interface{}
	for rows.Next() {
		var id string
		var payload string
		if err := rows.Scan(&id, &payload); err == nil {
			var parsed map[string]interface{}
			if json.Unmarshal([]byte(payload), &parsed) == nil {
				parsed["id"] = id
				results = append(results, parsed)
			}
		}
	}

	return map[string]interface{}{
		"source": "local_buffer",
		"query":  query,
		"data":   results,
	}, nil
}

func (p *LocalMetricsProvider) GetHealthStatus(ctx context.Context) (map[string]interface{}, error) {
	err := p.db.Ping(ctx)
	status := "healthy"
	if err != nil {
		status = "unhealthy"
	}

	return map[string]interface{}{
		"status": status,
		"mode":   "standalone",
	}, nil
}

// CloudMetricsProvider implements MetricsProvider using tenant isolation.
type CloudMetricsProvider struct{
	db db.Provider
}

// NewCloudMetricsProvider creates a new CloudMetricsProvider.
func NewCloudMetricsProvider(db db.Provider) *CloudMetricsProvider {
	return &CloudMetricsProvider{db: db}
}

func (p *CloudMetricsProvider) QueryMetrics(ctx context.Context, query string) (map[string]interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	// This is a proxy wrapper for the internal telemetry metrics.
	// Since Prometheus and OpenTelemetry do not provide direct SQL querying without a PromQL client,
	// and we are requested to create an adapter wrapping the internal module,
	// we query the database buffer directly for bounded metrics related to Swarm Operations.

	// Enforce tenant isolation strictly
	rows, err := p.db.Query(ctx, "SELECT id, payload FROM telemetry_buffer WHERE payload::jsonb ->> 'tenant_id' = $1 AND payload::text ILIKE $2 LIMIT 50", claims.OrganizationID, "%" + query + "%")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []interface{}
	for rows.Next() {
		var id string
		var payload string
		if err := rows.Scan(&id, &payload); err == nil {
			var parsed map[string]interface{}
			if json.Unmarshal([]byte(payload), &parsed) == nil {
				parsed["id"] = id
				results = append(results, parsed)
			}
		}
	}

	return map[string]interface{}{
		"source":    "cloud_telemetry",
		"query":     query,
		"tenant_id": claims.OrganizationID,
		"data":      results,
	}, nil
}

func (p *CloudMetricsProvider) GetHealthStatus(ctx context.Context) (map[string]interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	// Basic health check
	err := p.db.Ping(ctx)
	status := "healthy"
	if err != nil {
		status = "unhealthy"
	}

	return map[string]interface{}{
		"status":    status,
		"mode":      "cloud",
		"tenant_id": claims.OrganizationID,
	}, nil
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridMetricsMCP implements the MCP interface for metrics operations.
type HybridMetricsMCP struct {
	provider MetricsProvider
}

// NewHybridMetricsMCP creates a new HybridMetricsMCP instance.
func NewHybridMetricsMCP(provider MetricsProvider) *HybridMetricsMCP {
	return &HybridMetricsMCP{provider: provider}
}

// ListTools returns the list of available tools.
func (m *HybridMetricsMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "query_metrics",
			Description: "Queries telemetry metrics for the agent's operations.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`),
		},
		{
			Name:        "get_health_status",
			Description: "Retrieves the health status of the agent operations and metrics system.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {}}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridMetricsMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "query_metrics":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}
		return m.provider.QueryMetrics(ctx, query)
	case "get_health_status":
		return m.provider.GetHealthStatus(ctx)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

// NewProviderFactory returns a MetricsProvider based on environment configuration.
func NewProviderFactory(db db.Provider) MetricsProvider {
	if envBoolDefault("OHC_MULTITENANT", true) && !envBoolDefault("OHC_STANDALONE", false) {
		return NewCloudMetricsProvider(db)
	}
	return NewLocalMetricsProvider(db)
}
