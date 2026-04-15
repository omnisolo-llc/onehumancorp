1. **Create `PIIRedactingHandler`**
   - Execute the following bash block to create `srcs/server/telemetry/logging.go` with the `PIIRedactingHandler` implementation and immediately verify it:
   ```bash
   cat << 'EOC' > srcs/server/telemetry/logging.go
   package telemetry

   import (
       "context"
       "log/slog"
   )

   // PIIRedactingHandler is a slog.Handler that wraps another handler
   // and redacts PII from all string attributes before logging them.
   type PIIRedactingHandler struct {
       handler slog.Handler
   }

   // NewPIIRedactingHandler creates a new PIIRedactingHandler wrapping the given handler.
   func NewPIIRedactingHandler(h slog.Handler) *PIIRedactingHandler {
       return &PIIRedactingHandler{handler: h}
   }

   // Enabled delegates to the underlying handler.
   func (h *PIIRedactingHandler) Enabled(ctx context.Context, level slog.Level) bool {
       return h.handler.Enabled(ctx, level)
   }

   // Handle redacts PII from attributes and then delegates to the underlying handler.
   func (h *PIIRedactingHandler) Handle(ctx context.Context, r slog.Record) error {
       // Create a new record with the same basic fields
       newRecord := slog.NewRecord(r.Time, r.Level, RedactPII(r.Message), r.PC)

       // Redact attributes
       r.Attrs(func(a slog.Attr) bool {
           newRecord.AddAttrs(redactAttr(a))
           return true
       })

       return h.handler.Handle(ctx, newRecord)
   }

   // WithAttrs delegates to the underlying handler with redacted attributes.
   func (h *PIIRedactingHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
       redactedAttrs := make([]slog.Attr, len(attrs))
       for i, a := range attrs {
           redactedAttrs[i] = redactAttr(a)
       }
       return &PIIRedactingHandler{handler: h.handler.WithAttrs(redactedAttrs)}
   }

   // WithGroup delegates to the underlying handler.
   func (h *PIIRedactingHandler) WithGroup(name string) slog.Handler {
       return &PIIRedactingHandler{handler: h.handler.WithGroup(name)}
   }

   func redactAttr(a slog.Attr) slog.Attr {
       if a.Value.Kind() == slog.KindString {
           return slog.String(a.Key, RedactPII(a.Value.String()))
       }
       // If it's a group, we would ideally need to redact its contents recursively,
       // but slog.Group creates an attribute containing a slice of attributes.
       if a.Value.Kind() == slog.KindGroup {
           attrs := a.Value.Group()
           redactedAttrs := make([]any, len(attrs))
           for i, groupAttr := range attrs {
               redactedAttrs[i] = redactAttr(groupAttr)
           }
           return slog.Group(a.Key, redactedAttrs...)
       }
       return a
   }
   EOC
   cat srcs/server/telemetry/logging.go
   ```

2. **Update `srcs/server/main.go`**
   - Modify `srcs/server/main.go` to wrap the `slog.Handler` with `telemetry.NewPIIRedactingHandler` to ensure all logs are scrubbed of PII. Execute this and verify:
   ```bash
   cat << 'EOC' > update_main.py
import sys

with open("srcs/server/main.go", "r") as f:
    content = f.read()

old_str = """	var handler slog.Handler = slog.NewJSONHandler(os.Stdout, opts)
	// Provide unified logging across Cloud and Local standalone modes
	if os.Getenv("OHC_STANDALONE") == "true" {
		handler = slog.NewTextHandler(os.Stdout, opts)
	}
	logger := slog.New(handler)
	slog.SetDefault(logger)"""

new_str = """	var handler slog.Handler = slog.NewJSONHandler(os.Stdout, opts)
	// Provide unified logging across Cloud and Local standalone modes
	if os.Getenv("OHC_STANDALONE") == "true" {
		handler = slog.NewTextHandler(os.Stdout, opts)
	}

	// Wrap with PII redacting handler
	redactingHandler := telemetry.NewPIIRedactingHandler(handler)
	logger := slog.New(redactingHandler)
	slog.SetDefault(logger)"""

new_content = content.replace(old_str, new_str)
with open("srcs/server/main.go", "w") as f:
    f.write(new_content)
EOC
   python3 update_main.py
   rm update_main.py
   git diff srcs/server/main.go
   ```

