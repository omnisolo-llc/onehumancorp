package pb

import (
	"google.golang.org/protobuf/reflect/protoreflect"
	"google.golang.org/protobuf/runtime/protoimpl"
)

type MeshEvent struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	AgentId   string `protobuf:"bytes,1,opt,name=agent_id,json=agentId,proto3" json:"agent_id,omitempty"`
	Channel   string `protobuf:"bytes,2,opt,name=channel,proto3" json:"channel,omitempty"`
	EventType string `protobuf:"bytes,3,opt,name=event_type,json=eventType,proto3" json:"event_type,omitempty"`
	Payload   []byte `protobuf:"bytes,4,opt,name=payload,proto3" json:"payload,omitempty"`
}

func (x *MeshEvent) ProtoReflect() protoreflect.Message { return nil }
func (x *MeshEvent) ProtoMessage()                       {}
func (x *MeshEvent) Reset()                              {}
func (x *MeshEvent) String() string                      { return "" }
