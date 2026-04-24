package departments

import (
	"fmt"
	"sync"
)

type Manager struct {
	mu          sync.RWMutex
	departments map[string]Department
}

func NewManager() *Manager {
	return &Manager{
		departments: make(map[string]Department),
	}
}

func (m *Manager) RegisterDepartment(name string, d Department) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.departments[name] = d
}

func (m *Manager) GetDepartment(name string) (Department, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	d, ok := m.departments[name]
	if !ok {
		return nil, fmt.Errorf("department %s not found", name)
	}
	return d, nil
}
