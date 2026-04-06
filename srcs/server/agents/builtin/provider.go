package builtin

import (
	"github.com/onehumancorp/mono/srcs/server/agents"
)

// Provider implements agents.Provider for the Builtin agent.
type Provider struct{}

func (p *Provider) Type() agents.ProviderType { return agents.ProviderTypeBuiltin }
func (p *Provider) Description() string       { return "Built-in local agent reimplementing the claude-code harness." }

func (p *Provider) SupportedRoles() []string {
	return []string{
		"CEO", "PRODUCT_MANAGER", "SOFTWARE_ENGINEER", "ENGINEERING_DIRECTOR",
		"QA_TESTER", "SECURITY_ENGINEER", "DESIGNER", "MARKETING_MANAGER",
		"GROWTH_AGENT", "CONTENT_STRATEGIST", "SEO_SPECIALIST", "PAID_MEDIA_MANAGER",
		"ANALYTICS_ENGINEER", "CFO", "BOOKKEEPER", "TAX_SPECIALIST",
		"AUDIT_MANAGER", "PAYROLL_MANAGER", "AI_NEWS_COLLECTOR",
	}
}

func (p *Provider) Authenticate(_ agents.Credentials) error { return nil }
func (p *Provider) GetCredentials() agents.Credentials      { return agents.Credentials{} }
func (p *Provider) IsAuthenticated() bool                   { return true }
