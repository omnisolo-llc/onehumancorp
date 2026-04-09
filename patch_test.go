package main

import (
	"os"
	"strings"
)

func main() {
	content, _ := os.ReadFile("srcs/server/hub/rag_sync_test.go")
	str := string(content)

	str = strings.ReplaceAll(str, "INSERT INTO autodream_memories (id, content, sync_status)", "INSERT INTO autodream_memories (id, organization_id, agent_id, source_type, content, sync_status)")
	str = strings.ReplaceAll(str, "VALUES ('id1', 'content 1', 'pending')", "VALUES ('id1', 'org1', 'agent1', 'src', 'content 1', 'pending')")
	str = strings.ReplaceAll(str, "VALUES ('id2', 'content 2', 'synced')", "VALUES ('id2', 'org1', 'agent1', 'src', 'content 2', 'synced')")
	str = strings.ReplaceAll(str, "VALUES ('id3', 'content 3', 'pending')", "VALUES ('id3', 'org1', 'agent1', 'src', 'content 3', 'pending')")

	os.WriteFile("srcs/server/hub/rag_sync_test.go", []byte(str), 0644)
}
