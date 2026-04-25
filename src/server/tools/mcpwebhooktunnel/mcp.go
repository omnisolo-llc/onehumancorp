package mcpwebhooktunnel

import (
	"context"
	"fmt"
	"log/slog"
	"io"
	"sync"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/integrations/mcp_webhook_tunnel"
	"google.golang.org/grpc"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// LocalTunnelClient connects to the CloudRelay via gRPC.
type LocalTunnelClient struct {
	agentID  string
	conn     *grpc.ClientConn
	client   mcp_webhook_tunnel.WebhookTunnelClient
	provider db.Provider

	mu       sync.Mutex
	received int
	cancel   context.CancelFunc
}

// NewLocalTunnelClient establishes a connection to the Cloud Relay over gRPC.
// In a real environment, `dialOpts` must include credentials that present a SPIFFE SVID.
func NewLocalTunnelClient(ctx context.Context, agentID string, relayTarget string, provider db.Provider, dialOpts ...grpc.DialOption) (*LocalTunnelClient, error) {
	conn, err := grpc.DialContext(ctx, relayTarget, dialOpts...)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to relay: %w", err)
	}

	client := mcp_webhook_tunnel.NewWebhookTunnelClient(conn)

	// Ensure the local event bus table exists
	if provider != nil && provider.IsSQLite() {
		_, _ = provider.Exec(ctx, `
			CREATE TABLE IF NOT EXISTS local_webhook_events (
				id INTEGER PRIMARY KEY AUTOINCREMENT,
				agent_id TEXT,
				payload BLOB,
				created_at DATETIME DEFAULT CURRENT_TIMESTAMP
			)
		`)
	}

	c := &LocalTunnelClient{
		agentID:  agentID,
		conn:     conn,
		client:   client,
		provider: provider,
	}

	c.startListener()
	return c, nil
}

func (c *LocalTunnelClient) startListener() {
	ctx, cancel := context.WithCancel(context.Background())
	c.cancel = cancel

	go func() {
		// Auto-reconnect loop
		for {
			if ctx.Err() != nil {
				return
			}

			stream, err := c.client.ConnectStream(ctx, &mcp_webhook_tunnel.TunnelRequest{
				AgentId: c.agentID,
			})
			if err != nil {
				time.Sleep(2 * time.Second)
				continue
			}

			for {
				payload, err := stream.Recv()
				if err == io.EOF {
					break
				}
				if err != nil {
					break
				}

				c.mu.Lock()
				c.received++
				c.mu.Unlock()

				// Inject into local SQLite event bus
				if c.provider != nil && c.provider.IsSQLite() {
					_, err := c.provider.Exec(ctx,
						"INSERT INTO local_webhook_events(agent_id, payload) VALUES(?, ?)",
						c.agentID, payload.Body)
					if err != nil {
						slog.Error("failed to inject webhook to sqlite bus", "error", err)
					}
				}
			}

			time.Sleep(2 * time.Second)
		}
	}()
}

// GetReceivedCount returns the number of received webhooks (for testing).
func (c *LocalTunnelClient) GetReceivedCount() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.received
}

// ListTools returns the tools exposed by this client.
func (c *LocalTunnelClient) ListTools() []Tool {
	return []Tool{
		{
			Name:        "get_tunnel_status",
			Description: "Gets the status of the local webhook tunnel.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (c *LocalTunnelClient) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "get_tunnel_status":
		c.mu.Lock()
		count := c.received
		c.mu.Unlock()

		return map[string]interface{}{
			"status":   "connected",
			"agent_id": c.agentID,
			"received": count,
		}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// Close cancels the listener and closes the gRPC connection.
func (c *LocalTunnelClient) Close() {
	if c.cancel != nil {
		c.cancel()
	}
	if c.conn != nil {
		c.conn.Close()
	}
}
