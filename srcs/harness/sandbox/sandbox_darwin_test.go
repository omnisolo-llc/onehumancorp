//go:build darwin

package sandbox

import (
    "context"
    "testing"

    "github.com/stretchr/testify/assert"
)

func TestDarwinSandbox(t *testing.T) {
    oldExecCommandContext := execCommandContext
    execCommandContext = helperCommandContext
    defer func() { execCommandContext = oldExecCommandContext }()

    sb := &darwinSandbox{}
    req := &ExecutionRequest{CmdArgs: []string{"echo", "hello"}}
    resp, err := sb.Execute(context.Background(), req)
    assert.NoError(t, err)
    assert.NotNil(t, resp)
    assert.Equal(t, 0, resp.ExitCode)
}
