package domain

import (
    pb "onehumancorp/src/proto"
    "google.golang.org/protobuf/proto"
)

// Transport represents a generic messaging transport layer.
type Transport interface {
    Publish(topic string, payload []byte) error
    Subscribe(topic string, handler func([]byte)) (func(), error)
}

// TeammateMeshClient provides the Go counterpart to the Rust built-in agent mesh interop.
type TeammateMeshClient struct {
    transport Transport
}

func NewTeammateMeshClient(t Transport) *TeammateMeshClient {
    return &TeammateMeshClient{transport: t}
}

func (c *TeammateMeshClient) PublishTask(payload []byte) error {
    event := &pb.TeammateMeshEvent{
        AgentId: "main_server",
        Action:  "task",
        Status:  "ok",
        Payload: payload,
    }

    buf, err := proto.Marshal(event)
    if err != nil {
        return err
    }

    return c.transport.Publish("mesh:tasks", buf)
}

func (c *TeammateMeshClient) PublishCoordination(payload []byte) error {
    event := &pb.TeammateMeshEvent{
        AgentId: "main_server",
        Action:  "coordination",
        Status:  "ok",
        Payload: payload,
    }

    buf, err := proto.Marshal(event)
    if err != nil {
        return err
    }

    return c.transport.Publish("mesh:coordination", buf)
}

func (c *TeammateMeshClient) SubscribeTasks(handler func([]byte)) (func(), error) {
    return c.transport.Subscribe("mesh:tasks", func(rawMsg []byte) {
        event := &pb.TeammateMeshEvent{}
        if err := proto.Unmarshal(rawMsg, event); err == nil {
            handler(event.Payload)
        } else {
            // Fallback for missing/invalid protobuf wrappers (backward compatibility)
            handler(rawMsg)
        }
    })
}

func (c *TeammateMeshClient) SubscribeCoordination(handler func([]byte)) (func(), error) {
    return c.transport.Subscribe("mesh:coordination", func(rawMsg []byte) {
        event := &pb.TeammateMeshEvent{}
        if err := proto.Unmarshal(rawMsg, event); err == nil {
            handler(event.Payload)
        } else {
            // Fallback for missing/invalid protobuf wrappers (backward compatibility)
            handler(rawMsg)
        }
    })
}
