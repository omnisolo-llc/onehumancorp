#!/bin/bash
set -e

# Update telemetry bridge
cat << 'INNEREOF' > patch_telemetry_bridge.go
package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("srcs/server/telemetry/telemetry_bridge.go")
	if err != nil {
		panic(err)
	}

	str := string(content)

	if !strings.Contains(str, "\"encoding/json\"") {
		str = strings.Replace(str, "\"os\"", "\"encoding/json\"\n\t\"os\"", 1)
	}

	recordSent := `func RecordBridgeMessageSent(ctx context.Context) {
	if bridgeMessagesSentTotal != nil {
		bridgeMessagesSentTotal.Add(ctx, 1)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"count": 1,
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "ohc_mesh_bridge_messages_sent_total", string(payloadBytes))
	}
}`
	str = replaceFunc(str, "func RecordBridgeMessageSent(ctx context.Context)", recordSent)

	recordReceived := `func RecordBridgeMessageReceived(ctx context.Context) {
	if bridgeMessagesReceivedTotal != nil {
		bridgeMessagesReceivedTotal.Add(ctx, 1)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"count": 1,
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "ohc_mesh_bridge_messages_received_total", string(payloadBytes))
	}
}`
	str = replaceFunc(str, "func RecordBridgeMessageReceived(ctx context.Context)", recordReceived)

	recordStatus := `func RecordBridgeStatus(ctx context.Context, active int64) {
	if bridgeStatusGauge != nil {
		bridgeStatusGauge.Add(ctx, active)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"active": active,
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "ohc_mesh_bridge_status_gauge", string(payloadBytes))
	}
}`
	str = replaceFunc(str, "func RecordBridgeStatus(ctx context.Context, active int64)", recordStatus)

	os.WriteFile("srcs/server/telemetry/telemetry_bridge.go", []byte(str), 0644)
	fmt.Println("Telemetry bridge patched successfully.")
}

func replaceFunc(str, funcSignature, funcReplacement string) string {
	startIdx := strings.Index(str, funcSignature)
	if startIdx == -1 {
		panic("Could not find function: " + funcSignature)
	}
	endIdx := startIdx
	braceCount := 0
	foundBrace := false
	for i := startIdx; i < len(str); i++ {
		if str[i] == '{' {
			braceCount++
			foundBrace = true
		} else if str[i] == '}' {
			braceCount--
		}
		if foundBrace && braceCount == 0 {
			endIdx = i + 1
			break
		}
	}
	return str[:startIdx] + funcReplacement + str[endIdx:]
}
INNEREOF
go run patch_telemetry_bridge.go
rm patch_telemetry_bridge.go
gofmt -w srcs/server/telemetry/telemetry_bridge.go

# Update minimax metrics
cat << 'INNEREOF' > patch_minimax_metrics.go
package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("srcs/server/telemetry/minimax_metrics.go")
	if err != nil {
		panic(err)
	}

	str := string(content)

	if !strings.Contains(str, "\"encoding/json\"") {
		str = strings.Replace(str, "\"go.opentelemetry.io/otel/attribute\"", "\"encoding/json\"\n\t\"go.opentelemetry.io/otel/attribute\"", 1)
	}

	recordMinimax := `// RecordMinimaxCall records metrics for a Minimax API call.
func RecordMinimaxCall(ctx context.Context, operation string, durationSeconds float64, err error) {
	attrs := metric.WithAttributes(attribute.String("operation", operation))

	if minimaxCallsCounter != nil {
		minimaxCallsCounter.Add(ctx, 1, attrs)
	}
	if minimaxLatencyHistogram != nil {
		minimaxLatencyHistogram.Record(ctx, durationSeconds, attrs)
	}
	if err != nil && minimaxErrorsCounter != nil {
		minimaxErrorsCounter.Add(ctx, 1, attrs)
	}

	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"operation":        operation,
			"duration_seconds": durationSeconds,
		}
		if err != nil {
			payloadMap["error"] = err.Error()
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "ohc_minimax_api_latency_seconds", string(payloadBytes))
	}
}`
	str = replaceFunc(str, "func RecordMinimaxCall(ctx context.Context, operation string, durationSeconds float64, err error)", recordMinimax)

	os.WriteFile("srcs/server/telemetry/minimax_metrics.go", []byte(str), 0644)
	fmt.Println("Minimax metrics patched successfully.")
}

