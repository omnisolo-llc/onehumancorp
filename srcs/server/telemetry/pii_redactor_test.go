package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	tests := []struct {
		name     string
		input    interface{}
		expected interface{}
	}{
		{
			name: "safe map",
			input: map[string]interface{}{
				"safe_field":   "safe_value",
				"another_safe": 123,
			},
			expected: map[string]interface{}{
				"safe_field":   "safe_value",
				"another_safe": 123,
			},
		},
		{
			name: "nested secrets and emails",
			input: map[string]interface{}{
				"safe_field": "safe_value",
				"nested": map[string]interface{}{
					"password":     "my_super_secret_password",
					"email":        "user@example.com",
					"another_safe": "value",
				},
				"array": []interface{}{
					map[string]interface{}{"ssn": "123-45-6789"},
					map[string]interface{}{"phone": "555-1234"},
				},
				"raw_email": "test@test.com",
				"API_KEY":   "sk-123456",
			},
			expected: map[string]interface{}{
				"safe_field": "safe_value",
				"nested": map[string]interface{}{
					"password":     "[REDACTED]",
					"email":        "[REDACTED]",
					"another_safe": "value",
				},
				"array": []interface{}{
					map[string]interface{}{"ssn": "[REDACTED]"},
					map[string]interface{}{"phone": "[REDACTED]"},
				},
				"raw_email": "[REDACTED]",
				"API_KEY":   "[REDACTED]",
			},
		},
		{
			name: "malicious payloads",
			input: map[string]interface{}{
				"payload": map[string]interface{}{
					"credit_card":     "4111-1111-1111-1111",
					"cvv":             "123",
					"dob":             "1990-01-01",
					"passport_number": "A1234567",
					"bank_account":    "123456789",
					"stripe_token":    "tok_123456789",
					"billing_address": "123 Main St, Anytown USA",
					"ssn":             "123-45-6789",
					"phone_number":    "555-123-4567",
					"email_address":   "malicious@example.com",
					"tenant_id":       "tenant-123",
					"organization_id": "org-456",
					"session_id":      "session-789",
					"ip_address":      "192.168.1.1",
					"mac_address":     "00:1B:44:11:3A:B7",
					"geolocation":     "37.7749,-122.4194",
				},
				"nested": map[string]interface{}{
					"deep": map[string]interface{}{
						"secret_key":     "sk-1234567890",
						"api_key":        "ak-0987654321",
						"auth_token":     "Bearer token",
						"password_hash":  "hash",
						"cookie_session": "cookie",
						"credential_id":  "cred-1",
					},
				},
				"array_of_evil": []interface{}{
					map[string]interface{}{"name": "John Doe", "email": "john@doe.com"},
					map[string]interface{}{"address": "456 Elm St", "phone": "555-987-6543"},
				},
				"safe_field":   "This should not be redacted",
				"another_safe": 123,
			},
			expected: map[string]interface{}{
				"payload": "[REDACTED]",
				"nested": map[string]interface{}{
					"deep": map[string]interface{}{
						"secret_key":     "[REDACTED]",
						"api_key":        "[REDACTED]",
						"auth_token":     "[REDACTED]",
						"password_hash":  "[REDACTED]",
						"cookie_session": "[REDACTED]",
						"credential_id":  "[REDACTED]",
					},
				},
				"array_of_evil": []interface{}{
					map[string]interface{}{"name": "[REDACTED]", "email": "[REDACTED]"},
					map[string]interface{}{"address": "[REDACTED]", "phone": "[REDACTED]"},
				},
				"safe_field":   "This should not be redacted",
				"another_safe": 123,
			},
		},
		{
			name: "email string values",
			input: map[string]interface{}{
				"safe_key_but_email_value": "user@example.com",
				"safe_string":              "just a regular string",
			},
			expected: map[string]interface{}{
				"safe_key_but_email_value": "[REDACTED]", // because "email" is in the key!
				"safe_string":              "just a regular string",
			},
		},
		{
			name: "nested array of strings",
			input: map[string]interface{}{
				"list": []interface{}{
					"hello",
					"test@example.com",
					"world",
				},
			},
			expected: map[string]interface{}{
				"list": []interface{}{
					"hello",
					"[EMAIL_REDACTED]",
					"world",
				},
			},
		},
		{
			name:     "string slice direct input",
			input:    []interface{}{"test@example.com", "safe"},
			expected: []interface{}{"[EMAIL_REDACTED]", "safe"},
		},
		{
			name:     "string direct input",
			input:    "test@example.com",
			expected: "[EMAIL_REDACTED]",
		},
		{
			name:     "int direct input",
			input:    123,
			expected: 123,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := RedactInterfacePII(tt.input)
			if !reflect.DeepEqual(result, tt.expected) {
				t.Errorf("RedactInterfacePII() = %v, want %v", result, tt.expected)
			}
		})
	}
}

func TestIsSensitiveKey(t *testing.T) {
	sensitiveKeys := []string{
		"password", "secret", "key", "token", "auth", "cookie",
		"credential", "email", "phone", "ssn", "address", "name",
		"pii", "tenant_id", "organization_id", "session_id", "payload",
		"credit", "card", "cvv", "dob", "birth", "passport", "bank",
		"account", "stripe", "billing", "ip_address", "mac_address", "geolocation",
		"My_Secret_Token", "USER_PASSWORD", "BillingAddress", "metric_name",
	}

	safeKeys := []string{
		"id", "timestamp", "value", "status", "count",
		"duration", "latency", "mode", "type",
	}

	for _, key := range sensitiveKeys {
		if !IsSensitiveKey(key) {
			t.Errorf("IsSensitiveKey(%q) = false, want true", key)
		}
	}

	for _, key := range safeKeys {
		if IsSensitiveKey(key) {
			t.Errorf("IsSensitiveKey(%q) = true, want false", key)
		}
	}
}

func TestIsEmail(t *testing.T) {
	emails := []string{
		"test@example.com",
		"user.name+tag@domain.co.uk",
	}

	notEmails := []string{
		"just a string",
		"test@domain",
		"admin@localhost",
		"user.name",
		"@",
		".",
	}

	for _, s := range emails {
		if !IsEmail(s) {
			t.Errorf("IsEmail(%q) = false, want true", s)
		}
	}

	for _, s := range notEmails {
		if IsEmail(s) {
			t.Errorf("IsEmail(%q) = true, want false", s)
		}
	}
}
