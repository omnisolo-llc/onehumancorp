package harness

import (
    "context"

    "github.com/onehumancorp/mono/srcs/harness/sandbox"
)

type ExecutionRequest = sandbox.ExecutionRequest
type ExecutionResponse = sandbox.ExecutionResponse

func Execute(ctx context.Context, req *ExecutionRequest) (*ExecutionResponse, error) {
    sb := sandbox.New()
    return sb.Execute(ctx, req)
}
