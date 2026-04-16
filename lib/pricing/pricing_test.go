package pricing

import (
	"testing"
)

func TestGetFeatures(t *testing.T) {
	tests := []struct {
		name     string
		tier     Tier
		expected Features
	}{
		{"Free Tier", FreeTier, Features{MaxRAGQueries: 100, MaxAgents: 1, HasAutoDream: false}},
		{"Pro Tier", ProTier, Features{MaxRAGQueries: 1000, MaxAgents: 5, HasAutoDream: true}},
		{"Premium Tier", PremiumTier, Features{MaxRAGQueries: 5000, MaxAgents: 20, HasAutoDream: true}},
		{"Enterprise Tier", EnterpriseTier, Features{MaxRAGQueries: -1, MaxAgents: -1, HasAutoDream: true}},
		{"Unknown Tier", Tier("UNKNOWN"), Features{MaxRAGQueries: 100, MaxAgents: 1, HasAutoDream: false}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := GetFeatures(tt.tier)
			if got != tt.expected {
				t.Errorf("GetFeatures() = %v, want %v", got, tt.expected)
			}
		})
	}
}
