package local_stateful_execution_proxy

import (
	"context"
	"fmt"
	"io"
	"os/exec"
	"sync"

	pb "github.com/onehumancorp/mono/src/proto/mcp_proxy"
	"google.golang.org/grpc"
)

type ProxyClient struct {
	serverAddress  string
	spiffeID       string
	supportedTools []string

	mu sync.Mutex
}

func NewProxyClient(serverAddress string, spiffeID string, supportedTools []string) *ProxyClient {
	return &ProxyClient{
		serverAddress:  serverAddress,
		spiffeID:       spiffeID,
		supportedTools: supportedTools,
	}
}

func (c *ProxyClient) ConnectAndServe(ctx context.Context, conn *grpc.ClientConn) error {
	client := pb.NewMcpReverseTunnelServiceClient(conn)
	return c.ServeStream(ctx, client)
}

func (c *ProxyClient) ServeStream(ctx context.Context, client pb.McpReverseTunnelServiceClient) error {
	stream, err := client.EstablishTunnel(ctx)
	if err != nil {
		emitMetrics(ctx, "establish_tunnel_error", 1)
		return fmt.Errorf("failed to establish tunnel: %w", err)
	}
	emitMetrics(ctx, "establish_tunnel_success", 1)

	// Send registration
	err = stream.Send(&pb.ProxyToServer{
		RequestId: "init",
		Payload: &pb.ProxyToServer_Register{
			Register: &pb.RegisterProxyRequest{
				SpiffeId:       c.spiffeID,
				SupportedTools: c.supportedTools,
			},
		},
	})
	if err != nil {
		emitMetrics(ctx, "register_error", 1)
		return fmt.Errorf("failed to send registration: %w", err)
	}

	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			emitMetrics(ctx, "stream_receive_error", 1)
			return fmt.Errorf("error receiving from stream: %w", err)
		}

		if invokeReq := req.GetInvokeRequest(); invokeReq != nil {
			if invokeReq.ToolId == "shell" {
				out, execErr := c.ExecuteShellCommand(invokeReq.Params)

				resp := &pb.InvokeCommandResponse{
					Success: execErr == nil,
					Result:  out,
				}
				if execErr != nil {
					resp.ErrorDetails = execErr.Error()
					emitMetrics(ctx, "shell_execution_error", 1)
				} else {
					emitMetrics(ctx, "shell_execution_success", 1)
				}

				err = stream.Send(&pb.ProxyToServer{
					RequestId: req.RequestId,
					Payload: &pb.ProxyToServer_InvokeResponse{
						InvokeResponse: resp,
					},
				})
				if err != nil {
					emitMetrics(ctx, "send_response_error", 1)
					return fmt.Errorf("failed to send response: %w", err)
				}
			}
		}
	}
}

func (c *ProxyClient) ExecuteShellCommand(cmd string) (string, error) {
	out, err := exec.Command("sh", "-c", cmd).CombinedOutput()
	if err != nil {
		return string(out), fmt.Errorf("command execution failed: %w", err)
	}
	return string(out), nil
}
