package mesh

import (
	"google.golang.org/grpc/metadata"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/redis/go-redis/v9"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	pb "github.com/onehumancorp/mono/api/proto"
)

// setupTestRedis initializes a mock-friendly or real Redis connection for testing.
func setupTestRedis() *redis.Client {
	return redis.NewClient(&redis.Options{
		Addr: "localhost:6379",
	})
}

func TestTeammateMesh_PublishSubscribe(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Ensure Redis is reachable, otherwise skip the test.
	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	mesh := NewTeammateMesh(client)
	topic := "test_topic"

	msgReceived := make(chan MeshMessage, 1)
	handler := func(msg MeshMessage) {
		msgReceived <- msg
	}

	mesh.Subscribe(ctx, topic, handler)

	// Allow subscription to be established
	time.Sleep(100 * time.Millisecond)

	expectedMsg := MeshMessage{
		SenderID: "test_agent",
		Topic:    topic,
		Payload:  json.RawMessage(`{"status":"OK"}`),
	}

	if err := mesh.Publish(ctx, expectedMsg); err != nil {
		t.Fatalf("Publish failed: %v", err)
	}

	select {
	case <-ctx.Done():
		t.Fatal("Timeout waiting for message")
	case msg := <-msgReceived:
		if msg.SenderID != expectedMsg.SenderID {
			t.Errorf("Expected SenderID %s, got %s", expectedMsg.SenderID, msg.SenderID)
		}
		if msg.Topic != expectedMsg.Topic {
			t.Errorf("Expected Topic %s, got %s", expectedMsg.Topic, msg.Topic)
		}
		if string(msg.Payload) != string(expectedMsg.Payload) {
			t.Errorf("Expected Payload %s, got %s", expectedMsg.Payload, msg.Payload)
		}
	}
}

func TestTeammateMesh_Standalone_LocalFallback(t *testing.T) {
	// No Redis client = Standalone mode
	mesh := NewTeammateMesh(nil)
	topic := "local_topic"
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	msgReceived := make(chan MeshMessage, 1)
	mesh.Subscribe(ctx, topic, func(msg MeshMessage) {
		msgReceived <- msg
	})

	expectedMsg := MeshMessage{
		SenderID: "local_agent",
		Topic:    topic,
		Payload:  json.RawMessage(`{"status":"LOCAL_OK"}`),
	}

	err := mesh.Publish(ctx, expectedMsg)
	require.NoError(t, err)

	select {
	case msg := <-msgReceived:
		assert.Equal(t, expectedMsg.SenderID, msg.SenderID)
		assert.Equal(t, expectedMsg.Topic, msg.Topic)
	case <-ctx.Done():
		t.Fatal("Timeout waiting for local message")
	}
}

func TestTeammateMesh_HandlePublish(t *testing.T) {
	mesh := NewTeammateMesh(nil)
	topic := "api_topic"

	msg := MeshMessage{
		SenderID: "api_agent",
		Topic:    topic,
		Payload:  json.RawMessage(`{"data":"test"}`),
	}
	body, _ := json.Marshal(msg)

	req := httptest.NewRequest(http.MethodPost, "/mesh/publish", strings.NewReader(string(body)))
	rr := httptest.NewRecorder()

	mesh.HandlePublish(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)
	var resp map[string]string
	json.Unmarshal(rr.Body.Bytes(), &resp)
	assert.Equal(t, "published", resp["status"])
}

func TestTeammateMesh_HandleSubscribe(t *testing.T) {
	mesh := NewTeammateMesh(nil)
	server := httptest.NewServer(http.HandlerFunc(mesh.HandleSubscribe))
	defer server.Close()

	topic := "ws_topic"
	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "?topic=" + topic

	dialer := websocket.Dialer{}
	conn, _, err := dialer.Dial(wsURL, nil)
	require.NoError(t, err)
	defer conn.Close()

	expectedMsg := MeshMessage{
		SenderID: "ws_agent",
		Topic:    topic,
		Payload:  json.RawMessage(`{"msg":"hello"}`),
	}

	// Publish message after a short delay to allow subscription to register
	go func() {
		time.Sleep(100 * time.Millisecond)
		mesh.Publish(context.Background(), expectedMsg)
	}()

	var receivedMsg MeshMessage
	err = conn.ReadJSON(&receivedMsg)
	require.NoError(t, err)

	assert.Equal(t, expectedMsg.SenderID, receivedMsg.SenderID)
	assert.Equal(t, expectedMsg.Topic, receivedMsg.Topic)
}

