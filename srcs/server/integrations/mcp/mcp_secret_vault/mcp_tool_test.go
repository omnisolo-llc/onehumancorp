package mcp_secret_vault

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestListTools(t *testing.T) {
    mockStorage := new(MockSecretStorage)
    vault, _ := NewMCPSecretVault("local", mockStorage)
	tools := vault.ListTools()
	assert.Len(t, tools, 2)
	assert.Equal(t, "get_secret", tools[0].Name)
	assert.Equal(t, "set_secret", tools[1].Name)
}

func TestCallTool(t *testing.T) {
    ctx := context.Background()
    mockStorage := new(MockSecretStorage)
    vault, _ := NewMCPSecretVault("local", mockStorage)

    mockStorage.On("GetSecret", ctx, "mykey", "").Return("mysecret", nil)

    res, err := vault.CallTool(ctx, "get_secret", map[string]interface{}{"key": "mykey"})
    assert.NoError(t, err)

    resMap, ok := res.(map[string]interface{})
    assert.True(t, ok)
    assert.Equal(t, "mysecret", resMap["secret"])

    mockStorage.On("SetSecret", ctx, "mykey", "myval", "").Return(nil)

    res, err = vault.CallTool(ctx, "set_secret", map[string]interface{}{"key": "mykey", "value": "myval"})
    assert.NoError(t, err)

    resMap2, ok := res.(map[string]interface{})
    assert.True(t, ok)
    assert.Equal(t, "success", resMap2["status"])
}
