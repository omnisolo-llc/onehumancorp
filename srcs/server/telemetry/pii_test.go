package telemetry

import (
	"encoding/json"
	"strings"
	"testing"
	"time"
)

func TestRedactInterfacePIISubstrings(t *testing.T) {
	attrs := map[string]interface{}{
		"customer_email": "test@example.com",
		"user_password":  "secret",
		"stripe_token":   "tok_123",
        "some_name":      "allowed name",
        "name":           "redacted exact",
	}
	redacted := RedactInterfacePII(attrs)

	if redacted["customer_email"] != "[REDACTED]" {
		t.Errorf("Expected customer_email to be redacted, got %v", redacted["customer_email"])
	}
	if redacted["user_password"] != "[REDACTED]" {
		t.Errorf("Expected user_password to be redacted, got %v", redacted["user_password"])
	}
	if redacted["stripe_token"] != "[REDACTED]" {
		t.Errorf("Expected stripe_token to be redacted, got %v", redacted["stripe_token"])
	}
    if redacted["some_name"] == "[REDACTED]" {
        t.Errorf("Expected some_name to not be redacted")
    }
    if redacted["name"] != "[REDACTED]" {
        t.Errorf("Expected exact name to be redacted")
    }
}

func TestRedactInterfacePIIStructAndTime(t *testing.T) {
	type TestStruct struct {
		TenantID string
		Email    string
		Normal   string
		Time     time.Time
	}

	now := time.Now()
	input := TestStruct{
		TenantID: "secret123",
		Email:    "test@example.com",
		Normal:   "hello",
		Time:     now,
	}

	attrs := map[string]interface{}{
		"struct_val": input,
	}

	redacted := RedactInterfacePII(attrs)

	b, _ := json.Marshal(redacted)
	res := string(b)

	if !strings.Contains(res, `"[REDACTED]"`) {
		t.Errorf("Expected struct fields to be redacted, got %s", res)
	}
	if strings.Contains(res, "0001-01-01T00:00:00Z") {
		t.Errorf("Expected time.Time to not be zeroed out, got %s", res)
	}
}
