package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/stretchr/testify/assert"
)

func TestFactory_Standalone(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	provider := Factory("/tmp")
	assert.True(t, provider.IsLocal())
	_, ok := provider.(*LocalFSProvider)
	assert.True(t, ok)
}

func TestFactory_Cloud(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "false")
	provider := Factory("/tmp")
	assert.False(t, provider.IsLocal())
	adapter, ok := provider.(*CloudToFSProviderAdapter)
	assert.True(t, ok)

	// Test context extraction
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant1"})
	assert.Equal(t, "tenant1", adapter.getTenantID(ctx))
}