func TestTeammateMesh_LockAcquisition(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	// Clean up any existing test lock
	client.Del(ctx, "lock:test_resource")

	mesh := NewTeammateMesh(client)

	// Test 1: Acquire new lock
	req := &pb.LockRequest{
		AgentId:        "test_agent_1",
		TargetResource: "test_resource",
		TtlSeconds:     5,
	}

	resp, err := mesh.AcquireLock(ctx, req)
	require.NoError(t, err)
	assert.True(t, resp.Acquired)

	// Test 2: Fail to acquire existing lock
	req2 := &pb.LockRequest{
		AgentId:        "test_agent_2",
		TargetResource: "test_resource",
		TtlSeconds:     5,
	}

	resp2, err := mesh.AcquireLock(ctx, req2)
	require.NoError(t, err)
	assert.False(t, resp2.Acquired)

	// Test 3: Release lock
	relReq := &pb.ReleaseRequest{
		AgentId:        "test_agent_1",
		TargetResource: "test_resource",
	}
	relResp, err := mesh.ReleaseLock(ctx, relReq)
	require.NoError(t, err)
	assert.True(t, relResp.Success)

	// Test 4: Acquire lock after release
	resp3, err := mesh.AcquireLock(ctx, req2)
	require.NoError(t, err)
	assert.True(t, resp3.Acquired)

	// Cleanup
	client.Del(ctx, "lock:test_resource")
}

// mockStreamAgentState is a mock for pb.CoordinationService_StreamAgentStateServer
type mockStreamAgentState struct {
	ctx      context.Context
	Updates  []*pb.StateUpdate
}

func (m *mockStreamAgentState) Send(update *pb.StateUpdate) error {
	m.Updates = append(m.Updates, update)
	return nil
}

func (m *mockStreamAgentState) SetHeader(metadata.MD) error { return nil }
func (m *mockStreamAgentState) SendHeader(metadata.MD) error { return nil }
func (m *mockStreamAgentState) SetTrailer(metadata.MD) {}
func (m *mockStreamAgentState) Context() context.Context { return m.ctx }
func (m *mockStreamAgentState) SendMsg(m_ interface{}) error { return nil }
func (m *mockStreamAgentState) RecvMsg(m_ interface{}) error { return nil }

func TestTeammateMesh_StreamAgentState(t *testing.T) {
	mesh := NewTeammateMesh(nil)

	ctx, cancel := context.WithCancel(context.Background())

	stream := &mockStreamAgentState{
		ctx: ctx,
	}

	req := &pb.StateStreamRequest{
		DomainFilter: "test_domain",
	}

	// Start streaming in background
	errCh := make(chan error, 1)
	go func() {
		errCh <- mesh.StreamAgentState(req, stream)
	}()

	// Give it time to set up subscription
	time.Sleep(100 * time.Millisecond)

	// Broadcast some states
	err := mesh.BroadcastStateUpdate(context.Background(), "agent_1", "WORKING", "mission_test_domain_123")
	require.NoError(t, err)

	// Should be filtered out
	err = mesh.BroadcastStateUpdate(context.Background(), "agent_2", "IDLE", "mission_other_123")
	require.NoError(t, err)

	time.Sleep(100 * time.Millisecond)
	cancel()

	// Wait for stream to finish
	err = <-errCh
	require.NoError(t, err)

	// Verify we got the correct update
	assert.Len(t, stream.Updates, 1)
	assert.Equal(t, "agent_1", stream.Updates[0].AgentId)
	assert.Equal(t, "WORKING", stream.Updates[0].NewStatus)
}
