package onboarding

import (
	"context"
	"testing"
)

func TestValidateConfig(t *testing.T) {
	v := &ValidationEndpoint{}
	ctx := context.Background()

	tests := []struct {
		name    string
		config  map[string]string
		wantErr bool
	}{
		{
			name: "valid cloud config",
			config: map[string]string{
				"mode":  "cloud",
				"db":    "postgres",
				"cache": "redis",
			},
			wantErr: false,
		},
		{
			name: "invalid cloud config db",
			config: map[string]string{
				"mode":  "cloud",
				"db":    "sqlite",
				"cache": "redis",
			},
			wantErr: true,
		},
		{
			name: "valid standalone config",
			config: map[string]string{
				"mode":  "standalone",
				"db":    "sqlite",
				"cache": "memory",
			},
			wantErr: false,
		},
		{
			name: "invalid standalone config cache",
			config: map[string]string{
				"mode":  "standalone",
				"db":    "sqlite",
				"cache": "redis",
			},
			wantErr: true,
		},
		{
			name: "missing mode",
			config: map[string]string{
				"db":    "sqlite",
				"cache": "memory",
			},
			wantErr: true,
		},
		{
			name: "unknown mode",
			config: map[string]string{
				"mode":  "unknown",
				"db":    "sqlite",
				"cache": "memory",
			},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if err := v.ValidateConfig(ctx, tt.config); (err != nil) != tt.wantErr {
				t.Errorf("ValidateConfig() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
