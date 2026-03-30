# Developer Insights

Extracted technical debt notes and TODOs from the codebase to maintain harmony between codebase realities and manual specifications.

## `srcs/cmd/ironclaw/main.go`
- **Line 249:** `TODO: fix security` - Insecure TODO comment found by Ironclaw.

## `srcs/cmd/ironclaw/main_test.go`
- **Line 150:** `TODO: fix security issue here`
- **Line 281:** `content := "TODO: fix security\npassword = \"secret\"\n"`
- **Line 300:** `if err := os.WriteFile(filepath.Join(dir, "one.go"), []byte("TODO: fix security"), 0o600); err != nil {`
