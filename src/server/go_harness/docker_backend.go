package harness

import (
	"context"
	"fmt"
)

type DockerBackend struct {}

func NewDockerBackend() *DockerBackend {
	return &DockerBackend{}
}

func (d *DockerBackend) ExecuteCommand(ctx context.Context, cmd string) (*ExecutionResult, error) {
	return &ExecutionResult{Stdout: fmt.Sprintf("Mock Docker Execution: %s", cmd), ExitCode: 0}, nil
}

func (d *DockerBackend) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return []byte("mock file content"), nil
}

func (d *DockerBackend) WriteFile(ctx context.Context, path string, content []byte) error {
	return nil
}
