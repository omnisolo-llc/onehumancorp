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
