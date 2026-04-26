package registry

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/src/server/tools/localstatefulproxy"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestRegistryAgentE2EFlow(t *testing.T) {
	// 1. Instantiate the Unified Tool Registry
	r := NewUnifiedToolRegistry()

	// 2. Instantiate and Register a Real Tool Refactored for AgentTool
	baseProxy := localstatefulproxy.NewProxyTool()
	agentProxy := &localstatefulproxy.AgentProxyTool{Proxy: baseProxy}

	err := r.RegisterTool(agentProxy)
	require.NoError(t, err)

	// 3. Agent Flow: Discover Tools
	manifests := r.ListTools()
	require.Len(t, manifests, 1)
	assert.Equal(t, "local_stateful_proxy", manifests[0].Name)
	assert.NotEmpty(t, manifests[0].Description)
	assert.NotEmpty(t, manifests[0].InputSchema)

	// 4. Agent Flow: Select and Execute Tool
	_, exists := r.GetTool("local_stateful_proxy")
	require.True(t, exists)

	// Mocking agent creating JSON arguments
	inputArgs := map[string]interface{}{
		"command":    "SELECT * FROM users;",
		"context_id": "tenant-123",
	}
	inputBytes, err := json.Marshal(inputArgs)
	require.NoError(t, err)

	ctx := context.Background()
	resultBytes, err := r.ExecuteTool(ctx, "local_stateful_proxy", inputBytes)
	require.NoError(t, err)

	// Validate execution output
	var result map[string]interface{}
	err = json.Unmarshal(resultBytes, &result)
	require.NoError(t, err)

	assert.Equal(t, "success", result["status"])
	assert.Contains(t, result["message"], "Command 'SELECT * FROM users;' successfully proxied to context 'tenant-123'")
}
