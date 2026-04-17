package onboarding

import (
"context"
"os"
"path/filepath"
"testing"
)

// setTestDataDirs configures isolated temporary directories for both local and
// cloud OHC data so that tests never write inside the source tree.
func setTestDataDirs(t *testing.T) (localDir, cloudDir string) {
t.Helper()
tmp := t.TempDir()
localDir = filepath.Join(tmp, "local")
cloudDir = filepath.Join(tmp, "cloud")
t.Setenv("OHC_LOCAL_DATA_DIR", localDir)
t.Setenv("OHC_CLOUD_DATA_DIR", cloudDir)
return localDir, cloudDir
}

func TestProvisionEnvironment_Local(t *testing.T) {
localDir, _ := setTestDataDirs(t)

err := ProvisionEnvironment(context.Background(), false)
if err != nil {
t.Fatalf("expected nil error, got %v", err)
}

expectedDirs := []string{
filepath.Join(localDir, "db"),
filepath.Join(localDir, "blob"),
filepath.Join(localDir, "config"),
}

for _, dir := range expectedDirs {
if _, err := os.Stat(dir); os.IsNotExist(err) {
t.Errorf("expected directory %s to exist", dir)
}
}
}

func TestProvisionEnvironment_Cloud(t *testing.T) {
_, cloudDir := setTestDataDirs(t)

err := ProvisionEnvironment(context.Background(), true)
if err != nil {
t.Fatalf("expected nil error, got %v", err)
}

expectedDirs := []string{
filepath.Join(cloudDir, "db"),
filepath.Join(cloudDir, "blob"),
filepath.Join(cloudDir, "config"),
}

for _, dir := range expectedDirs {
if _, err := os.Stat(dir); os.IsNotExist(err) {
t.Errorf("expected directory %s to exist", dir)
}
}
}

func TestCheckEnvironment_Local(t *testing.T) {
setTestDataDirs(t)

err := CheckEnvironment(false)
if err == nil {
t.Fatalf("expected error for missing environment, got nil")
}

if err := ProvisionEnvironment(context.Background(), false); err != nil {
t.Fatalf("provision failed: %v", err)
}
if err := CheckEnvironment(false); err != nil {
t.Fatalf("expected nil error for provisioned environment, got %v", err)
}
}

func TestCheckEnvironment_Cloud(t *testing.T) {
setTestDataDirs(t)

err := CheckEnvironment(true)
if err == nil {
t.Fatalf("expected error for missing environment, got nil")
}

if err := ProvisionEnvironment(context.Background(), true); err != nil {
t.Fatalf("provision failed: %v", err)
}
if err := CheckEnvironment(true); err != nil {
t.Fatalf("expected nil error for provisioned environment, got %v", err)
}
}

func TestCleanupEnvironment_Local(t *testing.T) {
setTestDataDirs(t)

if err := ProvisionEnvironment(context.Background(), false); err != nil {
t.Fatalf("provision failed: %v", err)
}
err := CleanupEnvironment(context.Background(), false)
if err != nil {
t.Fatalf("expected nil error for cleanup environment, got %v", err)
}
if err := CheckEnvironment(false); err == nil {
t.Fatalf("expected error for missing environment after cleanup, got nil")
}
}

func TestCleanupEnvironment_Cloud(t *testing.T) {
setTestDataDirs(t)

if err := ProvisionEnvironment(context.Background(), true); err != nil {
t.Fatalf("provision failed: %v", err)
}
err := CleanupEnvironment(context.Background(), true)
if err != nil {
t.Fatalf("expected nil error for cleanup environment, got %v", err)
}
if err := CheckEnvironment(true); err == nil {
t.Fatalf("expected error for missing environment after cleanup, got nil")
}
}
