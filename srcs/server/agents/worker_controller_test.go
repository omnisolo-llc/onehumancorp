package agents

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestWorkerControllerEnsureProvisionedWaitsForReady(t *testing.T) {
	hub := orchestration.NewHub()
	defer hub.Close()

	controller := &workerController{
		hub:     hub,
		handles: make(map[string]managedWorker),
		opts: workerControllerOptions{
			runtime:      "process",
			hubAddress:   "127.0.0.1:1",
			readyTimeout: time.Second,
		},
	}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	if err := controller.EnsureProvisioned(ctx, orchestration.Agent{ID: "managed-1", Managed: true}); err != nil {
		t.Fatalf("EnsureProvisioned: %v", err)
	}
	defer func() {
		_ = controller.Deprovision(context.Background(), "managed-1")
	}()

	state, ok := hub.WorkerState("managed-1")
	if !ok {
		t.Fatal("expected worker state to be recorded")
	}
	if state.Phase != "READY" {
		t.Fatalf("expected READY worker phase, got %q", state.Phase)
	}
}

func TestWorkerControllerWaitUntilReadyTimesOut(t *testing.T) {
	hub := orchestration.NewHub()
	defer hub.Close()

	controller := &workerController{
		hub: hub,
		opts: workerControllerOptions{
			readyTimeout: 25 * time.Millisecond,
		},
	}

	err := controller.waitUntilReady(context.Background(), "missing-agent")
	if err == nil {
		t.Fatal("expected readiness timeout error")
	}
}

func TestWorkerConfigRoundTripUsesProtoEnvelope(t *testing.T) {
	encoded, err := builtin.EncodeWorkerConfig(workerConfig(orchestration.Agent{
		ID:             "agent-1",
		Name:           "Builder",
		Role:           "SOFTWARE_ENGINEER",
		OrganizationID: "org-1",
		ProviderType:   string(ProviderTypeBuiltin),
		Region:         "process",
	}, "127.0.0.1:9090", "127.0.0.1:50051"))
	if err != nil {
		t.Fatalf("EncodeWorkerConfig: %v", err)
	}

	decoded, err := builtin.DecodeWorkerConfig(encoded)
	if err != nil {
		t.Fatalf("DecodeWorkerConfig: %v", err)
	}
	if decoded.GetAgentId() != "agent-1" {
		t.Fatalf("expected agent-1, got %q", decoded.GetAgentId())
	}
	if decoded.GetHubAddress() != "127.0.0.1:9090" {
		t.Fatalf("unexpected hub address %q", decoded.GetHubAddress())
	}
	if decoded.GetBuiltinAddress() != "127.0.0.1:50051" {
		t.Fatalf("unexpected builtin address %q", decoded.GetBuiltinAddress())
	}
}
