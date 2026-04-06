package main

import (
	"os"
	"strings"
)

func main() {
	content, _ := os.ReadFile("srcs/server/orchestration/mesh.go")
	newContent := strings.Replace(string(content), `		AgentID:   msg.AgentID,
		Content:   msg.Content,
		Role:      msg.Role,
		Timestamp: msg.Timestamp,`, `		Content:   []byte(msg.Content),`, 1)

	// I had an issue previously where `[]byte(msg.Content)` couldn't be used as string.
	// Oh, `Message` expects Content as string or []byte? Let's check `Message` struct in `centrifuge_hub.go` or somewhere.
	// Wait, last time the error was "cannot use []byte(msg.Content) (value of type []byte) as string value in struct literal"
	// So `Message` expects `Content` to be a string!
	// But `Message` struct only has `ID` and `Content`? Oh, in `patch_queue_and_mesh.go`, I replaced it with `Content: msg.Content`.
	newContent = strings.Replace(string(content), `		ID:        msg.AgentID,
		AgentID:   msg.AgentID,
		Content:   msg.Content,
		Role:      msg.Role,
		Timestamp: msg.Timestamp,`, `		ID:        msg.AgentID,
		Content:   msg.Content,`, 1)
	os.WriteFile("srcs/server/orchestration/mesh.go", []byte(newContent), 0644)
}
