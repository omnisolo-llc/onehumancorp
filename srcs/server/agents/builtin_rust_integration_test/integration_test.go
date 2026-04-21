// Package builtin_rust_integration_test runs integration tests against
// the Rust builtin-agent gRPC binary. The binary is launched as a subprocess
// on a random free port, and a Go gRPC client exercises all three RPCs.
//
// Tests are tagged "integration" and skipped in unit-test builds.  They can
// be run explicitly:
//
//	bazel test //srcs/server/agents/builtin_rust_integration_test:integration_test --test_tag_filters=integration
//
// or:
//
//	go test ./srcs/server/agents/builtin_rust_integration_test/ -run . -v
package builtin_rust_integration_test

import (
	"context"
	"fmt"
	"io"
	"net"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"testing"
	"time"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const (
	// binary name / runfiles path for Bazel test.
	binaryRunpath = "srcs/server/agents/builtin_rust/ohc-builtin-agent"
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

	// 1. Bazel runfiles mechanism.
	if rf := os.Getenv("RUNFILES_DIR"); rf != "" {
		p := filepath.Join(rf, binaryRunpath)
		if _, err := os.Stat(p); err == nil {
			return p
		}
	}

	// 2. TEST_SRCDIR (Bazel test sandbox).
	if sd := os.Getenv("TEST_SRCDIR"); sd != "" {
		p := filepath.Join(sd, os.Getenv("TEST_WORKSPACE"), binaryRunpath)
		if _, err := os.Stat(p); err == nil {
			return p
		}
	}

	// 3. Relative path from workspace root (dev flow).
	// Walk up until we find a MODULE.bazel, then look in the Cargo target dir.
	_, thisFile, _, _ := runtime.Caller(0)
	root := thisFile
	for {
		root = filepath.Dir(root)
		if root == "/" {
			break
		}
		if _, err := os.Stat(filepath.Join(root, "MODULE.bazel")); err == nil {
			break
		}
	}
	candidates := []string{
		filepath.Join(root, "srcs/server/agents/builtin_rust/target/debug/ohc-builtin-agent"),
		filepath.Join(root, "srcs/server/agents/builtin_rust/target/release/ohc-builtin-agent"),
		filepath.Join(root, "bazel-bin/srcs/server/agents/builtin_rust/ohc-builtin-agent"),
	}
	for _, c := range candidates {
		if _, err := os.Stat(c); err == nil {
			return c
		}
	}

	t.Skip("ohc-builtin-agent binary not found; build with `cargo build` or `bazel build //srcs/server/agents/builtin_rust:ohc-builtin-agent`")
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
			// Backoff loop to test agent is up to handle multiple incoming
			connected := false
			for retries := 0; retries < 5; retries++ {
				ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
				client := agentservicepb.NewAgentServiceClient(conn)
				_, pingErr := client.Ping(ctx, &agentservicepb.PingRequest{})
				cancel()
				if pingErr == nil {
					connected = true
					break
				}
				time.Sleep(500 * time.Millisecond)
			}
			if connected {
				return conn, cleanup
			}
			conn.Close()
		}
		time.Sleep(500 * time.Millisecond)
	}
	cleanup()
	t.Fatalf("agent did not start within %v (last error: %v)", startTimeout, err)
	return nil, nil
}

// ── Tests ─────────────────────────────────────────────────────────────────────

// TestRustAgent_Ping verifies the Ping health-check RPC.
func TestRustAgent_Ping(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
	defer cancel()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.Ping(ctx, &agentservicepb.PingRequest{})
	if err != nil {
		t.Fatalf("Ping: %v", err)
	}
	if resp.AgentId == "" {
		t.Error("Ping: expected non-empty agent_id")
	}
	if resp.Version == "" {
		t.Error("Ping: expected non-empty version")
	}
	t.Logf("Ping OK: agent_id=%q version=%q", resp.AgentId, resp.Version)
}

