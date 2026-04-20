//go:build darwin

package sandbox

import (
    "bytes"
    "context"
    "os/exec"
)

type darwinSandbox struct{}

func newSandbox() Sandbox {
    return &darwinSandbox{}
}

func (s *darwinSandbox) Execute(ctx context.Context, req *ExecutionRequest) (*ExecutionResponse, error) {
    args := []string{"-p", "(version 1)(allow default)(deny file-write*)"}
    args = append(args, req.CmdArgs...)
    cmd := execCommandContext(ctx, "sandbox-exec", args...)
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
