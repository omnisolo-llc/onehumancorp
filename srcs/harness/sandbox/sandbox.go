package sandbox

import (
    "context"
    "os/exec"
)

var execCommandContext = exec.CommandContext

type ExecutionRequest struct {
    CmdArgs []string
    Env     []string
    WorkDir string
}

type ExecutionResponse struct {
    Stdout   []byte
    Stderr   []byte
    ExitCode int
}

type Sandbox interface {
    Execute(ctx context.Context, req *ExecutionRequest) (*ExecutionResponse, error)
}

func New() Sandbox {
    return newSandbox()
}
