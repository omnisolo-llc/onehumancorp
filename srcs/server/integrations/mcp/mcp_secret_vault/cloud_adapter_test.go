package mcp_secret_vault

import (
	"context"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCloudAdapter_NilDB(t *testing.T) {
	adapter := NewCloudAdapter(nil)
	assert.NotNil(t, adapter)

	ctx := context.Background()

	// Test SetSecret with nil DB
	err := adapter.SetSecret(ctx, "mykey", "myval", "tenant1")
	assert.Error(t, err)
	assert.Equal(t, "database connection is nil", err.Error())

	// Test GetSecret with nil DB
	_, err = adapter.GetSecret(ctx, "mykey", "tenant1")
	assert.Error(t, err)
	assert.Equal(t, "database connection is nil", err.Error())
}