func replaceFunc(str, funcSignature, funcReplacement string) string {
	startIdx := strings.Index(str, funcSignature)
	if startIdx == -1 {
		panic("Could not find function: " + funcSignature)
	}
	endIdx := startIdx
	braceCount := 0
	foundBrace := false
	for i := startIdx; i < len(str); i++ {
		if str[i] == '{' {
			braceCount++
			foundBrace = true
		} else if str[i] == '}' {
			braceCount--
		}
		if foundBrace && braceCount == 0 {
			endIdx = i + 1
			break
		}
	}
	return str[:startIdx] + funcReplacement + str[endIdx:]
}
INNEREOF
go run patch_minimax_metrics.go
rm patch_minimax_metrics.go
sed -i '/^\/\/ RecordMinimaxCall records metrics for a Minimax API call.$/d' srcs/server/telemetry/minimax_metrics.go
sed -i 's/func RecordMinimaxCall/\/\/ RecordMinimaxCall records metrics for a Minimax API call.\nfunc RecordMinimaxCall/' srcs/server/telemetry/minimax_metrics.go
gofmt -w srcs/server/telemetry/minimax_metrics.go

# Update rag sync metrics
cat << 'INNEREOF' > patch_rag_sync_metrics_export.go
package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("srcs/server/telemetry/rag_sync_metrics.go")
	if err != nil {
		panic(err)
	}

	str := string(content)

	if !strings.Contains(str, "\"context\"") {
		str = strings.Replace(str, "import (", "import (\n\t\"context\"\n\t\"encoding/json\"", 1)
	}

	newFuncs := `
func RecordRAGRecordsSynced(ctx context.Context, count int64) {
	if RAGRecordsSyncedTotal != nil {
		RAGRecordsSyncedTotal.Add(ctx, count)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"count": count,
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "rag_records_synced_total", string(payloadBytes))
	}
}

func RecordRAGSyncError(ctx context.Context, errStr string) {
	if RAGSyncErrorsTotal != nil {
		RAGSyncErrorsTotal.Add(ctx, 1)
	}
	if BufferMetricFunc != nil {
		payloadMap := map[string]interface{}{
			"error": errStr,
		}
		redacted := RedactInterfacePII(payloadMap)
		payloadBytes, _ := json.Marshal(redacted)
		_ = BufferMetricFunc(ctx, "rag_sync_errors_total", string(payloadBytes))
	}
}
`
	str += newFuncs

	os.WriteFile("srcs/server/telemetry/rag_sync_metrics.go", []byte(str), 0644)
	fmt.Println("RAG sync metrics patched successfully.")
}
INNEREOF
go run patch_rag_sync_metrics_export.go
rm patch_rag_sync_metrics_export.go
gofmt -w srcs/server/telemetry/rag_sync_metrics.go

TIMESTAMP=$(date +%Y-%m-%dT%H-%M-%SZ)
cat << INNEREOF > .agent-task/missions/$TIMESTAMP-done.md
status: DONE
agent: Nova
original_mission: 1776232792787168929.md

**Title:** Missing Standalone BufferMetricFunc in Telemetry Extensions

**Problem Statement:**
Several metric extensions in the OHC backend telemetry module are missing the \`BufferMetricFunc\` instrumentation. This is critical for the "Standalone Mode" (hybrid observability) to buffer metrics locally when disconnected from the cloud.

**Implementation Done:**
Implemented missing \`BufferMetricFunc\` instrumentation in \`telemetry_bridge.go\`, \`minimax_metrics.go\`, and \`rag_sync_metrics.go\` to properly buffer metrics locally when running in Standalone Mode.
INNEREOF

git add srcs/server/telemetry/telemetry_bridge.go srcs/server/telemetry/minimax_metrics.go srcs/server/telemetry/rag_sync_metrics.go -f .agent-task/missions/$TIMESTAMP-done.md
git commit -m "🚀 Nova: Add Standalone BufferMetricFunc in Telemetry Extensions"
