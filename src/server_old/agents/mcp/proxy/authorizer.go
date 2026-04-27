package proxy

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/src/server_old/telemetry"
)

// Capability Profile
type CapabilityProfile struct {
	AllowedCapabilities []string `json:"allowed_capabilities"`
	DeniedCapabilities  []string `json:"denied_capabilities"`
}

type SandboxViolationStore interface {
	LogViolation(ctx context.Context, sessionID, capability, toolName string) error
	GetViolations(ctx context.Context, sessionID string) ([]string, error)
}

type InMemoryViolationStore struct {
	mu         sync.RWMutex
	violations map[string][]string
}

func NewInMemoryViolationStore() *InMemoryViolationStore {
	return &InMemoryViolationStore{
		violations: make(map[string][]string),
	}
}

func (s *InMemoryViolationStore) LogViolation(ctx context.Context, sessionID, capability, toolName string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	entry := fmt.Sprintf("[%s] Denied capability '%s' for tool '%s'", time.Now().Format(time.RFC3339), capability, toolName)
	s.violations[sessionID] = append(s.violations[sessionID], entry)
	telemetry.RecordSandboxViolation(ctx, "capability_denied", sessionID, capability)
	return nil
}

func (s *InMemoryViolationStore) GetViolations(ctx context.Context, sessionID string) ([]string, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.violations[sessionID], nil
}


type CapabilityAuthorizer struct {
	mu             sync.RWMutex
	profiles       map[string]CapabilityProfile
	violationStore SandboxViolationStore
}

func NewCapabilityAuthorizer(store SandboxViolationStore) *CapabilityAuthorizer {
	if store == nil {
		store = NewInMemoryViolationStore()
	}
	return &CapabilityAuthorizer{
		profiles:       make(map[string]CapabilityProfile),
		violationStore: store,
	}
}

func (a *CapabilityAuthorizer) SetProfile(sessionID string, profile CapabilityProfile) {
	a.mu.Lock()
	defer a.mu.Unlock()
	a.profiles[sessionID] = profile
}

func (a *CapabilityAuthorizer) Authorize(ctx context.Context, sessionID, capability, toolName string) error {
	a.mu.RLock()
	profile, exists := a.profiles[sessionID]
	a.mu.RUnlock()

	if !exists {
		// Default deny if no profile exists
		a.violationStore.LogViolation(ctx, sessionID, capability, toolName)
		return fmt.Errorf("capability %s denied: no profile for session %s", capability, sessionID)
	}

	// Check explicit denies first
	for _, denied := range profile.DeniedCapabilities {
		if denied == capability || denied == "*" {
			a.violationStore.LogViolation(ctx, sessionID, capability, toolName)
			return fmt.Errorf("capability %s denied explicitly for session %s", capability, sessionID)
		}
	}

	// Check allows
	for _, allowed := range profile.AllowedCapabilities {
		if allowed == capability || allowed == "*" {
			return nil
		}
	}

	// Implicit deny
	a.violationStore.LogViolation(ctx, sessionID, capability, toolName)
	return fmt.Errorf("capability %s denied implicitly for session %s", capability, sessionID)
}
