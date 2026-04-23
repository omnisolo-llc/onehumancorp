package harness

import (
	"context"
	"fmt"
)

// DockerBackend is a mock backend that simulates spinning up a container per agent session.
type DockerBackend struct{}

func NewDockerBackend() *DockerBackend {
	return &DockerBackend{}
}

func (d *DockerBackend) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	// Mock implementation
	output := fmt.Sprintf("[DockerBackend] Executing command in container: %v\n", execCtx.Command)
	return []byte(output), nil
}
