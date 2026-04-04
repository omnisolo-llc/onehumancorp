import re

with open('srcs/server/dashboard/server.go', 'r') as f:
    content = f.read()

# Add tracking to handleMeshBroadcast
replacement = """
	err := s.hub.Publish(orchestration.Message{
		ID:        fmt.Sprintf("%d", time.Now().UnixNano()),
		FromAgent: "system",
		ToAgent:   "system",
		Type:      req.Channel,
		Content:   req.Payload,
	})

	if telemetry.MeshMessagesBroadcastCounter != nil {
		telemetry.MeshMessagesBroadcastCounter.Add(r.Context(), 1)
	}
"""

content = re.sub(
    r'err := s\.hub\.Publish\(orchestration\.Message\{\s*ID:\s*fmt\.Sprintf\("%d", time\.Now\(\)\.UnixNano\(\)\),\s*FromAgent:\s*"system",\s*ToAgent:\s*"system",\s*Type:\s*req\.Channel,\s*Content:\s*req\.Payload,\s*\}\)',
    replacement.strip(),
    content
)

with open('srcs/server/dashboard/server.go', 'w') as f:
    f.write(content)
