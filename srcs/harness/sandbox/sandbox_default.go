//go:build !linux && !darwin

package sandbox

import (
    "bytes"
    "context"
    "os/exec"
)

type defaultSandbox struct{}

func newSandbox() Sandbox {
    return &defaultSandbox{}
}

func (s *defaultSandbox) Execute(ctx context.Context, req *ExecutionRequest) (*ExecutionResponse, error) {
    if len(req.CmdArgs) == 0 {
        return &ExecutionResponse{ExitCode: 0}, nil
    }
    cmd := execCommandContext(ctx, req.CmdArgs[0], req.CmdArgs[1:]...)
    cmd.Env = req.Env
    cmd.Dir = req.WorkDir
    var stdout, stderr bytes.Buffer
    cmd.Stdout = &stdout
    cmd.Stderr = &stderr

    err := cmd.Run()
    exitCode := 0
    if err != nil {
        if exitError, ok := err.(*exec.ExitError); ok {
            exitCode = exitError.ExitCode()
        } else {
            return nil, err
        }
    }
    return &ExecutionResponse{
        Stdout:   stdout.Bytes(),
        Stderr:   stderr.Bytes(),
        ExitCode: exitCode,
    }, nil
}