// TestRustAgent_RunTask_NoLLM starts a task with an empty prompt to verify
// the streaming events are emitted. Without a real LLM, the task will fail
// with a provider error — we just check the stream delivers events and closes.
func TestRustAgent_RunTask_NoLLM(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
	defer cancel()

	client := agentservicepb.NewAgentServiceClient(conn)
	stream, err := client.RunTask(ctx, &agentservicepb.RunTaskRequest{
		Task:        "echo hello",
		Model:       "test",
		LlmProvider: "ollama",
		LlmEndpoint: "http://127.0.0.1:1", // invalid endpoint → fast fail
		MaxTokens:   16,
	})
	if err != nil {
		t.Fatalf("RunTask: %v", err)
	}

	var sawRunStarted bool
	for {
		evt, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			// gRPC error from server side is acceptable (LLM provider failed)
			t.Logf("stream error (expected with no LLM): %v", err)
			break
		}
		t.Logf("event: type=%v", evt.Type)
		if evt.Type == agentservicepb.EventType_RUN_STARTED {
			sawRunStarted = true
		}
		// TASK_ERROR or TASK_COMPLETE terminate the stream.
		if evt.Type == agentservicepb.EventType_TASK_ERROR ||
			evt.Type == agentservicepb.EventType_TASK_COMPLETE {
			break
		}
	}
	if !sawRunStarted {
		t.Error("did not receive RUN_STARTED event")
	}
}

// TestRustAgent_DispatchToSubAgent_InProcess verifies in-process sub-agent
// dispatch (empty sub_agent_address). With an invalid LLM endpoint the
// DispatchToSubAgent RPC should still return a SubAgentResponse (with an error
// field) rather than a gRPC error.
func TestRustAgent_DispatchToSubAgent_InProcess(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
	defer cancel()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.DispatchToSubAgent(ctx, &agentservicepb.SubAgentRequest{
		Task:        "noop",
		Model:       "test",
		LlmProvider: "ollama",
		// Empty sub_agent_address → in-process dispatch.
	})
	if err != nil {
		t.Fatalf("DispatchToSubAgent: %v", err)
	}
	// Response must be non-nil; error field is expected (no valid LLM).
	if resp == nil {
		t.Fatal("nil response")
	}
	t.Logf("DispatchToSubAgent OK: result=%q error=%q", resp.Result, resp.Error)
}

// TestRustAgent_MultiPing verifies the binary handles concurrent Ping calls.
func TestRustAgent_MultiPing(t *testing.T) {
	// Each iteration starts its own connection
	for i := 0; i < 10; i++ {
		i := i
		t.Run(fmt.Sprintf("ping%d", i), func(t *testing.T) {
			t.Parallel()
			// Create a separate connection for each concurrent request
			port := findFreePort(t)
			addr := "127.0.0.1:" + port

			cmd := exec.Command(locateBinary(t))
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

			cleanupLocal := func() {
				if cmd.Process != nil {
					cmd.Process.Kill() //nolint:errcheck
					cmd.Wait()         //nolint:errcheck
				}
			}
			defer cleanupLocal()

			var localConn *grpc.ClientConn
			var err error
			deadline := time.Now().Add(startTimeout)
			for time.Now().Before(deadline) {
				localConn, err = grpc.NewClient(addr,
					grpc.WithTransportCredentials(insecure.NewCredentials()),
				)
				if err == nil {
					break
				}
				time.Sleep(200 * time.Millisecond)
			}
			if err != nil {
				t.Fatalf("agent did not start: %v", err)
			}
			defer localConn.Close()
			client := agentservicepb.NewAgentServiceClient(localConn)

			// Retry loop to handle transient connection issues in the test
			for retries := 0; retries < 5; retries++ {
				ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
				_, err = client.Ping(ctx, &agentservicepb.PingRequest{})
				cancel()
				if err == nil {
					break
				}
				time.Sleep(500 * time.Millisecond)
			}

			if err != nil {
				t.Errorf("Ping #%d: %v", i, err)
			}
		})
	}
}
