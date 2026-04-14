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

    // Fix the mess in RedisMeshTransport capabilities functions
    badCode := `func (rm *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	data, err := json.Marshal(caps)
	if err != nil {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:capabilities").Build(), func(msg rueidis.PubSubMessage) {
			var c pb.AgentCapabilities
			if err := json.Unmarshal([]byte(msg.Message), &c); err == nil {
				select {
				case ch <- c:
				default:
					slog.Warn("RedisMeshTransport.SubscribeCapabilities channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeCapabilities error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}`

    goodCode := `func (rm *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	data, err := json.Marshal(caps)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:capabilities").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

func (rm *RedisMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {
	ch := make(chan pb.AgentCapabilities, 100)
	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:capabilities").Build(), func(msg rueidis.PubSubMessage) {
			var c pb.AgentCapabilities
			if err := json.Unmarshal([]byte(msg.Message), &c); err == nil {
				select {
				case ch <- c:
				default:
					slog.Warn("RedisMeshTransport.SubscribeCapabilities channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeCapabilities error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}`

    strContent = strings.Replace(strContent, badCode, goodCode, 1)


    err = ioutil.WriteFile("srcs/server/orchestration/mesh.go", []byte(strContent), 0644)
    if err != nil {
        fmt.Println("Error writing file:", err)
    }
}
