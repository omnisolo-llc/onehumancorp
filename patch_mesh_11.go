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

    // Put back RedisMeshTransport.AdvertiseCapabilities
    advertiseCapsCode := `func (rm *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	data, err := json.Marshal(caps)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:capabilities").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

`

    // Find func (rm *RedisMeshTransport) SubscribeCapabilities and put it before
    strContent = strings.Replace(strContent, "func (rm *RedisMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {", advertiseCapsCode+"func (rm *RedisMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {", 1)

    // Then delete the dummy one
    dummyCode := `func (rm *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	return fmt.Errorf("not implemented")
}

`
    strContent = strings.Replace(strContent, dummyCode, "", 1)


    err = ioutil.WriteFile("srcs/server/orchestration/mesh.go", []byte(strContent), 0644)
    if err != nil {
        fmt.Println("Error writing file:", err)
    }
}
