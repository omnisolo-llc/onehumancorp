package onboarding

import (
	"context"
	"os"
	"strings"
	"testing"
)

func TestRunDayOneSetup_Standalone(t *testing.T) {
	ctx := context.Background()
	os.RemoveAll(".ohc-local-data")
	defer os.RemoveAll(".ohc-local-data")

	report, err := RunDayOneSetup(ctx, false)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !strings.Contains(report, "PASSED") {
		t.Errorf("expected report to contain PASSED, got: %s", report)
	}
	if !strings.Contains(report, "Standalone") {
		t.Errorf("expected report to contain Standalone, got: %s", report)
	}
}

func TestRunDayOneSetup_Cloud(t *testing.T) {
	ctx := context.Background()
	os.RemoveAll(".ohc-cloud-data")
	defer os.RemoveAll(".ohc-cloud-data")

	report, err := RunDayOneSetup(ctx, true)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !strings.Contains(report, "PASSED") {
		t.Errorf("expected report to contain PASSED, got: %s", report)
	}
	if !strings.Contains(report, "Cloud-native") {
		t.Errorf("expected report to contain Cloud-native, got: %s", report)
	}
}