3. **Add tests for `PIIRedactingHandler`**
   - Create tests in `srcs/server/telemetry/logging_test.go` and add the new file to `BUILD.bazel`. Execute and verify:
   ```bash
   cat << 'EOC' > srcs/server/telemetry/logging_test.go
   package telemetry_test

   import (
       "bytes"
       "context"
       "log/slog"
       "testing"
       "github.com/onehumancorp/mono/srcs/server/telemetry"
   )

   func TestPIIRedactingHandler(t *testing.T) {
       var buf bytes.Buffer
       baseHandler := slog.NewTextHandler(&buf, nil)
       redactingHandler := telemetry.NewPIIRedactingHandler(baseHandler)
       logger := slog.New(redactingHandler)

       ctx := context.Background()
       logger.InfoContext(ctx, "Test message contact test@example.com",
           "email", "user@example.com",
           "phone", "123-456-7890",
           "safe", "safe text",
           "group", slog.GroupValue(
               slog.String("nested_email", "admin@example.com"),
           ),
       )

       output := buf.String()

       if bytes.Contains(buf.Bytes(), []byte("test@example.com")) {
           t.Errorf("Expected message to be redacted, got %q", output)
       }
       if bytes.Contains(buf.Bytes(), []byte("user@example.com")) {
           t.Errorf("Expected email attr to be redacted, got %q", output)
       }
       if bytes.Contains(buf.Bytes(), []byte("123-456-7890")) {
           t.Errorf("Expected phone attr to be redacted, got %q", output)
       }
       if bytes.Contains(buf.Bytes(), []byte("admin@example.com")) {
           t.Errorf("Expected nested email attr to be redacted, got %q", output)
       }
       if !bytes.Contains(buf.Bytes(), []byte("safe text")) {
           t.Errorf("Expected safe text to be present, got %q", output)
       }
       if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_EMAIL]")) {
           t.Errorf("Expected [REDACTED_EMAIL] in output, got %q", output)
       }
       if !bytes.Contains(buf.Bytes(), []byte("[REDACTED_PHONE]")) {
           t.Errorf("Expected [REDACTED_PHONE] in output, got %q", output)
       }
   }
   EOC
   cat srcs/server/telemetry/logging_test.go

   # Add to BUILD.bazel
   sed -i '/"telemetry_test.go",/a\        "logging_test.go",' srcs/server/telemetry/BUILD.bazel
   git diff srcs/server/telemetry/BUILD.bazel
   ```

4. **Add `logging.go` to `BUILD.bazel`**
   - Ensure the new file `logging.go` is added to `srcs/server/telemetry/BUILD.bazel` under `go_library`.
   ```bash
   sed -i '/"telemetry.go",/a\        "logging.go",' srcs/server/telemetry/BUILD.bazel
   git diff srcs/server/telemetry/BUILD.bazel
   ```

5. **Run all tests to verify**
   - Execute the tests to ensure all functionality is correct.
   ```bash
   export PATH=$PATH:$HOME/go/bin && bazelisk test //srcs/server/...
   ```

Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit PR**
   - Submit the PR with the following arguments:
     - `branch_name`: "maintainer/telemetry-pii-redacting-handler"
     - `commit_message`: "feat(telemetry): add PIIRedactingHandler to automatically scrub slog output"
     - `title`: "✨ Maintainer: Implement PIIRedactingHandler for slog"
     - `description`: "This PR implements a `PIIRedactingHandler` that wraps existing `slog.Handler`s and automatically scrubs PII from log messages and attributes using `telemetry.RedactPII`. It updates `main.go` to use this handler globally across both Cloud and Standalone modes, ensuring compliance guardrails against PII leakage in logs."
