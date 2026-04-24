package main

import (
	"fmt"
	"gopkg.in/yaml.v3"
	"os"
)

func main() {
	content := []byte(`
tenant_id: "test-tenant-123"
context: "some text"
`)
	var doc struct {
		TenantID string `yaml:"tenant_id"`
		Context  string `yaml:"context"`
	}
	err := yaml.Unmarshal(content, &doc)
	fmt.Println(doc.TenantID, err)
}
