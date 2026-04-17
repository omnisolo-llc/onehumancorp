package mcp_secret_vault

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/zalando/go-keyring"
)

func TestLocalAdapter(t *testing.T) {
	keyring.MockInit()
	ctx := context.Background()

	adapter := NewLocalAdapter()
	assert.NotNil(t, adapter)

	// Test SetSecret
	err := adapter.SetSecret(ctx, "my_local_key", "my_local_secret", "ignored_tenant")
	assert.NoError(t, err)

	// Test GetSecret
	val, err := adapter.GetSecret(ctx, "my_local_key", "ignored_tenant")
	assert.NoError(t, err)
	assert.Equal(t, "my_local_secret", val)

	// Test GetSecret not found
	_, err = adapter.GetSecret(ctx, "non_existent_key", "ignored_tenant")
	assert.Error(t, err)
	assert.Equal(t, keyring.ErrNotFound, err)
}
