package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("srcs/server/telemetry/telemetry.go")
	if err != nil {
		panic(err)
	}

	content := string(b)

	// Add variable declaration
	if !strings.Contains(content, "swarmTasksCompletedCounter metric.Int64Counter") {
		content = strings.Replace(content, "meetingEventsCounter     metric.Int64Counter\n", "meetingEventsCounter     metric.Int64Counter\n\tswarmTasksCompletedCounter metric.Int64Counter\n", 1)
	}

	// Add InitWithMeter
	if !strings.Contains(content, "ohc_swarm_tasks_completed") {
		initBlock := `
	swarmTasksCompletedCounter, err = m.Int64Counter(
		"ohc_swarm_tasks_completed",
		metric.WithDescription("Total swarm tasks completed"),
	)
	if err != nil {
		errs = append(errs, err)
	}
`
		content = strings.Replace(content, "if len(errs) > 0 {\n\t\treturn errs[0]", initBlock+"\n\tif len(errs) > 0 {\n\t\treturn errs[0]", 1)
	}

	// Add method
	if !strings.Contains(content, "RecordSwarmTaskCompleted") {
		methodBlock := `
// RecordSwarmTaskCompleted increments the global counter for completed swarm tasks.
func RecordSwarmTaskCompleted(ctx context.Context, missionID string) {
	if swarmTasksCompletedCounter == nil {
		return
	}
	swarmTasksCompletedCounter.Add(ctx, 1, metric.WithAttributes(
		attribute.String("mission_id", missionID),
	))

	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(map[string]interface{}{
			"mission_id": missionID,
		})
		_ = BufferMetricFunc(ctx, "swarm_task_completed", string(payloadBytes))
	}
}
`
		content = content + "\n" + methodBlock
	}

	err = os.WriteFile("srcs/server/telemetry/telemetry.go", []byte(content), 0644)
	if err != nil {
		panic(err)
	}
	fmt.Println("telemetry.go updated successfully")
}
