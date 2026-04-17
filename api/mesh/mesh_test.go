package mesh

import (
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


func TestTeammateMesh_Mailbox(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	mesh := NewTeammateMesh(client)
	agentID := "agent_123"

	// Cleanup before test
	client.Del(ctx, "mesh:mailbox:"+agentID)

	msg1 := MeshMessage{SenderID: "sender1", Topic: "t1", Payload: json.RawMessage(`"p1"`)}
	msg2 := MeshMessage{SenderID: "sender2", Topic: "t2", Payload: json.RawMessage(`"p2"`)}

	require.NoError(t, mesh.SendToMailbox(ctx, agentID, msg1))
	require.NoError(t, mesh.SendToMailbox(ctx, agentID, msg2))

	messages, err := mesh.GetMailbox(ctx, agentID)
	require.NoError(t, err)
	assert.Len(t, messages, 2)
	assert.Equal(t, msg1.SenderID, messages[0].SenderID)
	assert.Equal(t, msg2.SenderID, messages[1].SenderID)

	// Fetch again to ensure it's cleared
	messages2, err := mesh.GetMailbox(ctx, agentID)
	require.NoError(t, err)
	assert.Len(t, messages2, 0)
}

func TestTeammateMesh_Standalone_Mailbox(t *testing.T) {
	mesh := NewTeammateMesh(nil)
	agentID := "agent_local"
	ctx := context.Background()

	msg1 := MeshMessage{SenderID: "sender1", Topic: "t1", Payload: json.RawMessage(`"p1"`)}
	msg2 := MeshMessage{SenderID: "sender2", Topic: "t2", Payload: json.RawMessage(`"p2"`)}

	require.NoError(t, mesh.SendToMailbox(ctx, agentID, msg1))
	require.NoError(t, mesh.SendToMailbox(ctx, agentID, msg2))

	messages, err := mesh.GetMailbox(ctx, agentID)
	require.NoError(t, err)
	assert.Len(t, messages, 2)
	assert.Equal(t, msg1.SenderID, messages[0].SenderID)
	assert.Equal(t, msg2.SenderID, messages[1].SenderID)

	messages2, err := mesh.GetMailbox(ctx, agentID)
	require.NoError(t, err)
	assert.Len(t, messages2, 0)
}

func TestTeammateMesh_HandleMailbox(t *testing.T) {
	mesh := NewTeammateMesh(nil)
	agentID := "agent_api"

	// Test POST to mailbox
	msg := MeshMessage{
		SenderID: "sender_api",
		Topic:    "api_topic",
		Payload:  json.RawMessage(`{"hello":"world"}`),
	}
	body, _ := json.Marshal(msg)

	reqPost := httptest.NewRequest(http.MethodPost, "/mesh/mailbox?agent_id="+agentID, strings.NewReader(string(body)))
	rrPost := httptest.NewRecorder()
	mesh.HandleMailbox(rrPost, reqPost)

	assert.Equal(t, http.StatusOK, rrPost.Code)
	var respPost map[string]string
	json.Unmarshal(rrPost.Body.Bytes(), &respPost)
	assert.Equal(t, "sent to mailbox", respPost["status"])

	// Test GET from mailbox
	reqGet := httptest.NewRequest(http.MethodGet, "/mesh/mailbox?agent_id="+agentID, nil)
	rrGet := httptest.NewRecorder()
	mesh.HandleMailbox(rrGet, reqGet)

	assert.Equal(t, http.StatusOK, rrGet.Code)
	var respGet struct {
		Messages []MeshMessage `json:"messages"`
	}
	json.Unmarshal(rrGet.Body.Bytes(), &respGet)
	require.Len(t, respGet.Messages, 1)
	assert.Equal(t, msg.SenderID, respGet.Messages[0].SenderID)

	// Test GET again (should be empty)
	reqGet2 := httptest.NewRequest(http.MethodGet, "/mesh/mailbox?agent_id="+agentID, nil)
	rrGet2 := httptest.NewRecorder()
	mesh.HandleMailbox(rrGet2, reqGet2)
	assert.Equal(t, http.StatusOK, rrGet2.Code)

	var respGet2 struct {
		Messages []MeshMessage `json:"messages"`
	}
	json.Unmarshal(rrGet2.Body.Bytes(), &respGet2)
	assert.Len(t, respGet2.Messages, 0)
}
