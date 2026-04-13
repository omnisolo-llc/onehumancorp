package builtin

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"strings"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

func EncodeTaskAssignment(assignment *pb.TaskAssignment) (string, error) {
	return encodeProtoEnvelope("task assignment", assignment)
}

func DecodeTaskAssignment(content string) (*pb.TaskAssignment, error) {
	assignment := &pb.TaskAssignment{}
	if err := decodeProtoEnvelope(content, assignment); err == nil {
		return assignment, nil
	}

	var legacy struct {
		IssueID   string `json:"issue_id"`
		IssueName string `json:"issue_name"`
		Directive string `json:"directive"`
		Prompt    string `json:"prompt,omitempty"`
		WorkDir   string `json:"work_dir,omitempty"`
	}
	if err := json.Unmarshal([]byte(content), &legacy); err != nil {
		return nil, fmt.Errorf("decode task assignment: %w", err)
	}
	return pb.TaskAssignment_builder{
		IssueId:   proto.String(legacy.IssueID),
		IssueName: proto.String(legacy.IssueName),
		Directive: proto.String(legacy.Directive),
		Prompt:    proto.String(legacy.Prompt),
		WorkDir:   proto.String(legacy.WorkDir),
	}.Build(), nil
}

func EncodeKillTaskRequest(request *pb.KillTaskRequest) (string, error) {
	return encodeProtoEnvelope("kill task request", request)
}

func DecodeKillTaskRequest(content string) (*pb.KillTaskRequest, error) {
	request := &pb.KillTaskRequest{}
	if err := decodeProtoEnvelope(content, request); err == nil {
		return request, nil
	}

	var legacy struct {
		TaskID string `json:"task_id"`
	}
	if err := json.Unmarshal([]byte(content), &legacy); err != nil {
		return nil, fmt.Errorf("decode kill task request: %w", err)
	}
	return pb.KillTaskRequest_builder{TaskId: proto.String(legacy.TaskID)}.Build(), nil
}

func EncodeTaskResultEnvelope(result *pb.TaskResultEnvelope) (string, error) {
	return encodeProtoEnvelope("task result envelope", result)
}

func DecodeTaskResultEnvelope(content string) (*pb.TaskResultEnvelope, error) {
	result := &pb.TaskResultEnvelope{}
	if err := decodeProtoEnvelope(content, result); err != nil {
		return nil, fmt.Errorf("decode task result envelope: %w", err)
	}
	return result, nil
}

func EncodeWorkerConfig(config *pb.WorkerConfig) (string, error) {
	return encodeProtoEnvelope("worker config", config)
}

func DecodeWorkerConfig(encoded string) (*pb.WorkerConfig, error) {
	config := &pb.WorkerConfig{}
	if err := decodeProtoEnvelope(encoded, config); err == nil {
		return config, nil
	}

	raw, err := base64.StdEncoding.DecodeString(encoded)
	if err != nil {
		return nil, fmt.Errorf("decode worker config: %w", err)
	}
	var legacy struct {
		AgentID        string `json:"agentId"`
		AgentName      string `json:"agentName,omitempty"`
		Role           string `json:"role,omitempty"`
		OrganizationID string `json:"organizationId,omitempty"`
		ProviderType   string `json:"providerType,omitempty"`
		Region         string `json:"region,omitempty"`
		HubAddress     string `json:"hubAddress"`
		BuiltinAddress string `json:"builtinAddress,omitempty"`
	}
	if err := json.Unmarshal(raw, &legacy); err != nil {
		return nil, fmt.Errorf("decode worker config: %w", err)
	}
	return pb.WorkerConfig_builder{
		AgentId:        proto.String(legacy.AgentID),
		AgentName:      proto.String(legacy.AgentName),
		Role:           proto.String(legacy.Role),
		OrganizationId: proto.String(legacy.OrganizationID),
		ProviderType:   proto.String(legacy.ProviderType),
		Region:         proto.String(legacy.Region),
		HubAddress:     proto.String(legacy.HubAddress),
		BuiltinAddress: proto.String(legacy.BuiltinAddress),
	}.Build(), nil
}

func NormalizeWorkerPhase(phase pb.WorkerPhase) string {
	value := strings.TrimPrefix(phase.String(), "WORKER_PHASE_")
	if value == "" || value == "UNSPECIFIED" {
		return "STARTING"
	}
	return value
}

func WorkerPhaseForStatus(status HubStatus) pb.WorkerPhase {
	if status == HubStatusActive {
		return pb.WorkerPhase_WORKER_PHASE_BUSY
	}
	return pb.WorkerPhase_WORKER_PHASE_READY
}

func encodeProtoEnvelope(name string, message proto.Message) (string, error) {
	raw, err := proto.Marshal(message)
	if err != nil {
		return "", fmt.Errorf("marshal %s: %w", name, err)
	}
	return base64.StdEncoding.EncodeToString(raw), nil
}

func decodeProtoEnvelope(encoded string, message proto.Message) error {
	raw, err := base64.StdEncoding.DecodeString(encoded)
	if err != nil {
		return err
	}
	return proto.Unmarshal(raw, message)
}
