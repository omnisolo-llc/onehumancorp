package builtin_integration_test

import (
	"context"
	"net"
	"os"
	"os/exec"
	"testing"
	"time"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const (
	binaryRunpath = "srcs/server/agents/builtin/src/ohc-builtin-agent"
	// Attempt to run the binary from the test runfiles root
	binaryRunpathAlternative = "_main/srcs/server/agents/builtin/src/ohc-builtin-agent"
	startTimeout  = 10 * time.Second
	rpcTimeout    = 30 * time.Second
)

// findFreePort returns a free TCP port on localhost.
func findFreePort(t *testing.T) string {
	t.Helper()
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("findFreePort: %v", err)
	}
	addr := l.Addr().String()
	l.Close()
	_, port, err := net.SplitHostPort(addr)
	if err != nil {
		t.Fatalf("SplitHostPort: %v", err)
	}
	return port
}

// locateBinary finds the Rust binary either via Bazel runfiles or by scanning
// the workspace build output directory.
func locateBinary(t *testing.T) string {
	t.Helper()

	// Try common locations
	locations := []string{
		os.Getenv("RUNFILES_DIR") + "/_main/srcs/server/agents/builtin/src/ohc-builtin-agent",
		os.Getenv("RUNFILES_DIR") + "/mono/srcs/server/agents/builtin/src/ohc-builtin-agent",
		os.Getenv("TEST_SRCDIR") + "/_main/srcs/server/agents/builtin/src/ohc-builtin-agent",
		os.Getenv("TEST_SRCDIR") + "/mono/srcs/server/agents/builtin/src/ohc-builtin-agent",
		"../../../srcs/server/agents/builtin/src/ohc-builtin-agent",
	}

	for _, p := range locations {
		if _, err := os.Stat(p); err == nil {
			return p
		}
	}

	t.Skip("Skipping test because ohc-builtin-agent binary could not be found.")
	return ""
}

// startAgent launches the Rust agent on a free port, waits until it is
// reachable, and returns the gRPC client connection and a cleanup func.
func startAgent(t *testing.T) (*grpc.ClientConn, func()) {
	t.Helper()
	bin := locateBinary(t)
	port := findFreePort(t)
	addr := "127.0.0.1:" + port

	cmd := exec.Command(bin)
	cmd.Env = append(os.Environ(),
		"OHC_AGENT_ADDRESS="+addr,
		"OHC_AGENT_ID=test-rust-agent",
		"OHC_AGENT_AUTH_DISABLED=true",
		"RUST_LOG=error",
	)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Start(); err != nil {
		t.Fatalf("start agent binary: %v", err)
	}

	cleanup := func() {
		if cmd.Process != nil {
			cmd.Process.Kill() //nolint:errcheck
			cmd.Wait()         //nolint:errcheck
		}
	}

	// Wait until the agent is reachable.
	deadline := time.Now().Add(startTimeout)
	var conn *grpc.ClientConn
	var err error
	for time.Now().Before(deadline) {
		conn, err = grpc.NewClient(addr,
			grpc.WithTransportCredentials(insecure.NewCredentials()),
		)
		if err == nil {
			ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
			client := agentservicepb.NewAgentServiceClient(conn)
			_, pingErr := client.Ping(ctx, &agentservicepb.PingRequest{})
			cancel()
			if pingErr == nil {
				return conn, cleanup
			}
			conn.Close()
		}
		time.Sleep(200 * time.Millisecond)
	}
	cleanup()
	t.Fatalf("agent did not start within %v (last error: %v)", startTimeout, err)
	return nil, nil
}
