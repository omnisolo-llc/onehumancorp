
package local_execution_proxy

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/protobuf/proto"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

type toolArgs struct {
	Command string `json:"command"`
}

func getSPIFFETLSCredentials(expectedSpiffeID string) (credentials.TransportCredentials, error) {
	certPath := os.Getenv("SPIFFE_CERT_PATH")
	keyPath := os.Getenv("SPIFFE_KEY_PATH")
	caPath := os.Getenv("SPIFFE_CA_PATH")

	if certPath == "" {
		certPath = "/tmp/spire-agent/public/svid.pem"
	}
	if keyPath == "" {
		keyPath = "/tmp/spire-agent/private/svid_key.pem"
	}
	if caPath == "" {
		caPath = "/tmp/spire-agent/public/bundle.pem"
	}

	cert, err := tls.LoadX509KeyPair(certPath, keyPath)
	if err != nil {
        if os.Getenv("CI") == "true" || os.Getenv("ALLOW_INSECURE_TESTS") == "true" {
             return nil, fmt.Errorf("certificate missing, but insecure fallback disabled")
        }
		return nil, fmt.Errorf("could not load client key pair: %s", err)
	}

	caCert, err := os.ReadFile(caPath)
	if err != nil {
		return nil, fmt.Errorf("could not read ca certificate: %s", err)
	}

	caCertPool := x509.NewCertPool()
	caCertPool.AppendCertsFromPEM(caCert)

	tlsConfig := &tls.Config{
		Certificates: []tls.Certificate{cert},
		RootCAs:      caCertPool,
        MinVersion:   tls.VersionTLS13,
        InsecureSkipVerify: true, // SPIFFE relies on URI SANs, so we skip default DNS verification
		VerifyPeerCertificate: func(rawCerts [][]byte, verifiedChains [][]*x509.Certificate) error {
			if expectedSpiffeID == "" {
				return fmt.Errorf("SPIFFE ID validation is strictly required, expected ID cannot be empty")
			}

			if len(rawCerts) == 0 {
				return fmt.Errorf("no peer certificates provided")
			}

			// Parse all certificates
			var certs []*x509.Certificate
			for _, rawCert := range rawCerts {
				cert, err := x509.ParseCertificate(rawCert)
				if err != nil {
					return fmt.Errorf("failed to parse peer certificate: %w", err)
				}
				certs = append(certs, cert)
			}

			leaf := certs[0]

			// Setup intermediate pool if applicable
			intermediates := x509.NewCertPool()
			for i := 1; i < len(certs); i++ {
				intermediates.AddCert(certs[i])
			}

			// Verify the certificate cryptographically against our Root CAs
			opts := x509.VerifyOptions{
				Roots:         caCertPool,
				Intermediates: intermediates,
				KeyUsages:     []x509.ExtKeyUsage{x509.ExtKeyUsageAny},
			}
			if _, err := leaf.Verify(opts); err != nil {
				return fmt.Errorf("failed to verify certificate signature: %w", err)
			}

			// Verify SPIFFE ID
			for _, uri := range leaf.URIs {
				if uri.String() == expectedSpiffeID {
					return nil // Found matching SPIFFE ID
				}
			}

			return fmt.Errorf("peer certificate does not contain expected SPIFFE ID: %s", expectedSpiffeID)
		},
	}

	return credentials.NewTLS(tlsConfig), nil
}

type LocalStatefulExecutionProxyIntegration struct {
	tunnelClient *ReverseTunnelClient
	mcpTool      *LocalExecutionMCPTool
}

func NewLocalStatefulExecutionProxyIntegration(gatewayURL, spiffeID string) (*LocalStatefulExecutionProxyIntegration, error) {
	client := NewReverseTunnelClient(gatewayURL, spiffeID)
	tool, err := NewLocalExecutionMCPTool()
	if err != nil {
		return nil, err
	}

	return &LocalStatefulExecutionProxyIntegration{
		tunnelClient: client,
		mcpTool:      tool,
	}, nil
}

func (s *LocalStatefulExecutionProxyIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("local-stateful-execution-proxy"),
		Name:        proto.String("Local Stateful Execution Proxy"),
		Type:        proto.String("local_stateful_execution_proxy"),
		Category:    proto.String("system"),
		Description: proto.String("A bridge that allows cloud agents to execute commands locally on a user's machine."),
		Publisher:   proto.String("OneHumanCorp"),
		Icon:        proto.String("terminal"),
		Tags:        []string{"execution", "local", "system"}}.Build()
}

