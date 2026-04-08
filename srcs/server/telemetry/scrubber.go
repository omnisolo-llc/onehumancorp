package telemetry

import (
	"context"
	"log/slog"
)

// PIIScrubberHandler wraps an slog.Handler to automatically redact
// Personally Identifiable Information (PII) from log messages and attributes.
type PIIScrubberHandler struct {
	handler slog.Handler
}

// NewPIIScrubberHandler creates a new PIIScrubberHandler wrapping the provided handler.
func NewPIIScrubberHandler(h slog.Handler) *PIIScrubberHandler {
	return &PIIScrubberHandler{handler: h}
}

// Enabled reports whether the handler handles records at the given level.
func (h *PIIScrubberHandler) Enabled(ctx context.Context, level slog.Level) bool {
	return h.handler.Enabled(ctx, level)
}

// Handle formats its argument Record after redacting PII from the message and attributes.
func (h *PIIScrubberHandler) Handle(ctx context.Context, r slog.Record) error {
	// Create a new record to avoid modifying the caller's copy
	newRecord := slog.NewRecord(r.Time, r.Level, RedactPII(r.Message), r.PC)

	// Iterate over the original record's attributes and redact them
	r.Attrs(func(a slog.Attr) bool {
		newRecord.AddAttrs(redactAttr(a))
		return true
	})

	return h.handler.Handle(ctx, newRecord)
}

// WithAttrs returns a new PIIScrubberHandler whose attributes consist of
// both the receiver's attributes and the arguments.
func (h *PIIScrubberHandler) WithAttrs(attrs []slog.Attr) slog.Handler {
	redactedAttrs := make([]slog.Attr, len(attrs))
	for i, a := range attrs {
		redactedAttrs[i] = redactAttr(a)
	}
	return &PIIScrubberHandler{handler: h.handler.WithAttrs(redactedAttrs)}
}

// WithGroup returns a new PIIScrubberHandler with the given group appended to
// the receiver's existing groups.
func (h *PIIScrubberHandler) WithGroup(name string) slog.Handler {
	return &PIIScrubberHandler{handler: h.handler.WithGroup(name)}
}

// redactAttr deeply redacts PII from an slog.Attr.
func redactAttr(a slog.Attr) slog.Attr {
	if a.Value.Kind() == slog.KindGroup {
		attrs := a.Value.Group()
		redactedAttrs := make([]any, len(attrs))
		for i, attr := range attrs {
			redactedAttrs[i] = redactAttr(attr)
		}
		return slog.Group(a.Key, redactedAttrs...)
	}

	val := a.Value.Any()
	redactedVal := RedactInterfacePII(val)
	return slog.Any(a.Key, redactedVal)
}
