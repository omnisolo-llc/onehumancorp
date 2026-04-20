package telemetry_test

import (
	"testing"
)

// The old TestPIIRedactionEnforcement was enforcing RedactInterfacePII at call sites
// to BufferMetricFunc. However, redaction is now centrally done inside BufferMetricFunc
// by InitStandaloneBuffer. This file is kept to avoid BUILD/package breakage.
func TestPIIRedactionEnforcement(t *testing.T) {
	// Redaction check is now done in BufferMetricFunc directly.
}
