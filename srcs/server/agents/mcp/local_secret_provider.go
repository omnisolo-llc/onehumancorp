package mcp

// added for Hybrid Secrets Management MCP Proxy

import (
	"context"
	"fmt"
	"sync"
)

type LocalSecretProvider struct {
	// mock local store
	store map[string]string
	mu    sync.RWMutex
}

func (p *LocalSecretProvider) GetSecret(ctx context.Context, key string) (string, error) {
	p.mu.Lock()
	if p.store == nil {
		p.store = make(map[string]string)
		// Load mock secrets for local mode
		p.store["mock_key"] = "mock_secret_local"
	}
	p.mu.Unlock()

	p.mu.RLock()
	val, ok := p.store[key]
	p.mu.RUnlock()

	if !ok {
		return "", fmt.Errorf("secret %s not found in local store", key)
	}
	return val, nil
}
