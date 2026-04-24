package orchestration

import (
	"encoding/json"
	"strings"
	"testing"
)

// TestSanitizeHubEvent_EnforcesPIIRedaction acts as a compliance guardrail ensuring
// that any multi-tenant events processed via the Hub are strictly redacted of PII
// before being marshaled to JSON.
func TestSanitizeHubEvent_EnforcesPIIRedaction(t *testing.T) {
	// A raw payload simulating an unredacted Hub event with PII (email, phone, credit card).
	rawPayload := map[string]interface{}{
		"type":    "customer.contact",
		"message": "Please call user at test.user@example.com or phone 123-456-7890. Card: 1234-5678-9012-3456",
		"nested": map[string]interface{}{
			"ssn": "987-65-4321",
		},
	}

	event, err := sanitizeHubEvent(rawPayload)
	if err != nil {
		t.Fatalf("sanitizeHubEvent failed: %v", err)
	}

	// Unmarshal the payload back to check redaction
	var parsed map[string]interface{}
	if err := json.Unmarshal(event.Payload, &parsed); err != nil {
		t.Fatalf("failed to unmarshal redacted payload: %v", err)
	}

	// Verify standard fields
	if event.Type != "customer.contact" {
		t.Errorf("expected type 'customer.contact', got %q", event.Type)
	}

	payloadStr := string(event.Payload)

	// Verify that the PII values DO NOT exist in the serialized JSON
	forbiddenStrings := []string{
		"test.user@example.com",
		"123-456-7890",
		"1234-5678-9012-3456",
		"987-65-4321",
	}

	for _, str := range forbiddenStrings {
		if strings.Contains(payloadStr, str) {
			t.Errorf("Compliance Violation: PII leaked in serialized JSON payload: %q", str)
		}
	}

	// Verify that the redaction markers DO exist
	expectedMarkers := []string{
		"[REDACTED_EMAIL]",
		"[REDACTED_PHONE]",
		"[REDACTED_CREDIT_CARD]",
		"[REDACTED_SSN]",
	}

	for _, marker := range expectedMarkers {
		if !strings.Contains(payloadStr, marker) {
			t.Errorf("Compliance Check Failed: Expected redaction marker %q not found in payload", marker)
		}
	}
}
