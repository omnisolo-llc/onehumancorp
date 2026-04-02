package main

import (
	"bytes"
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("srcs/server/telemetry/telemetry_test.go")
	if err != nil {
		panic(err)
	}

	content := string(b)

	// Add to verify globals
	if !strings.Contains(content, "swarmTasksCompletedCounter to be initialized") {
		checkBlock := `if meetingEventsCounter == nil {
		t.Error("expected meetingEventsCounter to be initialized")
	}
	if swarmTasksCompletedCounter == nil {
		t.Error("expected swarmTasksCompletedCounter to be initialized")
	}`
		content = strings.Replace(content, "if meetingEventsCounter == nil {\n\t\tt.Error(\"expected meetingEventsCounter to be initialized\")\n\t}", checkBlock, 1)
	}

	// Add to TestTelemetryMetricErrors
	if !strings.Contains(content, "originalSwarmTasksCompletedCounter") {
		content = strings.Replace(content, "originalMeetingEventsCounter := meetingEventsCounter\n", "originalMeetingEventsCounter := meetingEventsCounter\n\toriginalSwarmTasksCompletedCounter := swarmTasksCompletedCounter\n", 1)
		content = strings.Replace(content, "meetingEventsCounter = originalMeetingEventsCounter\n\t}()", "meetingEventsCounter = originalMeetingEventsCounter\n\t\tswarmTasksCompletedCounter = originalSwarmTasksCompletedCounter\n\t}()", 1)
	}

	// Add RecordSwarmTaskCompleted test
	if !strings.Contains(content, "RecordSwarmTaskCompleted(ctx") {
		testBlock := `
	t.Run("RecordSwarmTaskCompleted", func(t *testing.T) {
		RecordSwarmTaskCompleted(ctx, "mission-123")
	})
`
		content = strings.Replace(content, "t.Run(\"RecordTokenBurnRate\", func(t *testing.T) {\n\t\tRecordTokenBurnRate(ctx, \"acme-org\", 123.45)\n\t})\n}", "t.Run(\"RecordTokenBurnRate\", func(t *testing.T) {\n\t\tRecordTokenBurnRate(ctx, \"acme-org\", 123.45)\n\t})\n"+testBlock+"}", 1)
	}

	// Add Uninitialized test
	if !strings.Contains(content, "originalSwarmTasksCompletedCounter := swarmTasksCompletedCounter") {
		content = strings.Replace(content, "originalMeetingEventsCounter := meetingEventsCounter\n\toriginalTokenBurnRateGauge := tokenBurnRateGauge\n", "originalMeetingEventsCounter := meetingEventsCounter\n\toriginalTokenBurnRateGauge := tokenBurnRateGauge\n\toriginalSwarmTasksCompletedCounter := swarmTasksCompletedCounter\n", 1)
		content = strings.Replace(content, "meetingEventsCounter = nil\n\ttokenBurnRateGauge = nil\n", "meetingEventsCounter = nil\n\ttokenBurnRateGauge = nil\n\tswarmTasksCompletedCounter = nil\n", 1)
		content = strings.Replace(content, "meetingEventsCounter = originalMeetingEventsCounter\n\t\ttokenBurnRateGauge = originalTokenBurnRateGauge\n\t}()", "meetingEventsCounter = originalMeetingEventsCounter\n\t\ttokenBurnRateGauge = originalTokenBurnRateGauge\n\t\tswarmTasksCompletedCounter = originalSwarmTasksCompletedCounter\n\t}()", 1)

		testBlock2 := `
	t.Run("RecordSwarmTaskCompleted Uninitialized", func(t *testing.T) {
		RecordSwarmTaskCompleted(ctx, "mission-123")
	})
`
		content = strings.Replace(content, "t.Run(\"RecordTokenBurnRate Uninitialized\", func(t *testing.T) {\n\t\tRecordTokenBurnRate(ctx, \"acme-org\", 123.45)\n\t})\n}", "t.Run(\"RecordTokenBurnRate Uninitialized\", func(t *testing.T) {\n\t\tRecordTokenBurnRate(ctx, \"acme-org\", 123.45)\n\t})\n"+testBlock2+"}", 1)
	}

	err = os.WriteFile("srcs/server/telemetry/telemetry_test.go", []byte(content), 0644)
	if err != nil {
		panic(err)
	}
	fmt.Println("telemetry_test.go updated successfully")
}
