import re

with open('srcs/server/orchestration/mesh.go', 'r') as f:
    content = f.read()

broadcast_event_method = """
func (rm *RedisMeshTransport) BroadcastEvent(ctx context.Context, channel string, payload map[string]interface{}) error {
	data, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel(channel).Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}
"""

subscribe_channel_method = """
func (rm *RedisMeshTransport) SubscribeChannel(ctx context.Context, channel string) (<-chan map[string]interface{}, error) {
	ch := make(chan map[string]interface{}, 100)
	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			var payload map[string]interface{}
			if err := json.Unmarshal([]byte(msg.Message), &payload); err == nil {
				select {
				case ch <- payload:
				default:
					slog.Warn("RedisMeshTransport.SubscribeChannel channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeChannel error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}
"""

if "func (rm *RedisMeshTransport) BroadcastEvent" not in content:
    content += "\n" + broadcast_event_method

if "func (rm *RedisMeshTransport) SubscribeChannel" not in content:
    content += "\n" + subscribe_channel_method

with open('srcs/server/orchestration/mesh.go', 'w') as f:
    f.write(content)
