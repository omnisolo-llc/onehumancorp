package integrations

import (
	"onehumancorp/srcs/server/integrations/restic"
)

type IntegrationProvider interface {
	Name() string
	Status() string
}

type Catalog struct {
	providers []IntegrationProvider
}

func NewCatalog() *Catalog {
	c := &Catalog{}
	c.Register(restic.NewProvider())
	return c
}

func (c *Catalog) Register(p IntegrationProvider) {
	c.providers = append(c.providers, p)
}

func (c *Catalog) List() []IntegrationProvider {
	return c.providers
}
