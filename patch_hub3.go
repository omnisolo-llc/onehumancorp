package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/orchestration/centrifuge_hub.go")
	if err != nil {
		panic(err)
	}

	content := string(b)

	toReplace := `// PublishCoordinationMessage fans out a coordination message to the coordination channel.
func (cn *CentrifugeNode) PublishCoordinationMessage(msg Message) {
	if cn.meshTransport != nil {
		// Convert Message to MeshMessage.
		// Note: We're doing a best-effort mapping here, as Message and MeshMessage
		// have different fields. Adjust mapping if needed.
		meshMsg := MeshMessage{
			AgentID:   msg.ToAgent,
			SenderID:  msg.FromAgent,
			Action:    msg.Type,
			Content:   msg.Content,
		}
		_ = cn.meshTransport.BroadcastCoordination(context.Background(), meshMsg)
	}
	channel := "mesh:coordination"
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal coordination message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}`

	newContent := `// PublishCoordinationMessage fans out a coordination message to the coordination channel.
func (cn *CentrifugeNode) PublishCoordinationMessage(msg Message) {
	if cn.meshTransport != nil {
		meshMsg := MeshMessage{
			AgentID:   msg.ToAgent,
			SenderID:  msg.FromAgent,
			Action:    msg.Type,
			Content:   msg.Content,
		}
		_ = cn.meshTransport.BroadcastCoordination(context.Background(), meshMsg)
	}
	channel := "mesh:coordination"
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal coordination message", "error", err)
		return
	}
	_, _ = cn.node.Publish(channel, data)
}`

	if !strings.Contains(content, toReplace) {
		fmt.Println("toReplace not found!")
	} else {
		content = strings.Replace(content, toReplace, newContent, 1)
		err = ioutil.WriteFile("srcs/server/orchestration/centrifuge_hub.go", []byte(content), 0644)
		if err != nil {
			panic(err)
		}
		fmt.Println("centrifuge_hub.go updated!")
	}
}
