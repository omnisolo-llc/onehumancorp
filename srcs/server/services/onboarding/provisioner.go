package onboarding

import (
"context"
"fmt"
"os"
"path/filepath"

"go.opentelemetry.io/otel"
"go.opentelemetry.io/otel/metric"
)

var (
provisionsCounter metric.Int64Counter
cleanupsCounter   metric.Int64Counter
)

func init() {
meter := otel.Meter("github.com/onehumancorp/mono/ohc")
provisionsCounter, _ = meter.Int64Counter("provisions_total")
cleanupsCounter, _ = meter.Int64Counter("cleanups_total")
}

// ohcDataDir returns the base data directory for OHC, preferring environment
// overrides over XDG defaults so that we never write inside the source tree.
func ohcDataDir(isCloud bool) string {
if isCloud {
if v := os.Getenv("OHC_CLOUD_DATA_DIR"); v != "" {
return v
}
} else {
if v := os.Getenv("OHC_LOCAL_DATA_DIR"); v != "" {
return v
}
}

dataHome := os.Getenv("XDG_DATA_HOME")
if dataHome == "" {
if home := os.Getenv("HOME"); home != "" {
dataHome = filepath.Join(home, ".local", "share")
} else {
dataHome = filepath.Join(os.TempDir(), "ohc-data")
}
}

suffix := "local"
if isCloud {
suffix = "cloud"
}
return filepath.Join(dataHome, "ohc", suffix)
}

// ProvisionEnvironment sets up the necessary folders for Day One Hybrid OS onboarding.
func ProvisionEnvironment(ctx context.Context, isCloud bool) error {
if provisionsCounter != nil {
provisionsCounter.Add(ctx, 1)
}

baseDir := ohcDataDir(isCloud)

dirs := []string{
filepath.Join(baseDir, "db"),
filepath.Join(baseDir, "blob"),
filepath.Join(baseDir, "config"),
}

for _, dir := range dirs {
if err := os.MkdirAll(dir, 0755); err != nil {
return fmt.Errorf("failed to create directory %s: %w", dir, err)
}
}

return nil
}

// CheckEnvironment verifies that the necessary folders for Day One Hybrid OS onboarding exist.
func CheckEnvironment(isCloud bool) error {
baseDir := ohcDataDir(isCloud)

dirs := []string{
filepath.Join(baseDir, "db"),
filepath.Join(baseDir, "blob"),
filepath.Join(baseDir, "config"),
}

for _, dir := range dirs {
if _, err := os.Stat(dir); os.IsNotExist(err) {
return fmt.Errorf("directory %s does not exist", dir)
}
}

return nil
}

// CleanupEnvironment removes the necessary folders for Day One Hybrid OS onboarding, useful for resetting state.
func CleanupEnvironment(ctx context.Context, isCloud bool) error {
if cleanupsCounter != nil {
cleanupsCounter.Add(ctx, 1)
}

return os.RemoveAll(ohcDataDir(isCloud))
}
