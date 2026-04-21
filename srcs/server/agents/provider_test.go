package agents

import (
	"testing"
)

func TestProviderGetCredentials(t *testing.T) {
	creds := Credentials{APIKey: "test-key"}

	tests := []struct {
		name     string
		provider Provider
	}{
		{"GeminiProvider", &GeminiProvider{}},
		{"OpenCodeProvider", &OpenCodeProvider{}},
		{"OpenClawProvider", &OpenClawProvider{}},
		{"IronClawProvider", &IronClawProvider{}},
		{"ScoutProvider", &ScoutProvider{}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_ = tt.provider.Authenticate(creds)
			if tt.provider.GetCredentials().APIKey != "test-key" {
				t.Errorf("Expected test-key for %s GetCredentials", tt.name)
			}
		})
	}
}

func TestScoutProvider(t *testing.T) {
	p := &ScoutProvider{}
	if p.Type() != ProviderTypeScout {
		t.Errorf("Expected Type Scout, got %v", p.Type())
	}
	if p.Description() != "Scout — resource scout and tool integrator agent dedicated to finding and safely registering new external APIs" {
		t.Errorf("Unexpected description: %s", p.Description())
	}

	roles := p.SupportedRoles()
	if len(roles) != 2 || roles[0] != "SCOUT" || roles[1] != "INTEGRATION_ENGINEER" {
		t.Errorf("Unexpected roles: %v", roles)
	}

	err := p.Authenticate(Credentials{})
	if err == nil {
		t.Error("Expected error for empty credentials, got nil")
	}

	if p.IsAuthenticated() {
		t.Error("Expected IsAuthenticated to be false, got true")
	}

	err = p.Authenticate(Credentials{APIKey: "test"})
	if err != nil {
		t.Errorf("Expected no error for valid credentials, got %v", err)
	}

	if !p.IsAuthenticated() {
		t.Error("Expected IsAuthenticated to be true, got false")
	}

	err = p.Authenticate(Credentials{OAuthToken: "test"})
	if err != nil {
		t.Errorf("Expected no error for valid oauth credentials, got %v", err)
	}
}
