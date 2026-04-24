package orchestration

import (
    "context"
    "encoding/json"
    "testing"
    "time"

    "github.com/onehumancorp/mono/src/server/db"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
)

func TestMemoryMeshTransport_TeammateMesh(t *testing.T) {
    provider, err := db.NewSQLiteProvider(":memory:")
    require.NoError(t, err)

    mesh := NewMemoryMeshTransport(provider)
    ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()

    topic := "teammate_mesh"
    eventsChan, err := mesh.SubscribeTeammateMesh(ctx, topic)
    require.NoError(t, err)

    payload := map[string]interface{}{"key": "value"}
    payloadBytes, _ := json.Marshal(payload)

    err = mesh.PublishTeammateMeshEvent(ctx, topic, "agent-123", "START", "ACTIVE", payloadBytes)
    require.NoError(t, err)

    select {
    case receivedBytes := <-eventsChan:
        var receivedMsg map[string]interface{}
        err := json.Unmarshal(receivedBytes, &receivedMsg)
        require.NoError(t, err)

        assert.Equal(t, "agent-123", receivedMsg["agent_id"])
        assert.Equal(t, "START", receivedMsg["action"])
        assert.Equal(t, "ACTIVE", receivedMsg["status"])

        // Extract payload
        receivedPayloadBytes, _ := json.Marshal(receivedMsg["payload"])
        var receivedPayload map[string]interface{}
        json.Unmarshal(receivedPayloadBytes, &receivedPayload)

        assert.Equal(t, "value", receivedPayload["key"])
    case <-time.After(2 * time.Second):
        t.Fatal("timed out waiting for teammate mesh event message")
    }
}

func TestMemoryMeshTransport_PublishSubscribe(t *testing.T) {
    provider, err := db.NewSQLiteProvider(":memory:")
    require.NoError(t, err)

    mesh := NewMemoryMeshTransport(provider)
    _, cancel := context.WithTimeout(context.Background(), 5*time.Second)
    defer cancel()

    topic := "general_topic"
    eventsChan, err := mesh.Subscribe(topic)
    require.NoError(t, err)

    payload := []byte("hello world")

    err = mesh.Publish(topic, payload)
    require.NoError(t, err)

    select {
    case receivedBytes := <-eventsChan:
        assert.Equal(t, payload, receivedBytes)
    case <-time.After(2 * time.Second):
        t.Fatal("timed out waiting for publish message")
    }
}
