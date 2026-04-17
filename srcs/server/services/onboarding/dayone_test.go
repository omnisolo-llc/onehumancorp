package onboarding

import (
"context"
"path/filepath"
"strings"
"testing"
)

func TestRunDayOneSetup_Standalone(t *testing.T) {
tmp := t.TempDir()
t.Setenv("OHC_LOCAL_DATA_DIR", filepath.Join(tmp, "local"))
t.Setenv("OHC_CLOUD_DATA_DIR", filepath.Join(tmp, "cloud"))

ctx := context.Background()
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
tmp := t.TempDir()
t.Setenv("OHC_LOCAL_DATA_DIR", filepath.Join(tmp, "local"))
t.Setenv("OHC_CLOUD_DATA_DIR", filepath.Join(tmp, "cloud"))

ctx := context.Background()
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
