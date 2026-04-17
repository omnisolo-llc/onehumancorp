with open('srcs/server/agents/builtin/loop.go', 'r') as f:
    content = f.read()

import_telemetry = """	"github.com/onehumancorp/mono/srcs/server/telemetry"
"""

content = content.replace('"context"', '"context"\n' + import_telemetry)

req = """		// Prepare request
		req := ChatRequest{
			Model:       a.Model,
			System:      a.System,
"""

req_with_tracker = """		// Prepare request
		systemPrompt := a.System
		if a.Tracker != nil {
			_, _, _, costUSD := a.Tracker.GetMetrics()
			systemPrompt += fmt.Sprintf("\\n\\n[System] Current Session Cost: $%.4f", costUSD)
		}

		req := ChatRequest{
			Model:       a.Model,
			System:      systemPrompt,
"""

content = content.replace(req, req_with_tracker)

record_usage = """		messages = append(messages, resp.Message)
		totalTurnTokens += resp.Usage.OutputTokens

		if a.Tracker != nil {
			a.Tracker.AddUsage(a.Model, int64(resp.Usage.InputTokens), int64(resp.Usage.OutputTokens), 0)
			_, _, _, costUSD := a.Tracker.GetMetrics()
			telemetry.RecordSessionCost(ctx, a.AgentID, costUSD)
		}
"""

content = content.replace("""		messages = append(messages, resp.Message)
		totalTurnTokens += resp.Usage.OutputTokens""", record_usage)


with open('srcs/server/agents/builtin/loop.go', 'w') as f:
    f.write(content)
