//go:build linux

package sandbox

import (
    "bytes"
    "context"
    "os/exec"
)

type linuxSandbox struct{}

func newSandbox() Sandbox {
    return &linuxSandbox{}
}

func (s *linuxSandbox) Execute(ctx context.Context, req *ExecutionRequest) (*ExecutionResponse, error) {
    args := []string{"--unshare-all", "--share-net", "--ro-bind", "/", "/", "--bind", "/tmp/agent_workspace", "/tmp/agent_workspace", "--dev", "/dev"}
    args = append(args, req.CmdArgs...)
    cmd := execCommandContext(ctx, "bwrap", args...)
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
