package agentgrpc

import (
"context"
"fmt"
"io"
"os"
"time"

agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
"google.golang.org/grpc"
"google.golang.org/grpc/credentials"
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

// ClientOptions controls how the gRPC dial is configured.
type ClientOptions struct {
// TLS holds mTLS certificate paths.  When TLS.IsSet() == true the client
// presents its certificate and validates the server against the CA.
TLS TLSConfig
// Token is the pre-shared HMAC token for bearer-token auth.
Token string
// DialTimeout overrides the default 10-second connection timeout.
DialTimeout time.Duration
}

// ClientOptionsFromEnv builds ClientOptions from the same env vars used by the server.
func ClientOptionsFromEnv() ClientOptions {
return ClientOptions{
TLS:   TLSConfigFromEnv(),
Token: os.Getenv("OHC_AGENT_TOKEN"),
}
}

// AddressFromEnv returns the agent address from OHC_AGENT_ADDRESS, else DefaultAddress.
func AddressFromEnv() string {
if addr := os.Getenv("OHC_AGENT_ADDRESS"); addr != "" {
return addr
}
return DefaultAddress
}

// NewClient dials the agent at address using opts for credentials.
// Call Close when done.
func NewClient(address string, opts ClientOptions) (*Client, error) {
timeout := opts.DialTimeout
if timeout == 0 {
timeout = defaultDialTimeout
}

dialOpts, err := buildDialOptions(opts)
if err != nil {
return nil, err
}

ctx, cancel := context.WithTimeout(context.Background(), timeout)
defer cancel()

conn, err := grpc.DialContext(ctx, address, append(dialOpts, grpc.WithBlock())...) //nolint:staticcheck
if err != nil {
return nil, fmt.Errorf("agentgrpc: dial %s: %w", address, err)
}

return &Client{
address: address,
conn:    conn,
stub:    agentservicepb.NewAgentServiceClient(conn),
}, nil
}

func buildDialOptions(opts ClientOptions) ([]grpc.DialOption, error) {
var dialOpts []grpc.DialOption

var transportCreds credentials.TransportCredentials
if opts.TLS.IsSet() {
creds, err := opts.TLS.ClientCredentials()
if err != nil {
return nil, fmt.Errorf("agentgrpc: TLS credentials: %w", err)
}
transportCreds = creds
}
if transportCreds != nil {
dialOpts = append(dialOpts, grpc.WithTransportCredentials(transportCreds))
} else {
dialOpts = append(dialOpts, grpc.WithTransportCredentials(insecure.NewCredentials()))
}

if opts.Token != "" {
dialOpts = append(dialOpts, grpc.WithPerRPCCredentials(newBearerTokenCreds(opts.Token)))
}

return dialOpts, nil
}

// Close closes the underlying gRPC connection.
func (c *Client) Close() error { return c.conn.Close() }

// EventHandler is called for each streamed event received from RunTask.
type EventHandler func(*agentservicepb.RunTaskEvent)

// RunTask sends a RunTaskRequest and calls handler for each received event.
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

// Ping sends a health-check to the agent.
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
