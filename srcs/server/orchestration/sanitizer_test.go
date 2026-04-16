package orchestration

import (
	"testing"
)

func TestSanitizePayload(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "No private markers",
			input:    "This is a normal string.",
			expected: "This is a normal string.",
		},
		{
			name:     "With private markers",
			input:    "This is [PRIVATE:secret data]a string.",
			expected: "This is a string.",
		},
		{
			name:     "With multiple private markers",
			input:    "[PRIVATE:start]Hello[PRIVATE:middle]World[PRIVATE:end]",
			expected: "HelloWorld",
		},
		{
			name:     "With PII data",
			input:    "My email is test@example.com.",
			expected: "My email is [REDACTED_EMAIL].",
		},
		{
			name:     "With both private markers and PII data",
			input:    "[PRIVATE:do not read] My email is secret@example.com and phone is 555-555-1234.",
			expected: " My email is [REDACTED_EMAIL] and phone is [REDACTED_PHONE].",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			result, err := SanitizePayload(tc.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if result != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, result)
			}
		})
	}
}

func TestSanitizePayloadMap(t *testing.T) {
	t.Run("Map", func(t *testing.T) {
		m := map[string]interface{}{
			"user":  "user@example.com",
			"other": "safe text",
		}
		res := SanitizePayloadMap(m).(map[string]interface{})
		if res["user"] != "[REDACTED_EMAIL]" {
			t.Errorf("Expected [REDACTED_EMAIL], got %v", res["user"])
		}
		if res["other"] != "safe text" {
			t.Errorf("Expected 'safe text', got %v", res["other"])
		}
		// ensure original map is not mutated
		if m["user"] != "user@example.com" {
			t.Errorf("Expected original map to be unchanged, got %v", m["user"])
		}
	})

	t.Run("Slice of interface", func(t *testing.T) {
		s := []interface{}{"user@example.com", "safe text"}
		res := SanitizePayloadMap(s).([]interface{})
		if res[0] != "[REDACTED_EMAIL]" {
			t.Errorf("Expected [REDACTED_EMAIL], got %v", res[0])
		}
		if s[0] != "user@example.com" {
			t.Errorf("Expected original slice to be unchanged, got %v", s[0])
		}
	})


	t.Run("Telemetry buffer PII deep leak check", func(t *testing.T) {
		input := map[string]interface{}{
			"agent_id": "agent-1",
			"tool_call": map[string]interface{}{
				"name": "lookup_user",
				"parameters": map[string]interface{}{
					"email": "customer@onehumancorp.com",
					"phone": "+1-555-123-4567",
					"social_security": "999-99-9999",
					"credit_card": "4532-1234-5678-9012",
					"api_key": "sk-123456789012345678901234567890123456789012345678",
				},
			},
			"raw_logs": []interface{}{
				"User customer@onehumancorp.com logged in.",
				"API Key sk-123456789012345678901234567890123456789012345678 was used.",
			},
		}

		got := SanitizePayloadMap(input).(map[string]interface{})

		tool_call := got["tool_call"].(map[string]interface{})
		params := tool_call["parameters"].(map[string]interface{})

		if params["email"] != "[REDACTED_EMAIL]" {
			t.Errorf("Email not redacted in parameters, got: %v", params["email"])
		}
		if params["social_security"] != "[REDACTED_SSN]" {
			t.Errorf("SSN not redacted in parameters, got: %v", params["social_security"])
		}
		if params["credit_card"] != "[REDACTED_CREDIT_CARD]" {
			t.Errorf("Credit card not redacted in parameters, got: %v", params["credit_card"])
		}
		if params["api_key"] != "[REDACTED_OPENAI_KEY]" {
			t.Errorf("API key not redacted in parameters, got: %v", params["api_key"])
		}

		raw_logs := got["raw_logs"].([]interface{})
		if raw_logs[0] != "User [REDACTED_EMAIL] logged in." {
			t.Errorf("Email not redacted in array log, got: %v", raw_logs[0])
		}
		if raw_logs[1] != "API Key [REDACTED_OPENAI_KEY] was used." {
			t.Errorf("Secret not redacted in array log, got: %v", raw_logs[1])
		}
	})

	t.Run("Remove rag_context and complex redaction", func(t *testing.T) {
		input := map[string]interface{}{
			"email":       "test@example.com",
			"rag_context": "highly sensitive internal data",
			"nested": map[string]interface{}{
				"secret": "[PRIVATE:password]",
				"cc":     "4111-1111-1111-1111",
			},
			"list": []interface{}{
				"sk-123456789012345678901234567890123456789012345678",
				map[string]interface{}{
					"ssn": "000-00-0000",
				},
			},
		}

		got := SanitizePayloadMap(input).(map[string]interface{})

		if _, ok := got["rag_context"]; ok {
			t.Errorf("SanitizePayloadMap should have deleted 'rag_context'")
		}

		if got["email"] != "[REDACTED_EMAIL]" {
			t.Errorf("Email not redacted: %v", got["email"])
		}

		nested := got["nested"].(map[string]interface{})
		if nested["secret"] != "" {
			t.Errorf("Private tag not removed, got: %q", nested["secret"])
		}
		if nested["cc"] != "[REDACTED_CREDIT_CARD]" {
			t.Errorf("Credit card not redacted: %v", nested["cc"])
		}

		list := got["list"].([]interface{})
		if list[0] != "[REDACTED_OPENAI_KEY]" {
			t.Errorf("OpenAI key in list not redacted: %v", list[0])
		}
		innerMap := list[1].(map[string]interface{})
		if innerMap["ssn"] != "[REDACTED_SSN]" {
			t.Errorf("SSN in nested list map not redacted: %v", innerMap["ssn"])
		}
	})
}
