import re

with open('srcs/server/orchestration/mesh.go', 'r') as f:
    content = f.read()

local_broadcast_event_method = """
func (lm *LocalTeammateMesh) BroadcastEvent(ctx context.Context, channel string, payload map[string]interface{}) error {
    data, err := json.Marshal(payload)
    if err != nil {
        return err
    }
    return lm.BroadcastMeshEvent(ctx, channel, data)
}
"""

local_subscribe_channel_method = """
func (lm *LocalTeammateMesh) SubscribeChannel(ctx context.Context, channel string) (<-chan map[string]interface{}, error) {
    bytesCh, err := lm.SubscribeMeshEvents(ctx, channel)
    if err != nil {
        return nil, err
    }

    ch := make(chan map[string]interface{}, 100)
    go func() {
        for data := range bytesCh {
            var payload map[string]interface{}
            if err := json.Unmarshal(data, &payload); err == nil {
                select {
                case ch <- payload:
                default:
                }
            }
        }
        close(ch)
    }()

    return ch, nil
}
"""

if "func (lm *LocalTeammateMesh) BroadcastEvent" not in content:
    content += "\n" + local_broadcast_event_method

if "func (lm *LocalTeammateMesh) SubscribeChannel" not in content:
    content += "\n" + local_subscribe_channel_method

with open('srcs/server/orchestration/mesh.go', 'w') as f:
    f.write(content)
