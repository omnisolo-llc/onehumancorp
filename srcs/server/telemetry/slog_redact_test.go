package telemetry

import (
	"log/slog"
	"testing"
)

func TestRedactInterfacePII_Slog(t *testing.T) {
	attr := slog.String("email", "test@example.com")
	redactedAttr := RedactInterfacePII(attr).(slog.Attr)
	if redactedAttr.Value.String() != "[REDACTED_EMAIL]" {
		t.Errorf("Expected [REDACTED_EMAIL], got %v", redactedAttr.Value.String())
	}

	val := slog.StringValue("test@example.com")
	redactedVal := RedactInterfacePII(val).(slog.Value)
	if redactedVal.String() != "[REDACTED_EMAIL]" {
		t.Errorf("Expected [REDACTED_EMAIL], got %v", redactedVal.String())
	}

	groupAttr := slog.Group("user", slog.String("email", "test@example.com"))
	redactedGroupAttr := RedactInterfacePII(groupAttr).(slog.Attr)

	groupVal := redactedGroupAttr.Value.Group()
	if len(groupVal) != 1 {
		t.Fatalf("Expected 1 attribute in group, got %d", len(groupVal))
	}
	if groupVal[0].Value.String() != "[REDACTED_EMAIL]" {
		t.Errorf("Expected [REDACTED_EMAIL] in group attr, got %v", groupVal[0].Value.String())
	}

	groupVal2 := slog.GroupValue(slog.String("email", "test@example.com"))
	redactedGroupVal := RedactInterfacePII(groupVal2).(slog.Value)
	groupVal3 := redactedGroupVal.Group()
	if len(groupVal3) != 1 {
		t.Fatalf("Expected 1 attribute in group value, got %d", len(groupVal3))
	}
	if groupVal3[0].Value.String() != "[REDACTED_EMAIL]" {
		t.Errorf("Expected [REDACTED_EMAIL] in group value, got %v", groupVal3[0].Value.String())
	}

	recursiveGroupAttr := slog.Group("parent",
		slog.Group("child",
			slog.String("email", "test@example.com"),
		),
	)

	redactedRecursiveAttr := RedactInterfacePII(recursiveGroupAttr).(slog.Attr)

	parentGroup := redactedRecursiveAttr.Value.Group()
	if len(parentGroup) != 1 {
		t.Fatalf("Expected 1 attribute in parent group, got %d", len(parentGroup))
	}

	childGroup := parentGroup[0].Value.Group()
	if len(childGroup) != 1 {
		t.Fatalf("Expected 1 attribute in child group, got %d", len(childGroup))
	}

	if childGroup[0].Value.String() != "[REDACTED_EMAIL]" {
		t.Errorf("Expected [REDACTED_EMAIL] in child group, got %v", childGroup[0].Value.String())
	}
}
