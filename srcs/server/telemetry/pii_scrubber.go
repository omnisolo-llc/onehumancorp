package telemetry

import (
	"context"
	"log/slog"
)

// PIIScrubberHandler wraps an existing slog.Handler to automatically redact
// PII from log messages and string attributes, providing a compliance guardrail
// against PII leakage in multi-tenant and local environments.
type PIIScrubberHandler struct {
	slog.Handler
}

// redactAttr recursively redacts PII from an slog.Attr.
func redactAttr(a slog.Attr) slog.Attr {
	if a.Value.Kind() == slog.KindString {
		a.Value = slog.StringValue(RedactPII(a.Value.String()))
	} else if a.Value.Kind() == slog.KindAny {
		a.Value = slog.AnyValue(RedactInterfacePII(a.Value.Any()))
	} else if a.Value.Kind() == slog.KindGroup {
		attrs := a.Value.Group()
		redactedAttrs := make([]slog.Attr, len(attrs))
		for i, attr := range attrs {
			redactedAttrs[i] = redactAttr(attr)
		}
		a.Value = slog.GroupValue(redactedAttrs...)
	}
	return a
}

// Handle processes the log record, redacting PII from the message and string attributes.
func (h *PIIScrubberHandler) Handle(ctx context.Context, r slog.Record) error {
	redactedMsg := RedactPII(r.Message)

	newRecord := slog.NewRecord(r.Time, r.Level, redactedMsg, r.PC)

	r.Attrs(func(a slog.Attr) bool {
		newRecord.AddAttrs(redactAttr(a))
		return true
	})

	return h.Handler.Handle(ctx, newRecord)
}

// WithAttrs returns a new PIIScrubberHandler with the additional attributes redacted.
func (h *PIIScrubberHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
	redactedAttrs := make([]slog.Attr, 0, len(attrs))
	for _, a := range attrs {
		redactedAttrs = append(redactedAttrs, redactAttr(a))
	}
	return &PIIScrubberHandler{Handler: h.Handler.WithAttrs(redactedAttrs)}
}

// WithGroup returns a new PIIScrubberHandler with the group.
func (h *PIIScrubberHandler) WithGroup(name string) slog.Handler {
	return &PIIScrubberHandler{Handler: h.Handler.WithGroup(name)}
}
