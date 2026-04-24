package registry

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type MockTool struct {
	name        string
	description string
	schema      string
}

func (m *MockTool) Name() string { return m.name }
func (m *MockTool) Description() string { return m.description }
func (m *MockTool) InputSchema() json.RawMessage { return json.RawMessage(m.schema) }
func (m *MockTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	return json.RawMessage(`{"status":"ok"}`), nil
}

func TestUnifiedToolRegistry_RegisterTool(t *testing.T) {
	r := NewUnifiedToolRegistry()

	t.Run("success", func(t *testing.T) {
		tool := &MockTool{
			name:        "valid_tool",
			description: "A valid tool",
			schema:      `{"type":"object"}`,
		}
		err := r.RegisterTool(tool)
		require.NoError(t, err)

		retrievedTool, exists := r.GetTool("valid_tool")
		assert.True(t, exists)
		assert.Equal(t, tool, retrievedTool)
	})

	t.Run("duplicate registration", func(t *testing.T) {
		tool := &MockTool{
			name:        "dup_tool",
			description: "A valid tool",
			schema:      `{"type":"object"}`,
		}
		err := r.RegisterTool(tool)
		require.NoError(t, err)

		err = r.RegisterTool(tool)
		require.Error(t, err)
		assert.Contains(t, err.Error(), "already registered")
	})

	t.Run("empty schema", func(t *testing.T) {
		tool := &MockTool{
			name:        "empty_schema_tool",
			description: "Has empty schema",
			schema:      ``,
		}
		err := r.RegisterTool(tool)
		require.Error(t, err)
		assert.Contains(t, err.Error(), "empty schema")
	})

	t.Run("invalid json schema", func(t *testing.T) {
		tool := &MockTool{
			name:        "invalid_json_tool",
			description: "Has invalid json",
			schema:      `{"type": "object"`, // missing closing brace
		}
		err := r.RegisterTool(tool)
		require.Error(t, err)
		assert.Contains(t, err.Error(), "invalid JSON schema")
	})
}

func TestUnifiedToolRegistry_ListTools(t *testing.T) {
	r := NewUnifiedToolRegistry()

	tool1 := &MockTool{
		name:        "tool1",
		description: "desc1",
		schema:      `{"type":"object"}`,
	}
	tool2 := &MockTool{
		name:        "tool2",
		description: "desc2",
		schema:      `{"type":"object"}`,
	}

	require.NoError(t, r.RegisterTool(tool1))
	require.NoError(t, r.RegisterTool(tool2))

	manifests := r.ListTools()
	assert.Len(t, manifests, 2)

	names := []string{manifests[0].Name, manifests[1].Name}
	assert.Contains(t, names, "tool1")
	assert.Contains(t, names, "tool2")
}

func TestUnifiedToolRegistry_GetTool(t *testing.T) {
	r := NewUnifiedToolRegistry()

	_, exists := r.GetTool("nonexistent")
	assert.False(t, exists)

	tool := &MockTool{
		name:        "existing",
		description: "desc",
		schema:      `{}`,
	}
	require.NoError(t, r.RegisterTool(tool))

	retrieved, exists := r.GetTool("existing")
	assert.True(t, exists)
	assert.Equal(t, "existing", retrieved.Name())
}
