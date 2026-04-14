package main

import (
    "fmt"
    "io/ioutil"
    "strings"
)

func main() {
    content, err := ioutil.ReadFile("srcs/server/orchestration/mesh.go")
    if err != nil {
        fmt.Println("Error reading file:", err)
        return
    }

    strContent := string(content)

    taskSearch := `type Task struct {
	AgentID string ` + "`json:\"agent_id\"`" + `
	Action  string ` + "`json:\"action\"`" + `
	Status  string ` + "`json:\"status\"`" + `
	TaskID  string ` + "`json:\"task_id\"`" + `
}`
    taskReplace := `type Task struct {
	AgentID string ` + "`json:\"agent_id\"`" + `
	Action  string ` + "`json:\"action\"`" + `
	Status  string ` + "`json:\"status\"`" + `
	TaskID  string ` + "`json:\"task_id\"`" + `
}`
    strContent = strings.Replace(strContent, taskSearch, taskReplace, 1)

    teammateMeshSearch := `type TeammateMesh interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
	BroadcastCoordination(ctx context.Context, msg MeshMessage) error
	SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)
}`
    teammateMeshReplace := `type TeammateMesh interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
	BroadcastCoordination(ctx context.Context, msg MeshMessage) error
	SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)
	DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error)
	AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error
}`
    strContent = strings.Replace(strContent, teammateMeshSearch, teammateMeshReplace, 1)

    discoverAgentsImplRedis := `func (rm *RedisMeshTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	return nil, fmt.Errorf("not implemented")
}

`
    strContent = strings.Replace(strContent, "func (rm *RedisMeshTransport) BroadcastTask(ctx context.Context, task Task) error {", discoverAgentsImplRedis+"func (rm *RedisMeshTransport) BroadcastTask(ctx context.Context, task Task) error {", 1)

    discoverAgentsImplLocal := `func (lm *LocalTeammateMesh) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	var agents []pb.Agent
	return agents, nil
}
`
    strContent = strings.Replace(strContent, "func (lm *LocalTeammateMesh) AdvertiseCapabilities", discoverAgentsImplLocal+"func (lm *LocalTeammateMesh) AdvertiseCapabilities", 1)

    err = ioutil.WriteFile("srcs/server/orchestration/mesh.go", []byte(strContent), 0644)
    if err != nil {
        fmt.Println("Error writing file:", err)
    }
}
