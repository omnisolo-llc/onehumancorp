package harness

import (
	"context"
	"time"
)

type AttemptResult struct {
	Stdout string
	Stderr string
	ExitCode int
}

type HarnessLifecycle interface {
	RunAttempt(ctx context.Context, agentID string, prompt string) (*AttemptResult, error)
	ResetSession(ctx context.Context, sessionID string) error
	CompactContext(ctx context.Context, sessionID string) error
}

type DefaultHarnessLifecycle struct {
	backend SandboxBackend
}

func NewDefaultHarnessLifecycle(backend SandboxBackend) *DefaultHarnessLifecycle {
	return &DefaultHarnessLifecycle{backend: backend}
}

func (l *DefaultHarnessLifecycle) RunAttempt(ctx context.Context, agentID string, prompt string) (*AttemptResult, error) {
	start := time.Now()
	res, err := l.backend.ExecuteCommand(ctx, prompt)

	duration := time.Since(start).Seconds()
	RecordRunAttempt(ctx, duration)

	if err != nil {
		return nil, err
	}

	return &AttemptResult{
		Stdout: res.Stdout,
		Stderr: res.Stderr,
		ExitCode: res.ExitCode,
	}, nil
}

func (l *DefaultHarnessLifecycle) ResetSession(ctx context.Context, sessionID string) error {
	return nil
}

func (l *DefaultHarnessLifecycle) CompactContext(ctx context.Context, sessionID string) error {
	return nil
}
