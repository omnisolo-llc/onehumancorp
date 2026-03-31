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
func TestGitHubMCPProvider(t *testing.T) {
	p := &GitHubMCPProvider{}

	if p.Type() != ProviderTypeGitHubMCP {
		t.Errorf("expected type %q, got %q", ProviderTypeGitHubMCP, p.Type())
	}
	if p.Description() == "" {
		t.Errorf("expected non-empty description")
	}
	if len(p.SupportedRoles()) == 0 {
		t.Errorf("expected non-empty supported roles")
	}

	if p.IsAuthenticated() {
		t.Errorf("expected new provider to be unauthenticated")
	}

	err := p.Authenticate(Credentials{})
	if err == nil {
		t.Errorf("expected error when authenticating with empty credentials")
	}

	err = p.Authenticate(Credentials{APIKey: "ghp_12345"})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if !p.IsAuthenticated() {
		t.Errorf("expected provider to be authenticated")
	}

	creds := p.GetCredentials()
	if creds.APIKey != "ghp_12345" {
		t.Errorf("expected credentials to match, got %q", creds.APIKey)
	}
}