func (s *LocalStatefulExecutionProxyIntegration) WizardSteps() []*pb.WizardStep { return nil }

type ReverseTunnelClient struct {
	gatewayURL string
	spiffeID   string
	conn       *grpc.ClientConn
}

func NewReverseTunnelClient(gatewayURL, spiffeID string) *ReverseTunnelClient {
	return &ReverseTunnelClient{
		gatewayURL: gatewayURL,
		spiffeID:   spiffeID,
	}
}

func (c *ReverseTunnelClient) Connect(ctx context.Context, dialOpts ...grpc.DialOption) error {
	var opts []grpc.DialOption

    // For unit tests we want to be able to inject insecure credentials
    if len(dialOpts) > 0 {
        opts = append(opts, dialOpts...)
    } else {
        tlsCreds, err := getSPIFFETLSCredentials(c.spiffeID)
        if err != nil {
            return fmt.Errorf("failed to get SPIFFE TLS credentials: %w", err)
        }
        opts = append(opts, grpc.WithTransportCredentials(tlsCreds))
    }

	conn, err := grpc.NewClient(c.gatewayURL, opts...)
	if err != nil {
		return fmt.Errorf("failed to connect to gateway %s: %w", c.gatewayURL, err)
	}
	c.conn = conn

	client := agentservicepb.NewAgentServiceClient(c.conn)
	_, err = client.Ping(ctx, &agentservicepb.PingRequest{})
	if err != nil {
		return fmt.Errorf("failed to ping gateway: %w", err)
	}

	return nil
}

func (c *ReverseTunnelClient) ListenAndServe(ctx context.Context, tool *LocalExecutionMCPTool) error {
	if c.conn == nil {
		return fmt.Errorf("connection is nil")
	}
	client := agentservicepb.NewAgentServiceClient(c.conn)

	stream, err := client.RunTask(ctx, &agentservicepb.RunTaskRequest{
		TaskId: "local-proxy",
	})
	if err != nil {
		return fmt.Errorf("failed to open stream: %w", err)
	}

    // Block and listen
    for {
        select {
        case <-ctx.Done():
            return ctx.Err()
        default:
            event, err := stream.Recv()
            if err != nil {
                return fmt.Errorf("stream closed or error received: %w", err)
            }

            if event.Type == agentservicepb.EventType_TOOL_CALL && event.ToolName == "local_execute" {
                var args toolArgs
                if err := json.Unmarshal([]byte(event.ToolArgsJson), &args); err == nil {
                    out, execErr := tool.ExecuteCommand(ctx, "bash", "-c", args.Command)

                    _, _ = client.DispatchToSubAgent(ctx, &agentservicepb.SubAgentRequest{
                        Task: fmt.Sprintf("Result: %s, Error: %v", out, execErr),
                    })
                }
            }
        }
    }
}

func (c *ReverseTunnelClient) Close() error {
	if c.conn != nil {
		return c.conn.Close()
	}
	return nil
}

type LocalExecutionMCPTool struct {
	meter           metric.Meter
	executionsTotal metric.Int64Counter
}

func NewLocalExecutionMCPTool() (*LocalExecutionMCPTool, error) {
	meter := otel.Meter("local-execution-proxy")
	executionsTotal, err := meter.Int64Counter("mcp_local_executions_total", metric.WithDescription("Total number of local command executions"))
	if err != nil {
		return nil, fmt.Errorf("failed to create metric: %w", err)
	}

	return &LocalExecutionMCPTool{
		meter:           meter,
		executionsTotal: executionsTotal,
	}, nil
}

func (t *LocalExecutionMCPTool) ExecuteCommand(ctx context.Context, command string, args ...string) (string, error) {
	if t.executionsTotal != nil {
	    t.executionsTotal.Add(ctx, 1)
    }

    execCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
    defer cancel()

	cmd := exec.CommandContext(execCtx, command, args...)
	out, err := cmd.CombinedOutput()
	if err != nil {
		return string(out), fmt.Errorf("command execution failed: %w - output: %s", err, string(out))
	}
	return string(out), nil
}
