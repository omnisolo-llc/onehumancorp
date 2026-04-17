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
	mesh := NewTeammateMesh(nil)
	agentID := "test_agent_box"

	msgReceived := make(chan MeshMessage, 1)
	mesh.SubscribeMailbox(context.Background(), agentID, func(msg MeshMessage) {
		msgReceived <- msg
	})

	expectedMsg := MeshMessage{
		SenderID: "sender_agent",
		Payload:  json.RawMessage(`{"status":"MAILBOX_OK"}`),
	}

	err := mesh.SendDirectMessage(context.Background(), agentID, expectedMsg)
	require.NoError(t, err)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	select {
	case msg := <-msgReceived:
		assert.Equal(t, expectedMsg.SenderID, msg.SenderID)
		assert.Equal(t, "mailbox:"+agentID, msg.Topic)
	case <-ctx.Done():
		t.Fatal("Timeout waiting for mailbox message")
	}
}

func TestTeammateMesh_HandleMailbox(t *testing.T) {
	mesh := NewTeammateMesh(nil)
	agentID := "test_poll_agent"

	// Send a direct message
	expectedMsg := MeshMessage{
		SenderID: "sender1",
		Payload:  json.RawMessage(`{"status":"QUEUED"}`),
	}
	err := mesh.SendDirectMessage(context.Background(), agentID, expectedMsg)
	require.NoError(t, err)

	req := httptest.NewRequest(http.MethodGet, "/mesh/mailbox?agent_id="+agentID, nil)
	rr := httptest.NewRecorder()

	mesh.HandleMailbox(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	var resp struct {
		Messages []MeshMessage `json:"messages"`
	}
	err = json.Unmarshal(rr.Body.Bytes(), &resp)
	require.NoError(t, err)
	assert.NotNil(t, resp.Messages)
	assert.Len(t, resp.Messages, 1)
	assert.Equal(t, expectedMsg.SenderID, resp.Messages[0].SenderID)

	// Second poll should be empty
	req2 := httptest.NewRequest(http.MethodGet, "/mesh/mailbox?agent_id="+agentID, nil)
	rr2 := httptest.NewRecorder()
	mesh.HandleMailbox(rr2, req2)
	assert.Equal(t, http.StatusOK, rr2.Code)
	var resp2 struct {
		Messages []MeshMessage `json:"messages"`
	}
	err = json.Unmarshal(rr2.Body.Bytes(), &resp2)
	require.NoError(t, err)
	assert.Empty(t, resp2.Messages)
}

func TestTeammateMesh_HandleMailbox_Redis(t *testing.T) {
	client := setupTestRedis()
	defer client.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	if err := client.Ping(ctx).Err(); err != nil {
		t.Skipf("Redis not reachable, skipping test: %v", err)
	}

	mesh := NewTeammateMesh(client)
	agentID := "test_redis_poll_agent"

	// Clear any existing state
	client.Del(ctx, "durable_mailbox:"+agentID)

	expectedMsg := MeshMessage{
		SenderID: "sender2",
		Payload:  json.RawMessage(`{"status":"REDIS_QUEUED"}`),
	}
	err := mesh.SendDirectMessage(ctx, agentID, expectedMsg)
	require.NoError(t, err)

	req := httptest.NewRequest(http.MethodGet, "/mesh/mailbox?agent_id="+agentID, nil)
	rr := httptest.NewRecorder()

	mesh.HandleMailbox(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	var resp struct {
		Messages []MeshMessage `json:"messages"`
	}
	err = json.Unmarshal(rr.Body.Bytes(), &resp)
	require.NoError(t, err)
	assert.NotNil(t, resp.Messages)
	assert.Len(t, resp.Messages, 1)
	assert.Equal(t, expectedMsg.SenderID, resp.Messages[0].SenderID)

	// Second poll should be empty
	req2 := httptest.NewRequest(http.MethodGet, "/mesh/mailbox?agent_id="+agentID, nil)
	rr2 := httptest.NewRecorder()
	mesh.HandleMailbox(rr2, req2)
	assert.Equal(t, http.StatusOK, rr2.Code)
	var resp2 struct {
		Messages []MeshMessage `json:"messages"`
	}
	err = json.Unmarshal(rr2.Body.Bytes(), &resp2)
	require.NoError(t, err)
	assert.Empty(t, resp2.Messages)
}
