package agents

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"sync"
	"syscall"
)

// LocalManager implements AgentManager using local processes (optionally sandboxed).
type LocalManager struct {
	binaryPath string
	useSandbox bool
	mu         sync.Mutex
	processes  map[string]*os.Process
}

// NewLocalManager creates a new LocalManager.
func NewLocalManager(binaryPath string, useSandbox bool) *LocalManager {
	return &LocalManager{
		binaryPath: binaryPath,
		useSandbox: useSandbox,
		processes:  make(map[string]*os.Process),
	}
}

// SpawnAgent starts a new agent instance as a local process.
func (m *LocalManager) SpawnAgent(ctx context.Context, agent Agent, config string) error {
	var cmd *exec.Cmd

	if m.useSandbox {
		args := []string{
			"--unshare-pid",
			"--unshare-uts",
			"--unshare-ipc",
			"--unshare-cgroup",
			"--unshare-net",
			"--proc", "/proc",
			"--dev", "/dev",
			"--ro-bind", "/", "/",
			"--", m.binaryPath,
		}
		cmd = exec.Command("bwrap", args...)
	} else {
		cmd = exec.Command(m.binaryPath)
	}

	cmd.Env = append(os.Environ(),
		fmt.Sprintf("OHC_AGENT_ID=%s", agent.ID),
		fmt.Sprintf("OHC_AGENT_ROLE=%s", agent.Role),
		"OHC_MESSAGE_BUS_URL=nats://127.0.0.1:4222", // Default for standalone
	)

	if err := cmd.Start(); err != nil {
		return fmt.Errorf("failed to start process: %w", err)
	}

	m.mu.Lock()
	m.processes[agent.ID] = cmd.Process
	m.mu.Unlock()

	return nil
}

// TerminateAgent stops a running agent process.
func (m *LocalManager) TerminateAgent(ctx context.Context, agentID string) error {
	m.mu.Lock()
	process, ok := m.processes[agentID]
	delete(m.processes, agentID)
	m.mu.Unlock()

	if !ok {
		return fmt.Errorf("process not found for agent %s", agentID)
	}

	if err := process.Kill(); err != nil {
		return fmt.Errorf("failed to kill process: %w", err)
	}

	return nil
}

// GetAgentStatus retrieves the current status of an agent process.
func (m *LocalManager) GetAgentStatus(ctx context.Context, agentID string) (Status, error) {
	m.mu.Lock()
	process, ok := m.processes[agentID]
	m.mu.Unlock()

	if !ok {
		return StatusIdle, nil
	}

	// Check if process is still running by sending signal 0
	err := process.Signal(os.Signal(syscall.Signal(0)))
	if err == nil {
		return StatusActive, nil
	}
	return StatusIdle, nil
}
