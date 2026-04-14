package agentgrpc

import (
	"context"
	"fmt"
	"io"
	"os"
	"time"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const (
	// DefaultAddress is the default gRPC listen / dial address for the agent process.
	DefaultAddress = "127.0.0.1:50051"

	defaultDialTimeout = 10 * time.Second
)

// Client is a gRPC client for communicating with a standalone agent process.
type Client struct {
	address string
	conn    *grpc.ClientConn
	stub    agentservicepb.AgentServiceClient
}

// AddressFromEnv returns the agent address from the OHC_AGENT_ADDRESS
// environment variable, falling back to DefaultAddress.
func AddressFromEnv() string {
	if addr := os.Getenv("OHC_AGENT_ADDRESS"); addr != "" {
		return addr
	}
	return DefaultAddress
}

// NewClient dials the agent at the given address and returns a Client.
// Call Close when done to release the connection.
func NewClient(address string) (*Client, error) {
	ctx, cancel := context.WithTimeout(context.Background(), defaultDialTimeout)
	defer cancel()

	conn, err := grpc.DialContext(ctx, address,
		grpc.WithTransportCredentials(insecure.NewCredentials()),
		grpc.WithBlock(),
	)
	if err != nil {
		return nil, fmt.Errorf("agentgrpc: dial %s: %w", address, err)
	}

	return &Client{
		address: address,
		conn:    conn,
		stub:    agentservicepb.NewAgentServiceClient(conn),
	}, nil
}

// Close closes the underlying gRPC connection.
func (c *Client) Close() error {
	return c.conn.Close()
}

// EventHandler is called for each streamed event received from RunTask.
type EventHandler func(*agentservicepb.RunTaskEvent)

// RunTask sends a RunTaskRequest and calls handler for each received event.
// It returns when the stream ends or an error occurs.
func (c *Client) RunTask(ctx context.Context, req *agentservicepb.RunTaskRequest, handler EventHandler) error {
	stream, err := c.stub.RunTask(ctx, req)
	if err != nil {
		return fmt.Errorf("agentgrpc: RunTask RPC: %w", err)
	}

	for {
		evt, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			return fmt.Errorf("agentgrpc: stream recv: %w", err)
		}
		if handler != nil {
			handler(evt)
		}
	}
}

// Ping sends a health-check to the agent and returns the response.
func (c *Client) Ping(ctx context.Context) (*agentservicepb.PingResponse, error) {
	resp, err := c.stub.Ping(ctx, &agentservicepb.PingRequest{})
	if err != nil {
		return nil, fmt.Errorf("agentgrpc: Ping: %w", err)
	}
	return resp, nil
}

// DispatchToSubAgent delegates a task to a sub-agent via the connected process.
func (c *Client) DispatchToSubAgent(ctx context.Context, req *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	resp, err := c.stub.DispatchToSubAgent(ctx, req)
	if err != nil {
		return nil, fmt.Errorf("agentgrpc: DispatchToSubAgent: %w", err)
	}
	return resp, nil
}
