package mcp

import (
	"context"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNewSecretProvider(t *testing.T) {
	tests := []struct {
		name          string
		standalone    string
		multitenant   string
		expectedType  interface{}
		expectedKey   string
		expectedValue string
	}{
		{
			name:          "Standalone mode only",
			standalone:    "true",
			multitenant:   "false",
			expectedType:  &LocalSecretProvider{},
			expectedKey:   "test-key",
			expectedValue: "local-secret-for-test-key",
		},
		{
			name:          "Multitenant mode only",
			standalone:    "false",
			multitenant:   "true",
			expectedType:  &CloudSecretProvider{},
			expectedKey:   "test-key",
			expectedValue: "cloud-secret-for-test-key",
		},
		{
			name:          "Both modes true (defaults to local)",
			standalone:    "true",
			multitenant:   "true",
			expectedType:  &LocalSecretProvider{},
			expectedKey:   "test-key",
			expectedValue: "local-secret-for-test-key",
		},
		{
			name:          "Neither mode true (defaults to local)",
			standalone:    "false",
			multitenant:   "false",
			expectedType:  &LocalSecretProvider{},
			expectedKey:   "test-key",
			expectedValue: "local-secret-for-test-key",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Setup environment
			os.Setenv("OHC_STANDALONE", tt.standalone)
			os.Setenv("OHC_MULTITENANT", tt.multitenant)
			defer os.Unsetenv("OHC_STANDALONE")
			defer os.Unsetenv("OHC_MULTITENANT")

			provider := NewSecretProvider()
			assert.IsType(t, tt.expectedType, provider)

			val, err := provider.GetSecret(context.Background(), tt.expectedKey)
			assert.NoError(t, err)
			assert.Equal(t, tt.expectedValue, val)

			// Test empty key error path
			_, err = provider.GetSecret(context.Background(), "")
			assert.Error(t, err)
			assert.Equal(t, "empty secret key", err.Error())
		})
	}
}
