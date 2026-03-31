package agents

import (
	"errors"
)

type GitHubMCPProvider struct {
	creds Credentials
}

func (p *GitHubMCPProvider) Type() ProviderType {
	return ProviderTypeGitHubMCP
}

func (p *GitHubMCPProvider) Description() string {
	return "GitHub Model Context Protocol server for repository management"
}

func (p *GitHubMCPProvider) SupportedRoles() []string {
	return []string{"SWE", "Reviewer"}
}

func (p *GitHubMCPProvider) Authenticate(creds Credentials) error {
	if creds.APIKey == "" {
		return errors.New("github-mcp provider requires a GitHub Personal Access Token")
	}
	p.creds = creds
	return nil
}

func (p *GitHubMCPProvider) IsAuthenticated() bool {
	return p.creds.APIKey != ""
}

func (p *GitHubMCPProvider) GetCredentials() Credentials {
	return p.creds
}
