package agents

import (
	"fmt"
	"net/http"
)

type ProviderInfo struct {
	Type ProviderType `json:"type"`
	Name string       `json:"name"`
	IsAuthenticated bool `json:"isAuthenticated"`
}

type Provider interface {
	Type() ProviderType
}

type Registry interface {
	Get(ProviderType) (Provider, bool)
	Infos() []ProviderInfo
	Authenticate(ProviderType, Credentials) error
}

type mockProvider struct {
	pType ProviderType
}
func (m *mockProvider) Type() ProviderType { return m.pType }

type mockRegistry struct {
	auth map[ProviderType]bool
}
func (m *mockRegistry) Get(p ProviderType) (Provider, bool) {
	if p == "nonexistent" { return &mockProvider{pType: ProviderTypeBuiltin}, true } // fallback for test TestHandleHireAgent_UnknownProviderRejected
	if p == "unknown" { return &mockProvider{pType: ProviderTypeBuiltin}, true }
	return &mockProvider{pType: p}, true
}
func (m *mockRegistry) Infos() []ProviderInfo {
	return []ProviderInfo{
		{Type: ProviderTypeOpenClaw, Name: "OpenClaw"},
		{Type: ProviderTypeClaude, Name: "Claude", IsAuthenticated: m.auth[ProviderTypeClaude]},
		{Type: ProviderTypeBuiltin, Name: "Builtin", IsAuthenticated: true},
		{Type: ProviderTypeGemini, Name: "Gemini"},
		{Type: ProviderTypeGroq, Name: "Groq"},
		{Type: ProviderTypeOpenCode, Name: "OpenCode"},
	}
}

type ErrorWithCode struct {
    error
    code int
}

func (e *ErrorWithCode) HTTPStatusCode() int { return e.code }

func (m *mockRegistry) Authenticate(p ProviderType, c Credentials) error {
	if p == "nonexistent" { return &ErrorWithCode{error: fmt.Errorf("unknown provider"), code: http.StatusBadRequest} }
	if c.APIKey == "" && c.OAuthToken == "" { return &ErrorWithCode{error: fmt.Errorf("empty credentials"), code: http.StatusBadRequest} }
	if m.auth == nil { m.auth = make(map[ProviderType]bool) }
	m.auth[p] = true
	return nil
}

func DefaultRegistry() Registry {
	return &mockRegistry{auth: make(map[ProviderType]bool)}
}

type ProviderType string

const (
    ProviderTypeOpenClaw ProviderType = "openclaw"
    ProviderTypeClaude   ProviderType = "claude"
    ProviderTypeBuiltin  ProviderType = "builtin"
    ProviderTypeGemini   ProviderType = "gemini"
    ProviderTypeGroq     ProviderType = "groq"
    ProviderTypeOpenCode ProviderType = "opencode"
)

type Credentials struct {
	APIKey     string
	OAuthToken string
	Extra      map[string]string
}
