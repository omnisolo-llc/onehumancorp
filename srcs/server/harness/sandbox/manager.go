package sandbox

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"sync"
)

type SandboxPolicy struct {
	DisabledCommands []string `json:"disabled_commands"`
	DisabledPatterns []string `json:"disabled_patterns"`
	ReadOnlyPaths    []string `json:"read_only_paths"`
	BlockedDomains   []string `json:"blocked_domains"`
}

type SandboxManager struct {
	mu        sync.RWMutex
	evaluator *PermissionEvaluator
	wrapper   *BashWrapper
}

var (
	instance *SandboxManager
	once     sync.Once
)

func parsePolicyFromEnv() *SandboxPolicy {
	policyStr := os.Getenv("OOS_SANDBOX_POLICY")
	if policyStr != "" {
		var policy SandboxPolicy
		err := json.Unmarshal([]byte(policyStr), &policy)
		if err == nil {
			return &policy
		}
	}
	return nil
}

func GetSandboxManager() *SandboxManager {
	once.Do(func() {
		instance = &SandboxManager{
			evaluator: NewPermissionEvaluator(),
			wrapper:   NewBashWrapper(),
		}

		policy := parsePolicyFromEnv()
		if policy != nil {
			instance.UpdatePolicy(*policy)
		}
	})
	return instance
}

func (s *SandboxManager) WrapCommand(ctx context.Context, cmd string) (string, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	if !s.evaluator.Evaluate(cmd) {
		RecordViolation(ctx, cmd, "Command execution denied by sandbox policy")
		return "", fmt.Errorf("Command execution denied by sandbox policy")
	}

	RecordExecution(ctx, cmd)
	return s.wrapper.Wrap(cmd), nil
}

func (s *SandboxManager) AnnotateError(err error, stdout string) string {
	return fmt.Sprintf("SANDBOX_FAILURE: %v\nSTDOUT:\n%s", err, stdout)
}

func (s *SandboxManager) UpdatePolicy(policy SandboxPolicy) {
	s.mu.Lock()
	defer s.mu.Unlock()

	s.evaluator.UpdatePolicy(policy)
	s.wrapper.UpdatePolicy(policy)
}
