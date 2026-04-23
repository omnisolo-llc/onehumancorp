package harness

import (
	"context"
	"fmt"
)

// ServerlessBackend is a mock backend that simulates serverless (e.g., Modal/Daytona) execution.
type ServerlessBackend struct{}

func NewServerlessBackend() *ServerlessBackend {
	return &ServerlessBackend{}
}

func (s *ServerlessBackend) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	// Mock implementation
	output := fmt.Sprintf("[ServerlessBackend] Executing command in serverless environment: %v\n", execCtx.Command)
	return []byte(output), nil
}
