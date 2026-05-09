package telemetry

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

// Ensure compliance in multi-tenant contexts by checking for cross-tenant PII logging.
func TestComplianceGuardrails(t *testing.T) {
	t.Run("RedactInterfacePII comprehensively", func(t *testing.T) {
		attrs := map[string]interface{}{
			"tenant_id":       "tenant-1",
			"organization_id": "org-1",
			"email":           "test@example.com",
			"phone":           "123-456-7890",
			"token":           "abcd",
			"nested": map[string]interface{}{
				"ssn": "000-00-0000",
			},
		}

		// In telemetry buffer_integration.go it's redactInterfacePII
		redacted, ok := redactInterfacePII(attrs).(map[string]interface{})
		assert.True(t, ok)

		assert.Equal(t, "[REDACTED]", redacted["email"])
		assert.Equal(t, "[REDACTED]", redacted["phone"])
		assert.Equal(t, "[REDACTED]", redacted["token"])
		assert.Equal(t, "[REDACTED]", redacted["tenant_id"])
		assert.Equal(t, "[REDACTED]", redacted["organization_id"])

		nested := redacted["nested"].(map[string]interface{})
		assert.Equal(t, "[REDACTED]", nested["ssn"])
	})
}
