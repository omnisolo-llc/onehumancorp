package orchestration

import (
	"context"
	"database/sql"
	_ "github.com/mattn/go-sqlite3"
	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
	agentgrpc "github.com/onehumancorp/mono/src/server_old/agents/grpc"
	"github.com/onehumancorp/mono/src/server/orchestration/mesh"
	"google.golang.org/grpc"
	"net"
	"path/filepath"
	"testing"
)

type MockMeshClient struct{}

func (m *MockMeshClient) Publish(ctx context.Context, topic string, payload []byte) error {
	return nil
}

func (m *MockMeshClient) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (mesh.Subscription, error) {
	return nil, nil
}

func TestMeshClientInterface(t *testing.T) {
	var client MeshClient = &MockMeshClient{}
	if client == nil {
		t.Fatal("client is nil")
	}
}

type mockAgentService struct {
	agentservicepb.UnimplementedAgentServiceServer
}

func (s *mockAgentService) RunTask(req *agentservicepb.RunTaskRequest, stream agentservicepb.AgentService_RunTaskServer) error {
	if req.Model == "cloud-burst-model" {
		stream.Send(&agentservicepb.RunTaskEvent{Type: agentservicepb.EventType_RUN_STARTED})
		stream.Send(&agentservicepb.RunTaskEvent{Type: agentservicepb.EventType_TASK_COMPLETE, Content: "success"})
	}
	return nil
}

func TestTriggerBurst(t *testing.T) {
	tempDir := t.TempDir()
	dbPath := filepath.Join(tempDir, "ohc.db")

	// Create mock sqlite db
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		t.Fatalf("failed to open mock db: %v", err)
	}

	_, err = db.Exec(`CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT NOT NULL,
		payload TEXT NOT NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create mock table: %v", err)
	}

	_, err = db.Exec(`INSERT INTO agent_missions (id, status, payload) VALUES ('test-burst-1', 'PENDING', '{"task":"mock-ai-task"}')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}
	db.Close()

	// Mock gRPC Server
	lis, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("failed to listen: %v", err)
	}
	grpcServer := grpc.NewServer()
	agentservicepb.RegisterAgentServiceServer(grpcServer, &mockAgentService{})
	go func() {
		_ = grpcServer.Serve(lis)
	}()
	defer grpcServer.Stop()

	// Override default address for testing
	oldAddr := agentgrpc.DefaultAddress
	agentgrpc.DefaultAddress = lis.Addr().String()
	defer func() { agentgrpc.DefaultAddress = oldAddr }()

	// Override db DSN
	oldDSN := dbDSN
	dbDSN = dbPath
	defer func() { dbDSN = oldDSN }()

	ctx := context.Background()
	err = TriggerBurst(ctx, "test-burst-1")
	if err != nil {
		t.Fatalf("TriggerBurst failed: %v", err)
	}

	// Verify status updated to BURSTING
	dbCheck, _ := sql.Open("sqlite3", dbPath)
	defer dbCheck.Close()
	var status string
	err = dbCheck.QueryRow("SELECT status FROM agent_missions WHERE id = 'test-burst-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to fetch status: %v", err)
	}
	if status != "BURSTING" {
		t.Fatalf("expected status BURSTING, got %v", status)
	}
}
