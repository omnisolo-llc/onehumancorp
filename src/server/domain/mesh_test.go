package domain

import (
    "testing"
    "bytes"

    pb "onehumancorp/src/proto"
    "google.golang.org/protobuf/proto"
)

type MockTransport struct {
    lastTopic   string
    lastPayload []byte
    handler     func([]byte)
}

func (m *MockTransport) Publish(topic string, payload []byte) error {
    m.lastTopic = topic
    m.lastPayload = payload
    if m.handler != nil {
        m.handler(payload)
    }
    return nil
}

func (m *MockTransport) Subscribe(topic string, handler func([]byte)) (func(), error) {
    m.handler = handler
    return func() {}, nil
}

func TestTeammateMeshClient_PublishTask(t *testing.T) {
    mock := &MockTransport{}
    client := NewTeammateMeshClient(mock)

    payload := []byte("test_task_payload")
    err := client.PublishTask(payload)
    if err != nil {
        t.Fatalf("PublishTask failed: %v", err)
    }

    if mock.lastTopic != "mesh:tasks" {
        t.Errorf("Expected topic 'mesh:tasks', got '%s'", mock.lastTopic)
    }

    event := &pb.TeammateMeshEvent{}
    if err := proto.Unmarshal(mock.lastPayload, event); err != nil {
        t.Fatalf("Failed to unmarshal payload: %v", err)
    }

    if event.Action != "task" {
        t.Errorf("Expected action 'task', got '%s'", event.Action)
    }
    if event.AgentId != "main_server" {
        t.Errorf("Expected agent_id 'main_server', got '%s'", event.AgentId)
    }
    if !bytes.Equal(event.Payload, payload) {
        t.Errorf("Expected payload '%s', got '%s'", string(payload), string(event.Payload))
    }
}

func TestTeammateMeshClient_PublishCoordination(t *testing.T) {
    mock := &MockTransport{}
    client := NewTeammateMeshClient(mock)

    payload := []byte("test_coord_payload")
    err := client.PublishCoordination(payload)
    if err != nil {
        t.Fatalf("PublishCoordination failed: %v", err)
    }

    if mock.lastTopic != "mesh:coordination" {
        t.Errorf("Expected topic 'mesh:coordination', got '%s'", mock.lastTopic)
    }

    event := &pb.TeammateMeshEvent{}
    if err := proto.Unmarshal(mock.lastPayload, event); err != nil {
        t.Fatalf("Failed to unmarshal payload: %v", err)
    }

    if event.Action != "coordination" {
        t.Errorf("Expected action 'coordination', got '%s'", event.Action)
    }
    if event.AgentId != "main_server" {
        t.Errorf("Expected agent_id 'main_server', got '%s'", event.AgentId)
    }
    if !bytes.Equal(event.Payload, payload) {
        t.Errorf("Expected payload '%s', got '%s'", string(payload), string(event.Payload))
    }
}

func TestTeammateMeshClient_SubscribeTasks_ValidEvent(t *testing.T) {
    mock := &MockTransport{}
    client := NewTeammateMeshClient(mock)

    var receivedPayload []byte
    _, err := client.SubscribeTasks(func(p []byte) {
        receivedPayload = p
    })
    if err != nil {
        t.Fatalf("SubscribeTasks failed: %v", err)
    }

    event := &pb.TeammateMeshEvent{
        AgentId: "other_agent",
        Action:  "task",
        Status:  "ok",
        Payload: []byte("inner_task_payload"),
    }
    buf, _ := proto.Marshal(event)

    mock.handler(buf)

    if !bytes.Equal(receivedPayload, []byte("inner_task_payload")) {
        t.Errorf("Expected received payload 'inner_task_payload', got '%s'", string(receivedPayload))
    }
}

func TestTeammateMeshClient_SubscribeTasks_Fallback(t *testing.T) {
    mock := &MockTransport{}
    client := NewTeammateMeshClient(mock)

    var receivedPayload []byte
    _, err := client.SubscribeTasks(func(p []byte) {
        receivedPayload = p
    })
    if err != nil {
        t.Fatalf("SubscribeTasks failed: %v", err)
    }

    rawPayload := []byte("invalid_protobuf_data")
    mock.handler(rawPayload)

    if !bytes.Equal(receivedPayload, rawPayload) {
        t.Errorf("Expected fallback to raw payload '%s', got '%s'", string(rawPayload), string(receivedPayload))
    }
}
