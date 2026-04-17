package onboarding

import (
"context"
"path/filepath"
"strings"
"testing"
)

func TestGenerateAuditReport_Passed(t *testing.T) {
tmp := t.TempDir()
t.Setenv("OHC_LOCAL_DATA_DIR", filepath.Join(tmp, "local"))
t.Setenv("OHC_CLOUD_DATA_DIR", filepath.Join(tmp, "cloud"))

if err := ProvisionEnvironment(context.Background(), false); err != nil {
t.Fatalf("provision failed: %v", err)
}

report := GenerateAuditReport(false)
if !strings.Contains(report, "PASSED") {
t.Fatalf("expected report to contain PASSED, got: %s", report)
}
if !strings.Contains(report, "backdrop-filter: blur(20px) saturate(200%)") {
t.Fatalf("expected report to contain OHC Glassmorphism tokens, got: %s", report)
}
}

func TestGenerateAuditReport_Failed(t *testing.T) {
tmp := t.TempDir()
t.Setenv("OHC_LOCAL_DATA_DIR", filepath.Join(tmp, "local"))
t.Setenv("OHC_CLOUD_DATA_DIR", filepath.Join(tmp, "cloud"))
// No ProvisionEnvironment call – cloud dirs don't exist.

report := GenerateAuditReport(true)
if !strings.Contains(report, "FAILED") {
t.Fatalf("expected report to contain FAILED, got: %s", report)
}
if !strings.Contains(report, "does not exist") {
t.Fatalf("expected report to contain details about missing directory, got: %s", report)
}
}
