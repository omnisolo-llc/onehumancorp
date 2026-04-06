package telemetry

import (
	"reflect"
	"testing"
)

func TestRedactInterfacePII(t *testing.T) {
	input := map[string]interface{}{
		"email": "test@example.com",
		"phone": "555-123-4567",
		"ssn":   "123-45-6789",
		"nested": map[string]interface{}{
			"data": []interface{}{
				"hello test@example.com world",
				"my phone is 555.123.4567",
			},
		},
		"strings": []string{"email is test@example.com"},
		"maps": []map[string]interface{}{
			{"key": "value test@example.com"},
		},
	}

	expected := map[string]interface{}{
		"email": "[REDACTED_EMAIL]",
		"phone": "[REDACTED_PHONE]",
		"ssn":   "[REDACTED_SSN]",
		"nested": map[string]interface{}{
			"data": []interface{}{
				"hello [REDACTED_EMAIL] world",
				"my phone is [REDACTED_PHONE]",
			},
		},
		"strings": []string{"email is [REDACTED_EMAIL]"},
		"maps": []map[string]interface{}{
			{"key": "value [REDACTED_EMAIL]"},
		},
	}

	output := RedactInterfacePII(input)
	if !reflect.DeepEqual(output, expected) {
		t.Errorf("RedactInterfacePII failed.\nExpected: %+v\nGot:      %+v", expected, output)
	}
}
