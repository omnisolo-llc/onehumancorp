package integrations

type IntegrationProvider interface {
	ID() string
	Initialize() error
	Tools() []string
}

var Catalog = make(map[string]IntegrationProvider)

func Register(provider IntegrationProvider) {
	Catalog[provider.ID()] = provider
}
