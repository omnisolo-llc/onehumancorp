package harness

import (
	"context"
	"fmt"
	"os"
)

type K8sBackend struct {}

func NewK8sBackend() *K8sBackend {
	return &K8sBackend{}
}

func (k *K8sBackend) ExecuteCommand(ctx context.Context, cmd string) (*ExecutionResult, error) {
	if os.Getenv("OHCMultitenant") != "true" {
		return nil, fmt.Errorf("k8s backend is disabled, set OHCMultitenant=true to enable")
	}
	return &ExecutionResult{Stdout: fmt.Sprintf("Mock K8s Execution: %s", cmd), ExitCode: 0}, nil
}

func (k *K8sBackend) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if os.Getenv("OHCMultitenant") != "true" {
		return nil, fmt.Errorf("k8s backend is disabled, set OHCMultitenant=true to enable")
	}
	return []byte("mock k8s file content"), nil
}

func (k *K8sBackend) WriteFile(ctx context.Context, path string, content []byte) error {
	if os.Getenv("OHCMultitenant") != "true" {
		return fmt.Errorf("k8s backend is disabled, set OHCMultitenant=true to enable")
	}
	return nil
}
