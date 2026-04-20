//go:build !linux && !darwin

package sandbox

import (
    "context"
    "testing"

    "github.com/stretchr/testify/assert"
)

func TestDefaultSandbox(t *testing.T) {
    oldExecCommandContext := execCommandContext
    execCommandContext = helperCommandContext
    defer func() { execCommandContext = oldExecCommandContext }()

    sb := &defaultSandbox{}
    req := &ExecutionRequest{CmdArgs: []string{"echo", "hello"}}
    resp, err := sb.Execute(context.Background(), req)
    assert.NoError(t, err)
    assert.NotNil(t, resp)
    assert.Equal(t, 0, resp.ExitCode)

    // Test zero args
    reqZero := &ExecutionRequest{CmdArgs: []string{}}
    respZero, errZero := sb.Execute(context.Background(), reqZero)
    assert.NoError(t, errZero)
    assert.Equal(t, 0, respZero.ExitCode)
}
