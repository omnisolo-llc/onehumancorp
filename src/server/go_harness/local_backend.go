package harness

import (
	"context"
	"fmt"
	"os/exec"
)

type ASTValidator struct{}

func NewASTValidator() *ASTValidator {
	return &ASTValidator{}
}

func (v *ASTValidator) Validate(cmd string) error {
	// mock basic validation
	return nil
}

type LocalBackend struct {
	validator *ASTValidator
}

func NewLocalBackend(validator *ASTValidator) *LocalBackend {
	return &LocalBackend{validator: validator}
}

func (l *LocalBackend) ExecuteCommand(ctx context.Context, cmd string) (*ExecutionResult, error) {
	if err := l.validator.Validate(cmd); err != nil {
		return nil, err
	}

	// simple implementation
	out, err := exec.CommandContext(ctx, "bash", "-c", cmd).CombinedOutput()
	exitCode := 0
	if err != nil {
		exitCode = -1
	}

	return &ExecutionResult{
		Stdout:   string(out),
		ExitCode: exitCode,
	}, nil
}

func (l *LocalBackend) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return nil, fmt.Errorf("not implemented")
}

func (l *LocalBackend) WriteFile(ctx context.Context, path string, content []byte) error {
	return fmt.Errorf("not implemented")
}
