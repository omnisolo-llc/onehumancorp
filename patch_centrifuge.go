package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/orchestration/centrifuge_hub.go"
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)

	oldPublishCoordinationMessage := `func (cn *CentrifugeNode) PublishCoordinationMessage(msg Message) {
	if cn.meshTransport != nil {
		data, err := json.Marshal(msg)
		if err == nil {
			_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "coordination", data)
		}
	}
	channel := "mesh:coordination"
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal coordination message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}`

	newPublishCoordinationMessage := `func (cn *CentrifugeNode) PublishCoordinationMessage(msg Message) {
	channel := "mesh:coordination"
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal coordination message", "error", err)
		return
	}
	if cn.meshTransport != nil {
		_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "coordination", data)
	}
	_, _ = cn.node.Publish(channel, data)
}`

	strContent = strings.Replace(strContent, oldPublishCoordinationMessage, newPublishCoordinationMessage, 1)

	oldPublishMeetingMessage := `func (cn *CentrifugeNode) PublishMeetingMessage(meetingID string, msg Message) {
	if cn.meshTransport != nil {
		data, err := json.Marshal(msg)
		if err == nil {
			_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "meeting:"+meetingID, data)
		}
	}
	channel := "meeting:" + meetingID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal meeting message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}`

	newPublishMeetingMessage := `func (cn *CentrifugeNode) PublishMeetingMessage(meetingID string, msg Message) {
	channel := "meeting:" + meetingID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal meeting message", "error", err)
		return
	}
	if cn.meshTransport != nil {
		_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), channel, data)
	}
	_, _ = cn.node.Publish(channel, data)
}`

	strContent = strings.Replace(strContent, oldPublishMeetingMessage, newPublishMeetingMessage, 1)

	oldPublishChatMessage := `func (cn *CentrifugeNode) PublishChatMessage(roomID string, msg Message) {
	if cn.meshTransport != nil {
		data, err := json.Marshal(msg)
		if err == nil {
			_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "chat:"+roomID, data)
		}
	}
	channel := "chat:" + roomID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal chat message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}`

	newPublishChatMessage := `func (cn *CentrifugeNode) PublishChatMessage(roomID string, msg Message) {
	channel := "chat:" + roomID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal chat message", "error", err)
		return
	}
	if cn.meshTransport != nil {
		_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), channel, data)
	}
	_, _ = cn.node.Publish(channel, data)
}`

	strContent = strings.Replace(strContent, oldPublishChatMessage, newPublishChatMessage, 1)

	oldPublishAgentNotification := `func (cn *CentrifugeNode) PublishAgentNotification(agentID string, msg Message) {
	if cn.meshTransport != nil {
		data, err := json.Marshal(msg)
		if err == nil {
			_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), "agent:"+agentID, data)
		}
	}
	channel := "agent:" + agentID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal agent notification", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}`

	newPublishAgentNotification := `func (cn *CentrifugeNode) PublishAgentNotification(agentID string, msg Message) {
	channel := "agent:" + agentID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal agent notification", "error", err)
		return
	}
	if cn.meshTransport != nil {
		_ = cn.meshTransport.BroadcastMeshEvent(context.Background(), channel, data)
	}
	_, _ = cn.node.Publish(channel, data)
}`

	strContent = strings.Replace(strContent, oldPublishAgentNotification, newPublishAgentNotification, 1)

	err = ioutil.WriteFile(filePath, []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}
	fmt.Println("Successfully patched centrifuge_hub.go")
}
